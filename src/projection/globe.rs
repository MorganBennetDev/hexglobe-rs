#[cfg(test)]
mod tests;

use std::collections::HashMap;
use glam::Vec3;
use itertools::Itertools;
use crate::interpolation::slerp::slerp_3;
use crate::projection::packed_index::PackedIndex;
use crate::projection::seed::Seed;
use crate::subdivision::subdivided_triangle::SubdividedTriangle;

const fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Copy, Clone, Debug)]
enum ExactFace {
    Pentagon([PackedIndex; 5]),
    Hexagon([PackedIndex; 6])
}

/// Represents a face of a Goldberg polyhedron as a list of (indices to) vertices in counterclockwise winding order.
#[derive(Copy, Clone, Debug)]
pub enum MeshFace {
    Pentagon([u32; 5]),
    Hexagon([u32; 6])
}

/// Contains functionality to create a Goldberg polyhedron from an icosahedron whose faces have been subdivided `N`
/// times.
pub struct ExactGlobe<const N: u32> {
    seed: Seed<N>,
    subdivision: SubdividedTriangle<N>,
    faces: Vec<ExactFace>,
}

impl<const N: u32> ExactGlobe<N> {
    const SEED_N_VERTICES: usize = 12;
    const SEED_N_EDGES: usize = 30;
    const SEED_N_FACES: usize = 20;
    const N_FACES_PER_VERTEX: usize = 1;
    const N_FACES_PER_EDGE: usize = N as usize - 1;
    const N_FACES_PER_FACE: usize = ((N - 1) * (max(N, 2) - 2) / 2) as usize;
    /// Initializes the data for a new polyhedron. This is very cheap as all the expensive computations are done during
    /// conversion to floating point coordinates.
    pub fn new() -> Self {
        let subdivision = SubdividedTriangle::<N>::new();
        let seed = Seed::<N>::icosahedron();
        
        let faces = Self::faces_from_template(&subdivision)
            .collect::<Vec<_>>();
        
        Self {
            seed,
            subdivision,
            faces
        }
    }
    
    // Compute hexagonal faces lying along icosahedron edges and in faces and pentagonal faces lying on vertices
    /*
    The face indices to treat as the top, upper middle, lower middle, and bottom faces of the icosahedron.
    t:   0..5
    um:  5..10
    lm: 10..15
    b:  15..20
    t-t:   wu-vu
    t-um:  vw-wv
    um-lm: uv-vu
    lm-um: wu-uw
    lm-b:  vw-wv
    b-b:   uv-uw
    */
    fn faces_from_template(template: &SubdividedTriangle<N>) -> impl Iterator<Item = ExactFace> {
        Self::vertex_faces_from_template(&template)
            .chain(
                Self::edge_faces_from_template(&template)
            )
            .chain(
                Self::face_faces_from_template(&template)
            )
    }
    
    fn edge_faces_from_template(template: &SubdividedTriangle<N>) -> impl Iterator<Item = ExactFace> {
        let vw_wv = template.vw().into_iter()
            .zip(template.vw().into_iter().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2);
        
        let t_t = (0..5).map(|face| (face, (face + 1) % 5))
            .cartesian_product(
                template.wu().into_iter()
                    .zip(template.uv().into_iter().rev())
                    .tuple_windows::<(_, _, _)>()
                    .step_by(2)
            );
        
        let t_um = (0..5).map(|face| (face, face + 5))
            .cartesian_product(vw_wv.clone());
        
        let um_lm = (5..10).map(|face| (face, face + 5))
            .cartesian_product(
                template.uv().into_iter()
                    .zip(template.uv().into_iter().rev())
                    .tuple_windows::<(_, _, _)>()
                    .step_by(2)
            );
        
        let lm_um = (10..15).map(|face| (face, 5 + (face + 1) % 5))
            .cartesian_product(
                template.wu().into_iter()
                    .zip(template.wu().into_iter().rev())
                    .tuple_windows::<(_, _, _)>()
                    .step_by(2)
            );
        
        let lm_b = (10..15).map(|face| (face, face + 5))
            .cartesian_product(vw_wv);
        
        let b_b = (15..20).map(|face| (face, 15 + (face + 1) % 5))
            .cartesian_product(
                template.uv().into_iter()
                    .zip(template.wu().into_iter().rev())
                    .tuple_windows::<(_, _, _)>()
                    .step_by(2)
            );
        
        t_t
            .chain(t_um)
            .chain(um_lm)
            .chain(lm_um)
            .chain(lm_b)
            .chain(b_b)
            .map(|((face_a, face_b), ((a0, b0), (a1, b1), (a2, b2)))| ExactFace::Hexagon([
                PackedIndex::new(face_a, a0),
                PackedIndex::new(face_a, a1),
                PackedIndex::new(face_a, a2),
                PackedIndex::new(face_b, b2),
                PackedIndex::new(face_b, b1),
                PackedIndex::new(face_b, b0),
            ]))
    }
    
