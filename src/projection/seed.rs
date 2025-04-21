use glam::{Mat3, Vec3};
use crate::subdivision::triangle::Triangle;

#[derive(Clone)]
enum Face {
    Base(Triangle<Vec3>),
    Symmetry(usize, Mat3)
}

pub struct Seed<const N: u32> {
    faces: Vec<Face>,
}

impl<const N: u32> Seed<N> {
    /*
    Outputs the faces of a regular icosahedron in spherical coordinates.
    Source for Cartesian coordinates and faces: github.com/virtualritz/polyhedron-ops
     */
    /// Initializes a seed icosahedron. Currently, this is the only seed option.
    pub fn icosahedron() -> Self {
        macro_rules! vertex {
            ($x: expr, $y: expr, $z: expr) => {
                Vec3::new($x, $y, $z).normalize()
            };
        }
        
        let c0 = 0.809_017;
        
        // It's not really unused
        #[allow(unused_variables)]
        let vertices = vec![
            vertex!(  0.5,  0.0,   c0), // 0
            vertex!(  0.5,  0.0,  -c0), // 1
            vertex!( -0.5,  0.0,   c0), // 2
            vertex!( -0.5,  0.0,  -c0), // 3
            vertex!(   c0,  0.5,  0.0), // 4
            vertex!(   c0, -0.5,  0.0), // 5
            vertex!(  -c0,  0.5,  0.0), // 6
            vertex!(  -c0, -0.5,  0.0), // 7
            vertex!(  0.0,   c0,  0.5), // 8
            vertex!(  0.0,   c0, -0.5), // 9
            vertex!(  0.0,  -c0,  0.5), // 10
            vertex!(  0.0,  -c0, -0.5), // 11
        ];
        
        // Shorthand macro for explicitly defining a triangular face.
        macro_rules! triangle {
            ($u: expr, $v: expr, $w: expr) => {
                Face::Base(Triangle::new(vertices[$u], vertices[$v], vertices[$w]))
            };
        }
        
        // Shorthand macro for defining a face which is the same as another face rotated about some axis.
        macro_rules! rotation {
            ($i: expr, $j: expr, $r: expr) => {
                Face::Symmetry($i, Mat3::from_axis_angle(
                    vertices[$j],
                    (($r) as f32).to_radians()
                ))
            };
        }
        
        // ((0.5, 0, c0), (c0, -0.5, 0), (0, -c0, 0.5)) -> ((0, -c0, -0.5), (0, -c0, 0.5), (c0, -0.5, 0))
        let mut faces = vec![
            // Top
            triangle!(  0,  5,  10 ), // 0
            rotation!(  0,  3,  72 ), // 1
            rotation!(  0,  3, 144 ), // 2
            rotation!(  0,  3, 216 ), // 3
            rotation!(  0,  3, 288 ), // 4
            // Upper middle
            triangle!( 11, 10,   5 ), // 5
            rotation!(  5,  3,  72 ), // 6
            rotation!(  5,  3, 144 ), // 7
            rotation!(  5,  3, 216 ), // 8
            rotation!(  5,  3, 288 ), // 9
            // Lower middle
            triangle!( 10, 11,   7 ), // 10
            rotation!( 10,  3,  72 ), // 11
            rotation!( 10,  3, 144 ), // 12
            rotation!( 10,  3, 216 ), // 13
            rotation!( 10,  3, 288 ), // 14
            // Bottom
            triangle!(  3,  7,  11 ), // 15
            rotation!( 15,  3,  72 ), // 16
            rotation!( 15,  3, 144 ), // 17
            rotation!( 15,  3, 216 ), // 18
            rotation!( 15,  3, 288 ), // 19
        ];
        
        Self {
            faces,
        }
    }
    
    /// Get the faces which are defined as transformed versions of other faces.
    /// 
    /// Returns a [Vec] of `(usize, usize, Mat3)` where the first entry is the index of the face being defined, the
    /// second is the index of the face being transformed, and the third is a matrix representing the transformation
    /// to turn the base face into the current face.
    pub fn symmetries(&self) -> Vec<(usize, usize, Mat3)> {
        self.faces.iter()
            .enumerate()
            .filter_map(|(i, face)| match face {
                Face::Base(_) => None,
                Face::Symmetry(f, t) => Some((i, f.clone(), t.clone()))
            })
            .collect::<Vec<_>>()
    }
    
    /// Get the faces which are defined explicitly.
    /// 
    /// Returns a [Vec] of `(usize, Triangle<Vec3>)` where the first entry is the index of the given face and the second
    /// is the representation of that face in Euclidean space.
    pub fn base_faces(&self) -> Vec<(usize, Triangle<Vec3>)> {
        self.faces.iter()
            .enumerate()
            .filter_map(|(i, face)| match face {
                Face::Base(t) => Some((i, t.clone())),
                Face::Symmetry(_, _) => None
            })
            .collect::<Vec<_>>()
    }
}
