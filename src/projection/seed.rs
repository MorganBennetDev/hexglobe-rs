use std::rc::Rc;
use glam::Vec3;
use crate::subdivision::triangle::Triangle;

pub struct Seed<const N: u32> {
    pub vertices: Vec<Rc<Vec3>>,
    pub faces: Vec<Triangle<Vec3>>,
}

impl<const N: u32> Seed<N> {
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
    pub fn icosahedron() -> Self {
        #[allow(unused_variables)]
        macro_rules! vertex {
            ($x: expr, $y: expr, $z: expr) => {
                Rc::new(
                    Vec3::new($x, $y, $z).normalize()
                )
            };
        }
        
        let c0 = 0.809_017;
        
        let vertices = vec![
            vertex!(0.5, 0.0, c0),
            vertex!(0.5, 0.0, -c0),
            vertex!(-0.5, 0.0, c0),
            vertex!(-0.5, 0.0, -c0),
            vertex!(c0, 0.5, 0.0),
            vertex!(c0, -0.5, 0.0),
            vertex!(-c0, 0.5, 0.0),
            vertex!(-c0, -0.5, 0.0),
            vertex!(0.0, c0, 0.5),
            vertex!(0.0, c0, -0.5),
            vertex!(0.0, -c0, 0.5),
            vertex!(0.0, -c0, -0.5),
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
    
    pub fn get_face(&self, face: usize) -> Triangle<Vec3> {
        self.faces[face].clone()
    }
}
