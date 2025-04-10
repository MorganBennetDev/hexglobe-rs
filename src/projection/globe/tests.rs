use crate::projection::globe::{ExactFace, ExactGlobe};
use crate::subdivision::subdivided_triangle::SubdividedTriangle;

// Number of vertices, edges, and faces of icosahedron.
const V: usize = 12;
const E: usize = 30;
const F: usize = 20;

fn face_creation_test_hexagons<const N: u32>() {
    let template = SubdividedTriangle::<N>::new();
    let edge_faces = ExactGlobe::<N>::edge_faces_from_template(&template).count();
    let face_faces = ExactGlobe::<N>::face_faces_from_template(&template).count();
    
    let n_edge_faces = E * (N - 1) as usize;
    let n_face_faces = F * ((N - 1) * (N.max(2) - 2) / 2) as usize;
    
    assert_eq!(edge_faces, n_edge_faces, "Incorrect number of hexagons crossing edges for icosahedron with {:?} subdivisions.", N);
    assert_eq!(face_faces, n_face_faces, "Incorrect number of hexagons within faces for icosahedron with {:?} subdivisions.", N);
    
    let pentagon = ExactGlobe::<N>::edge_faces_from_template(&template).position(|v| match v {
        ExactFace::Pentagon(_) => true,
        _ => false
    });
    
    assert_eq!(pentagon, None, "Found pentagon crossing edge for icosahedron with {:?} subdivisions.", N);
    
    let pentagon = ExactGlobe::<N>::face_faces_from_template(&template).position(|v| match v {
        ExactFace::Pentagon(_) => true,
        _ => false
    });
    
    assert_eq!(pentagon, None, "Found pentagon within face for icosahedron with {:?} subdivisions.", N);
}

fn face_creation_test_pentagons<const N: u32>() {
    let template = SubdividedTriangle::<N>::new();
    let vertex_faces = ExactGlobe::<N>::vertex_faces_from_template(&template).count();
    
    let n_vertex_faces = V;
    
    assert_eq!(vertex_faces, n_vertex_faces, "Incorrect number of pentagons on vertices for icosahedron with {:?} subdivisions.", N);
    
    let hexagon = ExactGlobe::<N>::vertex_faces_from_template(&template).position(|v| match v {
        ExactFace::Hexagon(_) => true,
        _ => false
    });
    
    assert_eq!(hexagon, None, "Found hexagonal face lying on a vertex for icosahedron with {:?} subdivisions.", N);
}

fn basic_count_test<const N: u32>() {
    let globe = ExactGlobe::<N>::new();
    let n_vertices_expected = F * (N * N) as usize;
    let n_faces = V + E * (N - 1) as usize + F * ((N - 1) * (N.max(2) - 2) / 2) as usize;
    let n_vertices = globe.vertices_f32(None).keys().count();
    
    assert_eq!(n_vertices, n_vertices_expected, "Incorrect number of vertices in icosahedron with {:?} subdivisions.", N);
    assert_eq!(globe.faces.len(), n_faces, "Incorrect number of faces in icosahedron with {:?} subdivisions.", N);
}

fn hexagon_count_test<const N: u32>() {
    let globe = ExactGlobe::<N>::new();
    let expected = E * (N - 1) as usize + F * ((N - 1) * (N.max(2) - 2) / 2) as usize;
    
    let (hexagons, _): (Vec<_>, Vec<_>) = globe.faces.iter()
        .partition(|f| match f {
            ExactFace::Hexagon(_) => true,
            ExactFace::Pentagon(_) => false
        });
    assert_eq!(hexagons.len(), expected, "Incorrect number of hexagons in icosahedron with {:?} subdivisions.", N);
}

fn pentagon_count_test<const N: u32>() {
    let globe = ExactGlobe::<N>::new();
    
    let (_, pentagons): (Vec<_>, Vec<_>) = globe.faces.iter()
        .partition(|f| match f {
            ExactFace::Hexagon(_) => true,
            ExactFace::Pentagon(_) => false
        });
    assert_eq!(pentagons.len(), V, "Incorrect number of pentagons in icosahedron with {:?} subdivisions.", N);
}

#[test]
fn basic_counts() {
    basic_count_test::<1>();
    basic_count_test::<2>();
    basic_count_test::<3>();
    basic_count_test::<4>();
}

#[test]
fn hexagon_counts_total() {
    hexagon_count_test::<1>();
    hexagon_count_test::<2>();
    hexagon_count_test::<3>();
    hexagon_count_test::<4>();
}

#[test]
fn pentagon_counts_total() {
    pentagon_count_test::<1>();
    pentagon_count_test::<2>();
    pentagon_count_test::<3>();
    pentagon_count_test::<4>();
}

#[test]
fn hexagon_counts_subdivision() {
    face_creation_test_hexagons::<1>();
    face_creation_test_hexagons::<2>();
    face_creation_test_hexagons::<3>();
    face_creation_test_hexagons::<4>();
}

#[test]
fn pentagon_counts_subdivision() {
    face_creation_test_pentagons::<1>();
    face_creation_test_pentagons::<2>();
    face_creation_test_pentagons::<3>();
    face_creation_test_pentagons::<4>();
}