    fn vertex_faces_from_template(template: &SubdividedTriangle<N>) -> impl Iterator<Item = ExactFace> {
        let tb = [
            ExactFace::Pentagon([
                PackedIndex::new(4, template.u()),
                PackedIndex::new(3, template.u()),
                PackedIndex::new(2, template.u()),
                PackedIndex::new(1, template.u()),
                PackedIndex::new(0, template.u()),
            ]),
            ExactFace::Pentagon([
                PackedIndex::new(15, template.u()),
                PackedIndex::new(16, template.u()),
                PackedIndex::new(17, template.u()),
                PackedIndex::new(18, template.u()),
                PackedIndex::new(19, template.u()),
            ])
        ].into_iter();
        
        let um = (5..10)
            .map(|face| ExactFace::Pentagon([
                PackedIndex::new(face - 5, template.w()),
                PackedIndex::new((face + 1) % 5, template.v()),
                PackedIndex::new(5 + (face + 1) % 5, template.w()),
                PackedIndex::new(face + 5, template.u()),
                PackedIndex::new(face, template.v()),
            ]));
        
        let lm = (10..15)
            .map(|face| ExactFace::Pentagon([
                PackedIndex::new(face + 5, template.w()),
                PackedIndex::new(15 + (face + 4) % 5, template.v()),
                PackedIndex::new(10 + (face + 4) % 5, template.w()),
                PackedIndex::new(face - 5, template.u()),
                PackedIndex::new(face, template.v()),
            ]));
        
        tb
            .chain(um)
            .chain(lm)
    }
    
    fn face_faces_from_template(template: &SubdividedTriangle<N>) -> impl Iterator<Item = ExactFace> {
        let face_vertices_iter = (0..N as usize)
            .map(|i| template.row(i).collect::<Vec<_>>())
            .tuple_windows::<(_, _)>()
            .flat_map(|(r1, r2)|
                r1[1..(r1.len() - 1)].iter()
                    .cloned()
                    .zip(r2)
                    .tuple_windows::<(_, _, _)>()
                    .step_by(2)
                    .collect::<Vec<_>>()
            );
        
        // Ensures that faces subdivided from the same seed face are near each other.
        (0..20).into_iter()
            .cartesian_product(face_vertices_iter)
            .map(|(face, ((a0, b0), (a1, b1), (a2, b2)))| {
                ExactFace::Hexagon([
                    PackedIndex::new(face, b0),
                    PackedIndex::new(face, b1),
                    PackedIndex::new(face, b2),
                    PackedIndex::new(face, a2),
                    PackedIndex::new(face, a1),
                    PackedIndex::new(face, a0),
                ])
            })
    }
    
    fn vertex_index_to_face_index(&self, f: usize, i: usize) -> usize {
        let v = self.subdivision.vertex_denominator(i);
        
        match (v.x, v.y, v.z) {
            // Vertices
            (_, 0, 0) => match f { // u
                0..5 => 0,
                5..10 => 7 + f % 5,
                10..15 => 2 + f % 5,
                15..20 => 1,
                _ => unreachable!()
            },
            (0, _, 0) => match f { // v
                0..5 => 2 + (f + 4) % 5,
                5..10 => 2 + f % 5,
                10..15 => 7 + f % 5,
                15..20 => 7 + (f + 1) % 5,
                _ => unreachable!()
            },
            (0, 0, _) => match f { // w
                0..5 => 2 + f,
                5..10 => 2 + (f + 4) % 5,
                10..15 => 7 + (f + 1) % 5,
                15..20 => 7 + f % 5,
                _ => unreachable!()
            },
            // Edges
            (_, _, 0) | (0, _, _) | (_, 0, _) => {
                let offset = Self::SEED_N_VERTICES * Self::N_FACES_PER_VERTEX - 1;
                
                match (v.x, v.y, v.z) {
                    (_, _, 0) => match f { // uv
                        0..5 => offset + ((f + 4) % 5) * Self::N_FACES_PER_EDGE + v.x as usize,
                        5..10 => offset + (10 + f % 5) * Self::N_FACES_PER_EDGE + v.y as usize,
                        10..15 => offset + (10 + f % 5) * Self::N_FACES_PER_EDGE + v.x as usize,
                        15..20 => offset + (25 + f % 5) * Self::N_FACES_PER_EDGE + v.y as usize,
                        _ => unreachable!()
                    },
                    (0, _, _) => match f { // vw
                        0..5 => offset + (5 + f) * Self::N_FACES_PER_EDGE + v.z as usize,
                        5..10 => offset + (5 + f % 5) * Self::N_FACES_PER_EDGE + v.y as usize,
                        10..15 => offset + (20 + f % 5) * Self::N_FACES_PER_EDGE + v.z as usize,
                        15..20 => offset + (20 + f % 5) * Self::N_FACES_PER_EDGE + v.y as usize,
                        _ => unreachable!()
                    },
                    // This is just (_, 0, _), but the interpreter doesn't know that other cases aren't possible.
                    _ => match f { // wu
                        0..5 => offset + f * Self::N_FACES_PER_EDGE + v.x as usize,
                        5..10 => offset + (15 + (f + 4) % 5) * Self::N_FACES_PER_EDGE + v.z as usize,
                        10..15 => offset + (15 + f % 5) * Self::N_FACES_PER_EDGE + v.x as usize,
                        15..20 => offset + (25 + (f + 4) % 5) * Self::N_FACES_PER_EDGE + v.z as usize,
                        _ => unreachable!()
                    }
                }
            },
            // Faces
            _ => {
                let offset = Self::SEED_N_VERTICES * Self::N_FACES_PER_VERTEX +
                    Self::SEED_N_EDGES * Self::N_FACES_PER_EDGE +
                    f * Self::N_FACES_PER_FACE;
                
                // Index of vertex i in the set of vertices excluding edges.
                let j = self.subdivision.vertex_interior_index_unchecked(v);
                
                offset + j
            }
        }
    }
    
