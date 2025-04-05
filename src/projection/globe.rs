#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::ops::Deref;
use glam::IVec3;
use itertools::Itertools;
use crate::denominator::ImplicitDenominator;
use crate::projection::packed_index::PackedIndex;
use crate::subdivision::subdivided_triangle::SubdividedTriangle;

#[derive(Debug)]
pub enum ExactFace {
    Pentagon([PackedIndex; 5]),
    Hexagon([PackedIndex; 6])
}

pub struct Globe<const N: u32> where
    [(); (3 * N) as usize] : Sized {
    pub vertices: HashMap<PackedIndex, ImplicitDenominator<IVec3, {3 * N}>>,
    pub faces: Vec<ExactFace>,
}

impl<const N: u32> Globe<N> where
    [(); (3 * N) as usize] : Sized {
    pub fn new() -> Self {
        let template = SubdividedTriangle::<N>::new();
        
        let vertices = template.triangles.iter()
            .map(|t| ImplicitDenominator::<_, {3 * N}>::wrap(
                t.u.deref().deref() + t.v.deref().deref() + t.w.deref().deref()
            ))
            .enumerate()
            .cartesian_product(0..20)
            .map(|((i, t), face)| (PackedIndex::new(face, i), t))
            .collect::<HashMap<_, _>>();
        
        let faces = Self::faces_from_template(&template)
            .collect::<Vec<_>>();
        
        Self {
            vertices,
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
        let t_t = template.wu()
            .zip(template.uv().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((0..5).map(|face| (face, (face + 1) % 5)));
        
        let t_um = template.vw()
            .zip(template.vw().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((0..5).map(|face| (face, face + 5)));
        
        let um_lm = template.uv()
            .zip(template.uv().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((5..10).map(|face| (face, face + 5)));
        
        let lm_um = template.wu()
            .zip(template.wu().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((10..15).map(|face| (face, 5 + face % 5)));
        
        let lm_b = template.vw()
            .zip(template.vw().rev())
            .tuple_windows::<(_, _, _)>()
            .step_by(2)
            .cartesian_product((10..15).map(|face| (face, face + 5)));
        
        let b_b = template.uv()
            .zip(template.wu().rev())
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
                PackedIndex::new(face_b + 5, b2),
                PackedIndex::new(face_b + 5, b1),
                PackedIndex::new(face_b + 5, b0),
            ]))
    }
    
    fn vertex_faces_from_template(template: &SubdividedTriangle<N>) -> impl Iterator<Item = ExactFace> {
        let tb = [
            ExactFace::Pentagon([
                PackedIndex::new(0, template.u()),
                PackedIndex::new(1, template.u()),
                PackedIndex::new(2, template.u()),
                PackedIndex::new(3, template.u()),
                PackedIndex::new(4, template.u()),
            ]),
            ExactFace::Pentagon([
                PackedIndex::new(19, template.u()),
                PackedIndex::new(18, template.u()),
                PackedIndex::new(17, template.u()),
                PackedIndex::new(16, template.u()),
                PackedIndex::new(15, template.u()),
            ])
        ].into_iter();
        
        let um = (5..10)
            .map(|face| ExactFace::Pentagon([
                PackedIndex::new(face, template.v()),
                PackedIndex::new(face + 5, template.u()),
                PackedIndex::new(5 + (face + 1) % 5, template.w()),
                PackedIndex::new((face + 1) % 5, template.v()),
                PackedIndex::new(face - 5, template.w()),
            ]));
        
        let lm = (10..15)
            .map(|face| ExactFace::Pentagon([
                PackedIndex::new(face, template.v()),
                PackedIndex::new(face - 5, template.u()),
                PackedIndex::new(10 + (face + 4) % 5, template.w()),
                PackedIndex::new(15 + (face + 4) % 5, template.u()),
                PackedIndex::new(face + 5, template.u()),
            ]));
        
        tb
            .chain(um)
            .chain(lm)
    }
    
    fn face_faces_from_template(template: &SubdividedTriangle<N>) -> impl Iterator<Item = ExactFace> {        
        (0..N)
            .map(|x| template.level_x(ImplicitDenominator::wrap(x)).collect::<Vec<_>>())
            .tuple_windows::<(_, _)>()
            .flat_map(|(r1, r2)|
                r1[1..(r1.len() - 1)].iter()
                    .cloned()
                    .zip(r2)
                    .tuple_windows::<(_, _, _)>()
                    .step_by(2)
                    .cartesian_product(0..20)
                    .map(|(((a0, b0), (a1, b1), (a2, b2)), face)| ExactFace::Hexagon([
                        PackedIndex::new(face, a0),
                        PackedIndex::new(face, a1),
                        PackedIndex::new(face, a2),
                        PackedIndex::new(face, b2),
                        PackedIndex::new(face, b1),
                        PackedIndex::new(face, b0)
                    ]))
                    .collect::<Vec<_>>()
            )
    }
}
