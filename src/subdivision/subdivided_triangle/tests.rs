use assert2::check;
use glam::IVec3;
use crate::subdivision::subdivided_triangle::SubdividedTriangle;

fn edge_length_test<const N: u32>() {
    let subdivided = SubdividedTriangle::<N>::new();
    let n_triangles_edge = (2 * N - 1) as usize;
    
    check!(subdivided.uv().len() == n_triangles_edge, "Incorrect number of triangles touching edge uv for subdivision level {:?}.", N);
    check!(subdivided.vw().len() == n_triangles_edge, "Incorrect number of triangles touching edge vw for subdivision level {:?}.", N);
    check!(subdivided.wu().len() == n_triangles_edge, "Incorrect number of triangles touching edge wu for subdivision level {:?}.", N);
}

fn vertex_index_test<const N: u32>(x: i32, y: i32, z: i32) {
    let subdivided = SubdividedTriangle::<N>::new();
    let v = IVec3::new(x, y, z);
    let i = subdivided.vertex_index(v);
    let j = subdivided.vertices.iter().position(|u| u.inner() == v);
    
    check!(i == j, "Incorrect index given for vertex {:?} in triangle with {:?} subdivisions.", v, N);
}

#[test]
fn subdivision() {
    let test = SubdividedTriangle::<3>::new();
    
    check!(test.vertices.iter().len() == 10, "Incorrect number of vertices in subdivision.");
    check!(test.triangles.len() == 9, "Incorrect number of triangles in subdivision.");
    
    check!(test.vertices.iter().len() == SubdividedTriangle::<3>::N_VERTICES, "Constant N_VERTICES is incorrect.");
    check!(test.triangles.len() == SubdividedTriangle::<3>::N_TRIANGLES, "Constant N_TRIANGLES is incorrect.");
}

#[test]
fn edge_length() {
    edge_length_test::<1>();
    edge_length_test::<2>();
    edge_length_test::<3>();
    edge_length_test::<4>();
}

#[test]
fn vertex_index() {
    vertex_index_test::<1>(0, 1, 0);
    vertex_index_test::<2>(1, 1, 0);
    vertex_index_test::<3>(1, 1, 1);
    vertex_index_test::<4>(2, 1, 1);
}