    /// [Vec] of undirected edges between adjacent faces.
    pub fn adjacency(&self) -> Vec<(usize, usize)> {
        self.subdivision.vertex_adjacency()
            .cartesian_product(0..20)
            .map(|((a, b), f)| (
                self.vertex_index_to_face_index(f, a),
                self.vertex_index_to_face_index(f, b)
            ))
            .map(|(a, b)| (a.min(b), a.max(b)))
            .unique()
            .collect::<Vec<_>>()
    }
    
    /// Returns the number of faces in the specified polyhedron.
    pub fn count_faces(&self) -> usize {
        self.faces.len()
    }
    
    /// Generates a Goldberg polyhedron with an optional radius (default of 1.0). This is the most expensive operation
    /// as it utilizes the `slerp_3` function. Optimizations have been made to exploit some of the symmetries between
    /// faces and only compute vertices for 5 of the 20 subdivided faces. Further optimizations can be made to only
    /// compute one and may be implemented in the future.
    pub fn vertices_f32(&self, r: Option<f32>) -> HashMap<PackedIndex, Vec3> {
        let radius = r.unwrap_or(1.0);
        
        let base = self.subdivision.triangles()
            .map(|t| (t.u + t.v + t.w).as_vec3() / (3 * N) as f32)
            .enumerate()
            .cartesian_product(self.seed.base_faces())
            .map(|((i, centroid), (f, face))| (
                PackedIndex::new(f, i),
                slerp_3(
                    centroid.x, face.u,
                    centroid.y, face.v,
                    centroid.z, face.w
                ) * radius
            ))
            .collect::<Vec<_>>();
            
        base.iter()
            .cloned()
            .chain(
                base.iter()
                    .cartesian_product(self.seed.symmetries())
                    .filter(|((i, _), (_, b, _))| i.face() == *b)
                    .map(|((i, v), (f, _, t))| (
                        PackedIndex::new(f, i.subdivision()),
                        t.mul_vec3(*v)
                    ))
            )
            .collect::<HashMap<_, _>>()
    }
    
    /// Generates the vertex buffer for a mesh of the given Goldberg polyhedron with radius `r` (default 1.0) using
    /// [vertices_f32].
    pub fn mesh_vertices(&self, r: Option<f32>) -> Vec<[f32; 3]> {
        let vertices = self.vertices_f32(r);
        
        self.faces.iter()
            .flat_map(|f|
                match f {
                    ExactFace::Pentagon(v) => &v[..],
                    ExactFace::Hexagon(v) => &v[..]
                }.iter()
                    .map(|i| vertices.get(&i).unwrap().to_array())
            )
            .collect()
    }
    
    /// Returns a list of the faces of the given Goldberg polyhedron used as a preliminary step in [mesh_triangles] but
    /// can also be used independently.
    pub fn mesh_faces(&self) -> Vec<MeshFace> {
        let mut n = 0;
        
        self.faces.iter()
            .map(|face| match face {
                ExactFace::Pentagon(_) => {
                    n += 5;
                    (n - 5, face)
                },
                ExactFace::Hexagon(_) => {
                    n += 6;
                    (n - 6, face)
                }
            })
            .map(|(i, face)| match face {
                ExactFace::Pentagon(_) => MeshFace::Pentagon([i, i + 1, i + 2, i + 3, i + 4]),
                ExactFace::Hexagon(_) => MeshFace::Hexagon([i, i + 1, i + 2, i + 3, i + 4, i + 5])
            })
            .collect()
    }
    
    /// Generates the triangle buffer for a mesh of the given Goldberg polyhedron with radius `r` (default 1.0). Vertex
    /// indices are deterministic so this is a cheap function and can be called independently of vertex computation.
    pub fn mesh_triangles(&self) -> Vec<u32> {
        self.mesh_faces().into_iter()
            .flat_map(|face| match face {
                MeshFace::Pentagon(v) => vec![
                    v[0], v[1], v[2],
                    v[0], v[2], v[3],
                    v[0], v[3], v[4]
                ],
                MeshFace::Hexagon(v) => vec![
                    v[0], v[1], v[2],
                    v[0], v[2], v[3],
                    v[0], v[3], v[4],
                    v[0], v[4], v[5]
                ]
            })
            .collect()
    }
}
