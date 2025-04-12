#[cfg(test)]
mod tests;

use std::collections::HashMap;
use glam::Vec3;
use itertools::Itertools;
use crate::interpolation::slerp::slerp_3;
use crate::projection::packed_index::PackedIndex;
use crate::projection::seed::Seed;
use crate::subdivision::subdivided_triangle::SubdividedTriangle;

#[derive(Debug)]
pub enum ExactFace {
    Pentagon([PackedIndex; 5]),
    Hexagon([PackedIndex; 6])
}

pub struct ExactGlobe<const N: u32> {
    seed: Seed<N>,
    subdivision: SubdividedTriangle<N>,
    pub faces: Vec<ExactFace>,
}

impl<const N: u32> ExactGlobe<N> {
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
        let t_t = template.wu().into_iter()
            .zip(template.uv().into_iter().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((0..5).map(|face| (face, (face + 1) % 5)));
        
        let t_um = template.vw().into_iter()
            .zip(template.vw().into_iter().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((0..5).map(|face| (face, face + 5)));
        
        let um_lm = template.uv().into_iter()
            .zip(template.uv().into_iter().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((5..10).map(|face| (face, face + 5)));
        
        let lm_um = template.wu().into_iter()
            .zip(template.wu().into_iter().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((10..15).map(|face| (face, 5 + (face + 1) % 5)));
        
        let lm_b = template.vw().into_iter()
            .zip(template.vw().into_iter().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((10..15).map(|face| (face, face + 5)));
        
        let b_b = template.uv().into_iter()
            .zip(template.wu().into_iter().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((15..20).map(|face| (face, 15 + (face + 1) % 5)));
        
        t_t
            .chain(t_um)
            .chain(um_lm)
            .chain(lm_um)
            .chain(lm_b)
            .chain(b_b)
            .map(|(((a0, b0), (a1, b1), (a2, b2)), (face_a, face_b))| ExactFace::Hexagon([
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
        (0..N as usize)
            .map(|i| template.row(i).collect::<Vec<_>>())
            .tuple_windows::<(_, _)>()
            .flat_map(|(r1, r2)|
                r1[1..(r1.len() - 1)].iter()
                    .cloned()
                    .zip(r2)
                    .tuple_windows::<(_, _, _)>()
                    .step_by(2)
                    .cartesian_product(0..20)
                    .map(|(((a0, b0), (a1, b1), (a2, b2)), face)| {
                        ExactFace::Hexagon([
                            PackedIndex::new(face, b0),
                            PackedIndex::new(face, b1),
                            PackedIndex::new(face, b2),
                            PackedIndex::new(face, a2),
                            PackedIndex::new(face, a1),
                            PackedIndex::new(face, a0),
                        ])
                    })
                    .collect::<Vec<_>>()
            )
    }
    
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
    
    pub fn mesh_triangles(&self) -> Vec<u32> {
        let mut n = 0;
        
        self.faces.iter()
            .map(|f|
                match f {
                    ExactFace::Pentagon(_) => {
                        n += 5;
                        (n - 5, f)
                    },
                    ExactFace::Hexagon(_) => {
                        n += 6;
                        (n - 6, f)
                    }
                }
            )
            .flat_map(|(i, f)|
                match f {
                    ExactFace::Pentagon(_) => &[
                        0, 1, 2,
                        0, 2, 3,
                        0, 3, 4
                    ][..],
                    ExactFace::Hexagon(_) => &[
                        0, 1, 2,
                        0, 2, 3,
                        0, 3, 4,
                        0, 4, 5
                    ][..]
                }.iter()
                    .map(move |j| j + i)
            )
            .collect::<Vec<_>>()
    }
}
