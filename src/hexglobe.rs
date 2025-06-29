#![doc = include_str!("hexglobe/DOCS.md")]

#[cfg(test)]
mod tests;

pub(crate) mod packed_index;
mod seed;

use glam::{Vec2, Vec3};
use itertools::Itertools;
use crate::slerp::slerp_3;
use packed_index::PackedIndex;
use seed::Seed;
use crate::subdivided_triangle::SubdividedTriangle;

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

impl ExactFace {
    const fn construct_hexagon(
        (f0, s0): (usize, usize),
        (f1, s1): (usize, usize),
        (f2, s2): (usize, usize),
        (f3, s3): (usize, usize),
        (f4, s4): (usize, usize),
        (f5, s5): (usize, usize)
    ) -> Self {
        Self::Hexagon([
            PackedIndex::new(f0, s0),
            PackedIndex::new(f1, s1),
            PackedIndex::new(f2, s2),
            PackedIndex::new(f3, s3),
            PackedIndex::new(f4, s4),
            PackedIndex::new(f5, s5)
        ])
    }
    
    const fn construct_pentagon(
        (f0, s0): (usize, usize),
        (f1, s1): (usize, usize),
        (f2, s2): (usize, usize),
        (f3, s3): (usize, usize),
        (f4, s4): (usize, usize)
    ) -> Self {
        Self::Pentagon([
            PackedIndex::new(f0, s0),
            PackedIndex::new(f1, s1),
            PackedIndex::new(f2, s2),
            PackedIndex::new(f3, s3),
            PackedIndex::new(f4, s4)
        ])
    }
}

impl Default for ExactFace {
    fn default() -> Self {
        Self::Hexagon([PackedIndex::default(); 6])
    }
}

/// Represents a face of a Goldberg polyhedron as a list of (indices to) vertices in counterclockwise winding order.
#[derive(Copy, Clone, Debug)]
pub enum MeshFace {
    Pentagon([u32; 5]),
    Hexagon([u32; 6])
}

impl Default for MeshFace {
    fn default() -> Self {
        Self::Hexagon([0; 6])
    }
}

#[derive(Copy, Clone, Debug)]
enum FaceUV {
    Pentagon([[f32; 2]; 5]),
    Hexagon([[f32; 2]; 6])
}

impl FaceUV {
    fn as_array<const N: usize>(&self) -> [[f32; 2]; N] {
        let mut output = [[0.0; 2]; N];
        
        match self {
            FaceUV::Pentagon(v) => &output[0..N].copy_from_slice(&v[0..N]),
            FaceUV::Hexagon(v) => &output[0..N].copy_from_slice(&v[0..N])
        };
        
        output
    }
}

// https://www.johndcook.com/blog/2023/08/27/intersect-circles/
// Assumes input is valid
fn circle_intersect(p0: Vec2, r0: f32, p1: Vec2, r1: f32) -> (Vec2, Vec2) {
    let v = p1 - p0;
    
    let d = v.length();
    let u = v.normalize();
    
    let xvec = p0 + (d * d - r1 * r1 + r0 * r0) * u / (2.0 * d);
    
    let uperp = Vec2::new(u.y, -u.x);
    let a = ((-d + r1 - r0) * (-d - r1 + r0) * (-d + r1 + r0) * (d + r1 + r0)).sqrt() / d;
    
    (xvec + a * uperp / 2.0, xvec - a * uperp / 2.0)
}

/// Contains functionality to create a Goldberg polyhedron from an icosahedron whose faces have been subdivided `N`
/// times.
///
/// # Creating a Mesh
///
/// The process of creating a mesh is designed to expose as much data as necessary and reasonable to the consumer,
/// allowing them to perform each step if and when they need to and avoiding the need to write caching logic for complex
/// backend calculations. This leads to a slightly verbose interface, but the tradeoffs should be worth the flexibility.
///
/// ```
/// use hexglobe::HexGlobe;
///
/// let globe = HexGlobe::<4>::new();
///
/// let centroids = globe.centroids(None);
/// let vertices = globe.mesh_vertices(&centroids);
/// let faces = globe.mesh_faces();
/// let triangles = globe.mesh_triangles(&faces);
/// let normals = globe.mesh_normals(&vertices);
///
/// // ... Pass vertices, triangles, and normals to your rendering library of choice.
/// ```
///
/// # Creating an Adjacency Graph
///
/// The [adjacency](#method.adjacency) method returns a list of tuples representing undirected edges between faces in the generated
/// polyhedron. This method can make assumptions about how faces are ordered, making it much faster than a naive
/// implementation.
///
/// This method was specifically provided for use in tile-based games, but it can be used for anything that needs an
/// adjacency graph.
///
/// ```
/// use hexglobe::HexGlobe;
///
/// let globe = HexGlobe::<4>::new();
///
/// let adjacency = globe.adjacency();
///
/// // ... Pass adjacency to your graph backend of choice.
/// ```
pub struct HexGlobe<const N: u32> {
    seed: Seed<N>,
    subdivision: SubdividedTriangle<N>,
    faces: Vec<ExactFace>,
}

