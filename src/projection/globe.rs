use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use glam::IVec3;
use itertools::Itertools;
use petgraph::graph::IndexType;
use crate::denominator::ImplicitDenominator;
use crate::subdivision::triangle::{SubdividedTriangle, Triangle};

struct Seed<const N: u32> where
    [(); (3 * N) as usize] : Sized {
    vertices: Vec<Rc<ImplicitDenominator<IVec3, {3 * N}>>>,
    faces: Vec<Triangle<ImplicitDenominator<IVec3, {3 * N}>>>,
}

impl<const N: u32> Seed<N> where
    [(); (3 * N) as usize] : Sized {
    /*
    Outputs the faces of a regular icosahedron in spherical coordinates.
    Source for Cartesian coordinates and faces: github.com/virtualritz/polyhedron-ops
     1,  0,  p |       k, 0
     1,  0, -p |    pi-k, 0
    -1,  0,  p |       k, 180
    -1,  0, -p |    pi-k, 180
     p,  1,  0 |       k, 90
     p, -1,  0 |      -k, 90
    -p,  1,  0 |    pi-k, 90
    -p, -1,  0 | -(pi-k), 90
     0,  p,  1 |       k, 90
     0,  p, -1 |    pi-k, 90
     0, -p,  1 |       k, 270
     0, -p, -1 |    pi-k, 270
     */
    fn icosahedron() -> Self {
        #[allow(unused_variables)]
        let denominator = 3 * N as i32;
        macro_rules! vertex {
            ($x: expr, $y: expr, $z: expr) => {
                // Factor of 3 in denominator is necessary for centroid calculations
                Rc::new(ImplicitDenominator::<IVec3, {3 * N }>::wrap(IVec3::new($x * denominator, $y * denominator, $z / 90)))
            };
        }
        let vertices = vec![
            vertex!( 0,  1, 0 ),   // 0
            vertex!( 1, -1, 0 ),   // 1
            vertex!( 0,  1, 180 ), // 2
            vertex!( 1, -1, 180 ), // 3
            vertex!( 0,  1, 90 ),  // 4
            vertex!( 0, -1, 90 ),  // 5
            vertex!( 1, -1, 90 ),  // 6
            vertex!(-1,  1, 90 ),  // 7
            vertex!( 0,  1, 90 ),  // 8
            vertex!( 1, -1, 270 ), // 9
            vertex!( 0,  1, 270 ), // 10
            vertex!( 1, -1, 270 ), // 11
        ];
        
        macro_rules! triangle {
            ($u: expr, $v: expr, $w: expr) => {
                Triangle::new(vertices[$u].clone(), vertices[$v].clone(), vertices[$w].clone())
            };
        }
        
        let faces = vec![
            // Top
            triangle!(  0, 10,  2 ), // 0
            triangle!(  0,  5, 10 ), // 1
            triangle!(  0,  4,  5 ), // 2
            triangle!(  0,  8,  4 ), // 3
            triangle!(  0,  2,  8 ), // 4
            // Upper middle
            triangle!(  7,  2, 10 ), // 5
            triangle!( 11, 10,  5 ), // 6
            triangle!(  1,  5,  4 ), // 7
            triangle!(  9,  4,  8 ), // 8
            triangle!(  6,  8,  2 ), // 9
            // Lower middle
            triangle!( 10, 11,  7 ), // 10
            triangle!(  5,  1, 11 ), // 11
            triangle!(  4,  9,  1 ), // 12
            triangle!(  8,  6,  9 ), // 13
            triangle!(  2,  7,  6 ), // 14
            // Bottom
            triangle!(  3,  7, 11 ), // 15
            triangle!(  3, 11,  1 ), // 16
            triangle!(  3,  1,  9 ), // 17
            triangle!(  3,  9,  6 ), // 18
            triangle!(  3,  6,  7 ), // 19
        ];
        
        Self {
            vertices,
            faces,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PackedIndex(usize);

impl PackedIndex {
    const fn new(face: usize, subdivision: usize) -> Self {
        Self((subdivision << 5) | face)
    }
    
    const fn face(&self) -> usize {
        self.0 & 0b11111
    }
    
    const fn subdivision(&self) -> usize {
        self.0 >> 5
    }
}

unsafe impl IndexType for PackedIndex {
    fn new(x: usize) -> Self {
        Self(x)
    }
    
    fn index(&self) -> usize {
        self.0
    }
    
    fn max() -> Self {
        Self(usize::MAX)
    }
}

enum ExactFace {
    Pentagon([PackedIndex; 5]),
    Hexagon([PackedIndex; 6])
}

pub struct Globe<const N: u32> where
    [(); (3 * N) as usize] : Sized {
    pub vertices: HashMap<PackedIndex, ImplicitDenominator<IVec3, {3 * N}>>,
    pub faces: Vec<ExactFace>,
    // adjacency: UnGraph<u32, ()>
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
    um-lm: uv-vu, wu-uw
    lm-b:  vw-wv
    b-b:   uv-uw
    */
    fn faces_from_template<const M: u32>(template: &SubdividedTriangle<M>) -> impl Iterator<Item = ExactFace> {
        Self::vertex_faces_from_template(&template)
            .chain(
                Self::edge_faces_from_template(&template)
            )
            .chain(
                Self::face_faces_from_template(&template)
            )
    }
    
    fn edge_faces_from_template<const M: u32>(template: &SubdividedTriangle<M>) -> impl Iterator<Item = ExactFace> {
        let t_t = template.wu()
            .zip(template.uv().rev())
            .tuple_windows::<(_, _, _)>()
            .cartesian_product((0..5).map(|face| (face, (face + 1) % 5)));
        
        let t_um = template.vw()
            .zip(template.vw().rev())
            .tuple_windows::<(_, _, _)>()
            .cartesian_product((0..5).map(|face| (face, face + 5)));
        
        let um_lm = template.uv()
            .zip(template.uv().rev())
            .tuple_windows::<(_, _, _)>()
            .cartesian_product((5..10).map(|face| (face, face + 5)));
        
        let lm_um = template.wu()
            .zip(template.wu().rev())
            .tuple_windows::<(_, _, _)>()
            .cartesian_product((10..15).map(|face| (face, 5 + face % 5)));
        
        let lm_b = template.vw()
            .zip(template.vw().rev())
            .tuple_windows::<(_, _, _)>()
            .cartesian_product((10..15).map(|face| (face, face + 5)));
        
        let b_b = template.uv()
            .zip(template.wu().rev())
            .tuple_windows::<(_, _, _)>()
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
    
    fn vertex_faces_from_template<const M: u32>(template: &SubdividedTriangle<M>) -> impl Iterator<Item = ExactFace> {
        let tb = (0..5)
            .chain(15..16)
            .map(|face| ExactFace::Pentagon([
                PackedIndex::new(face, template.u()),
                PackedIndex::new(face, template.u()),
                PackedIndex::new(face, template.u()),
                PackedIndex::new(face, template.u()),
                PackedIndex::new(face, template.u()),
            ]));
        
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
    
    fn face_faces_from_template<const M: u32>(template: &SubdividedTriangle<M>) -> impl Iterator<Item = ExactFace> {
        (0..M)
            .map(|x| template.level_x(ImplicitDenominator::wrap(x)).collect::<Vec<_>>())
            .tuple_windows::<(_, _)>()
            .flat_map(|(r1, r2)|
                r1.iter()
                    .skip(1)
                    .take(r1.len() - 2)
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