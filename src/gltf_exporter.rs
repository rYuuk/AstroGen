use gltf_json as json;

use std::{fs, mem};

use crate::asteroid_mesh::Asteroid;
use crate::ExportButtonClicked;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{Assets, Handle};
use bevy::prelude::{EventReader, Mesh, Query, Res, With};
use bevy::render::mesh::{Indices, VertexAttributeValues};
use json::validation::Checked::Valid;
use json::validation::USize64;
use std::borrow::Cow;
use std::io::Write;

pub struct GlTFExporter;

impl Plugin for GlTFExporter {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, export_gltf);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Output {
    /// Output standard glTF.
    Standard,
    /// Output binary glTF.
    Binary,
}

fn export_gltf(
    asteroid_query: Query<&Handle<Mesh>, With<Asteroid>>,
    mut on_export_clicked: EventReader<ExportButtonClicked>,
    meshes: Res<Assets<Mesh>>,
) {
    for _ in on_export_clicked.read() {
        let mesh_handle = asteroid_query.get_single().unwrap();

        if let Some(mesh) = meshes.get(&*mesh_handle) {
            let mut mesh_vertices: &Vec<[f32; 3]> = &vec![];
            let mut mesh_normals: &Vec<[f32; 3]> = &vec![];
            let mut mesh_indices: &Vec<u32> = &vec![];

            if let Some(VertexAttributeValues::Float32x3(vertices)) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            {
                mesh_vertices = vertices;
            } else {
                println!("Vertices not found or not in Float32x3 format.");
            }

            if let Some(indices) = mesh.indices() {
                match indices {
                    Indices::U16(_) => {
                        println!("WARNING !! Indices are in u16");
                    }
                    Indices::U32(indices) => {
                        mesh_indices = indices;
                    }
                }
            } else {
                println!("Mesh has no indices.");
            }

            if let Some(VertexAttributeValues::Float32x3(normals)) =
                mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            {
                mesh_normals = normals;
            } else {
                println!("Vertex normals not found or not in Float32x3 format.");
            }

            export(mesh_vertices, mesh_indices, mesh_normals);
        } else {
            println!("Mesh not found.");
        }
    }
}

fn export(vertices: &Vec<[f32; 3]>, indices: &Vec<u32>, normals: &Vec<[f32; 3]>) {
    let output: Output = Output::Binary;
    let colors: Vec<[f32; 3]> = vec![[1., 1., 1.]; vertices.len()];

    let (min, max) = bounding_coords(&vertices);

    let mut root = gltf_json::Root::default();

    let positions_length = vertices.len() * mem::size_of::<[f32; 3]>();
    let colors_length = colors.len() * mem::size_of::<[f32; 3]>();
    let normals_length = normals.len() * mem::size_of::<[f32; 3]>();
    let indices_length = indices.len() * mem::size_of::<u32>();
    let buffer_length = positions_length + colors_length + normals_length + indices_length;

    let buffer = root.push(json::Buffer {
        byte_length: USize64::from(buffer_length),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: if output == Output::Standard {
            Some("buffer0.bin".into())
        } else {
            None
        },
    });

    // Buffer view for positions
    let position_buffer_view = root.push(json::buffer::View {
        buffer,
        byte_length: USize64::from(positions_length),
        byte_offset: None,
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    });

    // Buffer view for colors
    let color_buffer_view = root.push(json::buffer::View {
        buffer,
        byte_length: USize64::from(colors_length),
        byte_offset: Some(USize64::from(positions_length)),
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    });

    let normal_buffer_view = root.push(json::buffer::View {
        buffer,
        byte_length: USize64::from(normals_length),
        byte_offset: Some(USize64::from(positions_length + colors_length)),
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    });

    // Buffer view for indices
    let index_buffer_view = root.push(json::buffer::View {
        buffer,
        byte_length: USize64::from(indices_length),
        byte_offset: Some(USize64::from(
            positions_length + colors_length + normals_length,
        )),
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ElementArrayBuffer)),
    });

    let positions_accessor = root.push(json::Accessor {
        buffer_view: Some(position_buffer_view),
        byte_offset: None,
        count: USize64::from(vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(Vec::from(min))),
        max: Some(json::Value::from(Vec::from(max))),
        name: None,
        normalized: false,
        sparse: None,
    });

    let colors_accessor = root.push(json::Accessor {
        buffer_view: Some(color_buffer_view),
        byte_offset: None,
        count: USize64::from(colors.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let normals_accessor = root.push(json::Accessor {
        buffer_view: Some(normal_buffer_view),
        byte_offset: None,
        count: USize64::from(normals.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let indices_accessor = root.push(json::Accessor {
        buffer_view: Some(index_buffer_view),
        byte_offset: None,
        count: USize64::from(indices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::U32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::BTreeMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), positions_accessor);
            map.insert(Valid(json::mesh::Semantic::Colors(0)), colors_accessor);
            map.insert(Valid(json::mesh::Semantic::Normals), normals_accessor);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: Some(indices_accessor),
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None,
    };

    let mesh = root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: vec![primitive],
        weights: None,
    });

    let node = root.push(json::Node {
        mesh: Some(mesh),
        ..Default::default()
    });

    root.push(json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        nodes: vec![node],
    });

    match output {
        Output::Standard => {
            let _ = fs::create_dir("asteroid");

            let writer = fs::File::create("asteroid/triangle.gltf").expect("I/O error");
            json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

            let bin = {
                let mut data = to_padded_byte_vector(&vertices);
                data.extend_from_slice(&to_padded_byte_vector(&colors));
                data.extend_from_slice(&to_padded_byte_vector(&normals));
                data.extend_from_slice(&to_padded_byte_vector(&indices));
                data
            };
            let mut writer = fs::File::create("asteroid/buffer0.bin").expect("I/O error");
            writer.write_all(&bin).expect("I/O error");
            println!("Asteroid data written asteroid.glb");
        }
        Output::Binary => {
            let json_string = json::serialize::to_string(&root).expect("Serialization error");
            let mut json_offset = json_string.len();
            align_to_multiple_of_four(&mut json_offset);
            let glb = gltf::binary::Glb {
                header: gltf::binary::Header {
                    magic: *b"glTF",
                    version: 2,
                    length: (json_offset + buffer_length)
                        .try_into()
                        .expect("file size exceeds binary glTF limit"),
                },
                bin: Some(Cow::Owned({
                    let mut data = to_padded_byte_vector(&vertices);
                    data.extend_from_slice(&to_padded_byte_vector(&colors));
                    data.extend_from_slice(&to_padded_byte_vector(&normals));
                    data.extend_from_slice(&to_padded_byte_vector(&indices));
                    data
                })),
                json: Cow::Owned(json_string.into_bytes()),
            };
            let writer = std::fs::File::create("asteroid.glb").expect("I/O error");
            glb.to_writer(writer).expect("glTF binary output error");
            println!("Asteroid data written asteroid.glb");
        }
    }
}

/// Calculate bounding coordinates of a list of vertices, used for the clipping distance of the model
fn bounding_coords(points: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for &p in points {
        for i in 0..3 {
            min[i] = f32::min(min[i], p[i]);
            max[i] = f32::max(max[i], p[i]);
        }
    }
    (min, max)
}

fn align_to_multiple_of_four(n: &mut usize) {
    *n = (*n + 3) & !3;
}

fn to_padded_byte_vector<T>(vec: &[T]) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let mut new_vec =
        unsafe { std::slice::from_raw_parts(vec.as_ptr() as *const u8, byte_length).to_vec() };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}