impl<const N: u32> HexGlobe<N> {
    const SEED_VERTICES: usize = 12;
    const SEED_EDGES: usize = 30;
    const SEED_FACES: usize = 20;
    const FACES_PER_VERTEX: usize = 1;
    const FACES_PER_EDGE: usize = N as usize - 1;
    const FACES_PER_FACE: usize = ((N - 1) * (max(N, 2) - 2) / 2) as usize;
    /// The number of pentagons in this tiling. This will always be 12 due to the mathematical properties of the tiling.
    pub const PENTAGONS: usize = Self::SEED_VERTICES;
    /// The number of hexagons in this tiling.
    pub const HEXAGONS: usize = Self::SEED_EDGES * Self::FACES_PER_EDGE + Self::SEED_FACES * Self::FACES_PER_FACE;
    /// The total number of faces in this tiling ([PENTAGONS](#associatedconstant.PENTAGONS) +
    /// [HEXAGONS](#associatedconstant.HEXAGONS))
    pub const FACES: usize = Self::PENTAGONS + Self::HEXAGONS;
    const MESH_PENTAGON_VERTICES: usize = Self::PENTAGONS * 5;
    const MESH_HEXAGON_VERTICES: usize = Self::HEXAGONS * 6;
    /// The number of vertices in the produced mesh. This is useful for reducing unnecessary memory allocations while
    /// creating the mesh.
    pub const MESH_VERTICES: usize = Self::MESH_PENTAGON_VERTICES + Self::MESH_HEXAGON_VERTICES;
    const MESH_PENTAGON_TRIANGLES: usize = 3;
    const MESH_HEXAGON_TRIANGLES: usize = 4;
    pub const MESH_TRIANGLES: usize = Self::PENTAGONS * Self::MESH_PENTAGON_TRIANGLES + Self::HEXAGONS * Self::MESH_HEXAGON_TRIANGLES;
    
    /// Initializes the data for a new polyhedron. This is very cheap as all the expensive computations are done during
    /// conversion to floating point coordinates.
    pub fn new() -> Self {
        let subdivision = SubdividedTriangle::<N>::new();
        let seed = Seed::<N>::icosahedron();
        
        let mut faces = vec![ExactFace::default(); Self::FACES];
        
        Self::faces_from_template(&subdivision)
            .enumerate()
            .for_each(|(i, face)| faces[i] = face);
        
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
            .map(|((face_a, face_b), ((a0, b0), (a1, b1), (a2, b2)))|
                ExactFace::construct_hexagon(
                (face_a, a0),
                (face_a, a1),
                (face_a, a2),
                (face_b, b2),
                (face_b, b1),
                (face_b, b0)
                )
            )
    }
    
    fn vertex_faces_from_template(template: &SubdividedTriangle<N>) -> impl Iterator<Item = ExactFace> {
        let tb = [
            ExactFace::construct_pentagon(
                (4, template.u()),
                (3, template.u()),
                (2, template.u()),
                (1, template.u()),
                (0, template.u()),
            ),
            ExactFace::construct_pentagon(
                (15, template.u()),
                (16, template.u()),
                (17, template.u()),
                (18, template.u()),
                (19, template.u()),
            )
        ].into_iter();
        
        let um = (5..10)
            .map(|face| ExactFace::construct_pentagon(
                (face - 5, template.w()),
                ((face + 1) % 5, template.v()),
                (5 + (face + 1) % 5, template.w()),
                (face + 5, template.u()),
                (face, template.v()),
            ));
        
        let lm = (10..15)
            .map(|face| ExactFace::construct_pentagon(
                (face + 5, template.w()),
                (15 + (face + 4) % 5, template.v()),
                (10 + (face + 4) % 5, template.w()),
                (face - 5, template.u()),
                (face, template.v()),
            ));
        
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
            .map(|(face, ((a0, b0), (a1, b1), (a2, b2)))|
                ExactFace::construct_hexagon(
                    (face, a0),
                    (face, a1),
                    (face, a2),
                    (face, b2),
                    (face, b1),
                    (face, b0)
                )
            )
    }
    
