use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

#[derive(Resource)]
pub struct SphereMesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub normals: Vec<Vec3>,
}

impl Into<Mesh> for SphereMesh {
    fn into(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_indices(Indices::U32(self.indices));
        mesh
    }
}

impl SphereMesh {
    pub fn new(resolution: usize) -> Self {
        let resolution = resolution.max(0);
        let num_divisions = resolution;
        let num_verts_per_face = ((num_divisions + 3) * (num_divisions + 3) - (num_divisions + 3)) / 2;
        let num_verts = num_verts_per_face * 8 - (num_divisions + 2) * 12 + 6;
        let num_tris_per_face = (num_divisions + 1) * (num_divisions + 1);

        let mut vertices: Vec<Vec3> = Vec::with_capacity(num_verts);
        let mut indices: Vec<u32> = Vec::with_capacity(num_tris_per_face * 8 * 3);

        // The six initial vertices
        let base_vertices = [
            Vec3::Y,   // up
            -Vec3::X,  // left
            -Vec3::Z,  // back
            Vec3::X,   // right
            Vec3::Z,   // forward
            -Vec3::Y,  // down
        ];

        vertices.extend_from_slice(&base_vertices);

        // Indices of the vertex pairs that make up each of the initial 12 edges
        let vertex_pairs = [
            0, 1, 0, 2, 0, 3, 0, 4,
            1, 2, 2, 3, 3, 4, 4, 1,
            5, 1, 5, 2, 5, 3, 5, 4,
        ];

        // Indices of the edge triplets that make up the initial 8 faces
        let edge_triplets = [
            0, 1, 4, 1, 2, 5, 2, 3, 6, 3, 0, 7,
            8, 9, 4, 9, 10, 5, 10, 11, 6, 11, 8, 7,
        ];

        // Create 12 edges, with n vertices added along them (n = num_divisions)
        let mut edges = vec![vec![0; num_divisions + 2]; 12];
        for i in (0..vertex_pairs.len()).step_by(2) {
            let start_vertex = vertices[vertex_pairs[i]];
            let end_vertex = vertices[vertex_pairs[i + 1]];

            let mut edge_vertex_indices = vec![0; num_divisions + 2];
            edge_vertex_indices[0] = vertex_pairs[i] as u32;

            // Add vertices along edge
            for division_index in 0..num_divisions {
                let t = (division_index as f32 + 1.0) / (num_divisions as f32 + 1.0);
                edge_vertex_indices[division_index + 1] = vertices.len() as u32;
                // Use Dir3::slerp here
                let start_dir = Dir3::new(start_vertex);
                let end_dir = Dir3::new(end_vertex);
                let interpolated = start_dir.unwrap().slerp(end_dir.unwrap(), t);
                vertices.push(interpolated.as_vec3());
            }
            edge_vertex_indices[num_divisions + 1] = vertex_pairs[i + 1] as u32;
            let edge_index = i / 2;
            edges[edge_index] = edge_vertex_indices;
        }

        // Create faces
        for i in (0..edge_triplets.len()).step_by(3) {
            let face_index = i / 3;
            let reverse = face_index >= 4;
            Self::create_face(
                &mut vertices,
                &mut indices,
                &edges[edge_triplets[i]],
                &edges[edge_triplets[i + 1]],
                &edges[edge_triplets[i + 2]],
                num_divisions,
                reverse,
            );
        }

        let mut normals = vec![Vec3::ZERO; vertices.len()];

        for triangle in indices.chunks(3) {
            let i0 = triangle[0] as usize;
            let i1 = triangle[1] as usize;
            let i2 = triangle[2] as usize;

            let v0 = vertices[i0];
            let v1 = vertices[i1];
            let v2 = vertices[i2];

            let normal = (v1 - v0).cross(v2 - v0).normalize();

            // Assign the same normal to each vertex in the triangle
            normals[i0] = normal;
            normals[i1] = normal;
            normals[i2] = normal;
        }

        // Optionally, make sure sharp edges are preserved by not normalizing across entire mesh:
        for i in 0..normals.len() {
            if normals[i].length() > 0.001 {
                normals[i] = normals[i].normalize();
            } else {
                // Handle cases where the normal might be too small
                normals[i] = Vec3::new(0.0, 1.0, 0.0); // Default direction, adjust as needed
            }
        }

        SphereMesh {
            vertices,
            indices,
            normals,
        }
    }

    fn create_face(
        vertices: &mut Vec<Vec3>,
        indices: &mut Vec<u32>,
        side_a: &[u32],
        side_b: &[u32],
        bottom: &[u32],
        num_divisions: usize,
        reverse: bool,
    ) {
        let num_points_in_edge = side_a.len();
        let mut vertex_map: Vec<u32> = Vec::with_capacity(num_points_in_edge * num_points_in_edge);
        vertex_map.push(side_a[0]); // top of triangle

        for i in 1..num_points_in_edge - 1 {
            // Side A vertex
            vertex_map.push(side_a[i]);

            // Add vertices between sideA and sideB
            let side_a_vertex = vertices[side_a[i] as usize];
            let side_b_vertex = vertices[side_b[i] as usize];
            let num_inner_points = i - 1;
            for j in 0..num_inner_points {
                let t = (j as f32 + 1.0) / (num_inner_points as f32 + 1.0);
                vertex_map.push(vertices.len() as u32);

                let start_dir = Dir3::new(side_a_vertex);
                let end_dir = Dir3::new(side_b_vertex);
                let interpolated = start_dir.unwrap().slerp(end_dir.unwrap(), t);
                vertices.push(interpolated.as_vec3());
            }

            // Side B vertex
            vertex_map.push(side_b[i]);
        }

        // Add bottom edge vertices
        for i in 0..num_points_in_edge {
            vertex_map.push(bottom[i]);
        }

        // Triangulate
        let num_rows = num_divisions + 1;
        for row in 0..num_rows {
            let mut top_vertex = ((row + 1) * (row + 1) - row - 1) / 2;
            let mut bottom_vertex = ((row + 2) * (row + 2) - row - 2) / 2;

            let num_triangles_in_row = 1 + 2 * row;
            for column in 0..num_triangles_in_row {
                let (v0, v1, v2) = if column % 2 == 0 {
                    // Increment before using for next iteration
                    let result = (top_vertex, bottom_vertex + 1, bottom_vertex);
                    top_vertex += 1;
                    bottom_vertex += 1;
                    result
                } else {
                    (top_vertex, bottom_vertex, top_vertex - 1)
                };

                indices.push(vertex_map[v0] as u32);
                indices.push(vertex_map[if reverse { v2 } else { v1 }] as u32);
                indices.push(vertex_map[if reverse { v1 } else { v2 }] as u32);
            }
        }
    }
}
