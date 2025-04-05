use std::f32::consts::{FRAC_PI_3, FRAC_PI_6};
use std::rc::Rc;
use glam::{IVec3, Vec3};
use crate::denominator::ImplicitDenominator;
use crate::subdivision::triangle::Triangle;

/*
acos(phi/sqrt(phi^2 + 1))
Where phi is the Golden ratio
*/
const FRAC_K_3_32: f32 = 0.5535743588970452515085327300892685200;

pub struct Seed<const N: u32> where
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
    pub fn icosahedron() -> Self {
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
    
    pub fn to_local(&self, f: usize, v: ImplicitDenominator<IVec3, {3 * N }>) -> ImplicitDenominator<IVec3, {3 * N }> {
        let face = &self.faces[f];
        
        ImplicitDenominator::wrap((face.u.0 * v.x + face.v.0 * v.y + face.w.0 * v.z) / 3)
    }
    
    /*
    x is coefficient on pi in phi.
    y is coefficient on k in phi
    z is coefficient on pi/2 in theta.
    */
    pub fn local_to_euclidean(&self, v: &ImplicitDenominator<IVec3, {3 * N}>, radius: f32) -> Vec3 {
        let theta = (v.z as f32 / N as f32) * FRAC_PI_6;
        let phi = v.x as f32 * FRAC_PI_3 / N as f32 + v.y as f32 / N as f32 * FRAC_K_3_32;
        
        Vec3::new(
            theta.sin() * phi.cos(),
            theta.sin() * phi.cos(),
            theta.cos()
        ) * radius
    }
}