    fn vertex_index_to_face_index(&self, f: usize, i: usize) -> usize {
        let v = self.subdivision.vertex_denominator(i);
        
        match (v.x, v.y, v.z) {
            // Vertices
            (_, 0, 0) => match f { // u
                0..5 => 0,
                5..10 => 7 + f % 5,
                10..15 => 2 + f % 5,
                // 15..20
                _ => 1,
            },
            (0, _, 0) => match f { // v
                0..5 => 2 + (f + 4) % 5,
                5..10 => 2 + f % 5,
                10..15 => 7 + f % 5,
                // 15..20
                _ => 7 + (f + 1) % 5,
            },
            (0, 0, _) => match f { // w
                0..5 => 2 + f,
                5..10 => 2 + (f + 4) % 5,
                10..15 => 7 + (f + 1) % 5,
                // 15..20
                _ => 7 + f % 5,
            },
            // Edges
            (_, _, 0) | (0, _, _) | (_, 0, _) => {
                let offset = Self::SEED_VERTICES * Self::FACES_PER_VERTEX - 1;
                
                match (v.x, v.y, v.z) {
                    (_, _, 0) => match f { // uv
                        0..5 => offset + ((f + 4) % 5) * Self::FACES_PER_EDGE + v.x as usize,
                        5..10 => offset + (10 + f % 5) * Self::FACES_PER_EDGE + v.y as usize,
                        10..15 => offset + (10 + f % 5) * Self::FACES_PER_EDGE + v.x as usize,
                        // 15..20
                        _ => offset + (25 + f % 5) * Self::FACES_PER_EDGE + v.y as usize,
                    },
                    (0, _, _) => match f { // vw
                        0..5 => offset + (5 + f) * Self::FACES_PER_EDGE + v.z as usize,
                        5..10 => offset + (5 + f % 5) * Self::FACES_PER_EDGE + v.y as usize,
                        10..15 => offset + (20 + f % 5) * Self::FACES_PER_EDGE + v.z as usize,
                        // 15..20
                        _ => offset + (20 + f % 5) * Self::FACES_PER_EDGE + v.y as usize,
                    },
                    // This is just (_, 0, _), but the interpreter doesn't know that other cases aren't possible.
                    _ => match f { // wu
                        0..5 => offset + f * Self::FACES_PER_EDGE + v.x as usize,
                        5..10 => offset + (15 + (f + 4) % 5) * Self::FACES_PER_EDGE + v.z as usize,
                        10..15 => offset + (15 + f % 5) * Self::FACES_PER_EDGE + v.x as usize,
                        // 15..20
                        _ => offset + (25 + (f + 4) % 5) * Self::FACES_PER_EDGE + v.z as usize,
                    }
                }
            },
            // Faces
            _ => {
                let offset = Self::SEED_VERTICES * Self::FACES_PER_VERTEX +
                    Self::SEED_EDGES * Self::FACES_PER_EDGE +
                    f * Self::FACES_PER_FACE;
                
                // Index of vertex i in the set of vertices excluding edges.
                let j = self.subdivision.vertex_interior_index_unchecked(v);
                
                offset + j
            }
        }
    }
    
    /// [Vec] of undirected edges between adjacent faces represented by tuples of face indices. The output will never
    /// contain duplicate edges but no other guarantees are made. Edges may appear in any order in the list and edge
    /// endpoints may appear in any order in the corresponding tuple.
    pub fn adjacency(&self) -> Vec<(usize, usize)> {
        let mut adjacency = vec![(0, 0); SubdividedTriangle::<N>::EDGES * 20];
        
        for (i, (a, b)) in self.subdivision.vertex_adjacency().enumerate() {
            let n = i * 20;
            
            for f in 0..20 {
                adjacency[n + f] = (
                    self.vertex_index_to_face_index(f, a),
                    self.vertex_index_to_face_index(f, b)
                );
            }
        }
        
        adjacency
    }
    
    /// Generates vertices of a Goldberg polyhedron with an optional radius (default of 1.0), which are the centroids of
    /// the subdivided triangular faces. This is the most expensive operation as it utilizes the `slerp_3` function.
    /// Optimizations have been made to exploit some of the symmetries between faces and only compute vertices for 5 of
    /// the 20 subdivided faces. Further optimizations can be made to  only compute one and may be implemented in the
    /// future.
    pub fn centroids(&self, r: Option<f32>) -> Vec<Vec<Vec3>> {
        let radius = r.unwrap_or(1.0);
        
        let n = SubdividedTriangle::<N>::TRIANGLES;
        
        let face_vertices = vec![Vec3::ZERO; n];
        let mut vertices = vec![face_vertices.clone(); 20];
        
        for (f, face) in self.seed.base_faces() {
            for (i, t) in self.subdivision.triangles().enumerate() {
                let centroid = (t.u + t.v + t.w).as_vec3() / (3 * N) as f32;
                
                vertices[f][i] = slerp_3(
                    centroid.x, face.u,
                    centroid.y, face.v,
                    centroid.z, face.w
                ) * radius;
            }
        }
        
        for (f, base, s) in self.seed.symmetries() {
            for i in 0..n {
                vertices[f][i] = s.mul_vec3(vertices[base][i]);
            }
        }
        
        vertices
    }
    
