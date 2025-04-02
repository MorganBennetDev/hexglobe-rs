use std::rc::Rc;
use glam::{IVec2, IVec3};
use petgraph::graph::UnGraph;
use crate::denominator::ImplicitDenominator;
use crate::triangle::{SubdividedTriangle, Triangle};

enum Face {
    Pentagon([usize; 5]),
    Hexagon([usize; 6])
}

struct Seed<const N: u32> where
    [(); (3 * N) as usize] : Sized {
    vertices: Vec<Rc<ImplicitDenominator<IVec3, {3 * N}>>>,
    faces: Vec<Triangle<ImplicitDenominator<IVec3, {3 * N}>>>,
    adjacency: UnGraph<u32, ()>
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
        let denominator = 3 * N as i32;
        macro_rules! vertex {
            ($x: expr, $y: expr, $z: expr) => {
                // Factor of 3 in denominator is necessary for centroid calculations
                Rc::new(ImplicitDenominator::<IVec3, {3 * N }>::wrap(IVec3::new($x * denominator, $y * denominator, $z / 90)))
            };
        }
        let vertices = vec![
            vertex!( 0,  1, 0 ), // 0
            vertex!( 1, -1, 0 ), // 1
            vertex!( 0,  1, 2 ), // 2
            vertex!( 1, -1, 2 ), // 3
            vertex!( 0,  1, 1 ), // 4
            vertex!( 0, -1, 1 ), // 5
            vertex!( 1, -1, 1 ), // 6
            vertex!(-1,  1, 1 ), // 7
            vertex!( 0,  1, 1 ), // 8
            vertex!( 1, -1, 3 ), // 9
            vertex!( 0,  1, 3 ), // 10
            vertex!( 1, -1, 3 ), // 11
        ];
        
        macro_rules! triangle {
            ($u: expr, $v: expr, $w: expr) => {
                Triangle::new(vertices[$u].clone(), vertices[$v].clone(), vertices[$w].clone())
            };
        }
        
        let faces = vec![
            triangle!( 10,  2, 0  ), // 0
            triangle!(  5, 10, 0  ), // 1
            triangle!(  4,  5, 0  ), // 2
            triangle!(  8,  4, 0  ), // 3
            triangle!(  2,  8, 0  ), // 4
            triangle!(  6,  8, 2  ), // 5
            triangle!(  7,  6, 2  ), // 6
            triangle!( 10,  7, 2  ), // 7
            triangle!( 11,  7, 10 ), // 8
            triangle!(  5, 11, 10 ), // 9
            triangle!(  1, 11, 5  ), // 10
            triangle!(  4,  1, 5  ), // 11
            triangle!(  9,  1, 4  ), // 12
            triangle!(  8,  9, 4  ), // 13
            triangle!(  6,  9, 8  ), // 14
            triangle!(  3,  9, 6  ), // 15
            triangle!(  7,  3, 6  ), // 16
            triangle!( 11,  3, 7  ), // 17
            triangle!(  1,  3, 11 ), // 18
            triangle!(  9,  3, 1  ), // 19
        ];
        
        let adjacency = UnGraph::from_edges(vec![
            (0, 1), (0, 4), (0, 7),
            (1, 2), (1, 9),
            (2, 3), (2, 11),
            (3, 4), (3, 13),
            (4, 5),
            (5, 6), (5, 14),
            (6, 7), (6, 16),
            (7, 8),
            (8, 9), (8, 17),
            (9, 10),
            (10, 11), (10, 18),
            (11, 12),
            (12, 13), (12, 19),
            (13, 14),
            (14, 15),
            (15, 16), (15, 19),
            (16, 17),
            (17, 18),
            (18, 19)
        ]);
        
        Self {
            vertices,
            faces,
            adjacency
        }
    }
}

pub struct Globe<const N: u32> {
    vertices: Vec<IVec2>,
    faces: Vec<Face>,
    adjacency: UnGraph<u32, ()>
}

impl<const N: u32> Globe<N> where
    [(); (3 * N) as usize] : Sized {
    fn new() {
        let template = SubdividedTriangle::<N>::new();
        let seed = Seed::<N>::icosahedron();
    }
}
