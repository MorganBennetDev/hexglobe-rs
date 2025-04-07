use std::f32::consts::{FRAC_PI_3, PI};
use std::ops::Deref;
use std::rc::Rc;
use glam::{IVec3, Vec3};
use num::{Rational32, ToPrimitive};
use crate::denominator::ImplicitDenominator;
use crate::subdivision::triangle::Triangle;

/*
acos(phi/sqrt(phi^2 + 1))
Where phi is the Golden ratio
*/
const K_32: f32 = 0.5535743588970452515085327300892685200;

pub struct Seed<const N: u32> {
    vertices: Vec<Rc<IVec3>>,
    faces: Vec<Triangle<IVec3>>,
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
        let denominator = 3 * N as i32;
        macro_rules! vertex {
            ($x: expr, $y: expr, $z: expr) => {
                // Factor of 3 in denominator is necessary for centroid calculations
                Rc::new(
                    IVec3::new($x, $y, $z / 90)
                )
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
    
    pub fn to_local(&self, f: usize, v: ImplicitDenominator<ImplicitDenominator<IVec3, N>, 3>) -> ImplicitDenominator<ImplicitDenominator<IVec3, N>, 3> {
        let face = &self.faces[f];
        
        ImplicitDenominator::wrap(
            ImplicitDenominator::wrap(
                face.u.deref() * v.x + face.v.deref() * v.y + face.w.deref() * v.z
            )
        )
    }
    
    /*
    x is coefficient on pi in phi.
    y is coefficient on k in phi
    z is coefficient on pi/2 in theta.
    */
    pub fn local_to_euclidean<const M: u32>(&self, v: &ImplicitDenominator<ImplicitDenominator<IVec3, N>, M>, radius: f32) -> Vec3 {
        let (x, y, z) = (
            Rational32::new(v.x, (M * N) as i32).to_f32().unwrap(),
            Rational32::new(v.y, (M * N) as i32).to_f32().unwrap(),
            Rational32::new(v.z, (M * N) as i32).to_f32().unwrap()
        );
        let theta = z * FRAC_PI_3;
        let phi = x * PI + y * K_32;
        println!("{:?},{:?}", theta, phi);
        
        Vec3::new(
            theta.sin() * phi.cos(),
            theta.sin() * phi.sin(),
            theta.cos()
        ) * radius
    }
}