    /// Generates the vertex buffer for a mesh of the given Goldberg polyhedron where `centroids` is a reference to the
    /// output of the [centroids](#method.centroids) method.
    pub fn mesh_vertices(&self, centroids: &Vec<Vec<Vec3>>) -> Vec<[f32; 3]> {
        let mut vertices = vec![[0.0; 3]; Self::MESH_VERTICES];
        let mut n = 0;
        
        for face in &self.faces {
            match face {
                ExactFace::Pentagon(v) => {
                    vertices[n..(n + 5)].copy_from_slice(
                        &v[0..5].iter()
                            .map(|i| centroids[i.face()][i.subdivision()].to_array())
                            .collect_array::<5>()
                            .unwrap()[0..5]
                    );
                    n += 5;
                }
                ExactFace::Hexagon(v) => {
                    vertices[n..(n + 6)].copy_from_slice(
                        &v[0..6].iter()
                            .map(|i| centroids[i.face()][i.subdivision()].to_array())
                            .collect_array::<6>()
                            .unwrap()[0..6]
                    );
                    n += 6;
                }
            }
        }
        
        vertices
    }
    
    /// Returns a list of the faces of the given Goldberg polyhedron used as a preliminary step in
    /// [mesh_triangles](#method.mesh_triangles) but can also be used independently.
    pub fn mesh_faces(&self) -> Vec<MeshFace> {
        let mut output = vec![MeshFace::default(); Self::FACES];
        
        let pentagons = &mut output[0..Self::PENTAGONS];
        
        for i in 0..Self::PENTAGONS {
            let n = (i * 5) as u32;
            pentagons[i] = MeshFace::Pentagon([n, n + 1, n + 2, n + 3, n + 4]);
        }
        
        let hexagons = &mut output[Self::PENTAGONS..Self::FACES];
        let k = (Self::PENTAGONS * 5) as u32;
        
        for i in 0..Self::HEXAGONS {
            let n = k + (i * 6) as u32;
            hexagons[i] = MeshFace::Hexagon([n, n + 1, n + 2, n + 3, n + 4, n + 5]);
        }
        
        output
    }
    
    /// Generates the triangle buffer for a mesh of the given Goldberg polyhedron with radius `r` (default 1.0). Vertex
    /// indices are deterministic so this is a cheap function and can be called independently of vertex computation. The
    /// `faces` parameter should be a reference to the output of [mesh_faces](#method.mesh_faces).
    pub fn mesh_triangles(&self, faces: &Vec<MeshFace>) -> Vec<u32> {
        let mut output = vec![0; Self::MESH_TRIANGLES * 3];
        
        let mut n = 0;
        
        for face in faces {
            match face {
                MeshFace::Pentagon(v) => {
                    output[n..(n + 9)].copy_from_slice(&[
                        v[0], v[1], v[2],
                        v[0], v[2], v[3],
                        v[0], v[3], v[4]
                    ]);
                    n += 9;
                }
                MeshFace::Hexagon(v) => {
                    output[n..(n + 12)].copy_from_slice(&[
                        v[0], v[1], v[2],
                        v[0], v[2], v[3],
                        v[0], v[3], v[4],
                        v[0], v[4], v[5]
                    ]);
                    n += 12;
                }
            }
        }
        
        output
    }
    
    /// Computes the normals for the mesh of this polyhedron. This method is much faster than an external implementation
    /// because it can make assumptions about the input data. The `vertices` parameter should be a reference to the
    /// output of [mesh_vertices].
    pub fn mesh_normals(&self, vertices: &Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        assert_eq!(vertices.len(), Self::MESH_VERTICES, "Incorrect number of vertices passed to mesh_normals.");
        
        let mut normals = vec![[0.0; 3]; Self::MESH_VERTICES];
        
        for i in (0..Self::MESH_PENTAGON_VERTICES).step_by(5) {
            let pentagon = &vertices[i..(i + 5)];
            
            let [u, v, w] = [
                Vec3::from(pentagon[0]),
                Vec3::from(pentagon[2]),
                Vec3::from(pentagon[3])
            ];
            
            let normal = (v - u).cross(w - u).normalize().to_array();
            
            for k in 0..5 {
                normals[i + k] = normal;
            }
        }
        
        for i in (Self::MESH_PENTAGON_VERTICES..Self::MESH_VERTICES).step_by(6) {
            let hexagon = &vertices[i..(i + 6)];
            
            let [u, v, w] = [
                Vec3::from(hexagon[0]),
                Vec3::from(hexagon[2]),
                Vec3::from(hexagon[4])
            ];
            
            let normal = (v + u + w).normalize().to_array();
            
            for k in 0..6 {
                normals[i + k] = normal;
            }
        }
        
        normals
    }
}
