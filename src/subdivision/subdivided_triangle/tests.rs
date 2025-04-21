use std::collections::HashSet;
use assert2::check;
use glam::IVec3;
use itertools::Itertools;
use crate::subdivision::subdivided_triangle::SubdividedTriangle;
use crate::subdivision::triangle::Triangle;

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

// Const parameter `M` should always be `N-3`. If only generic const expressions were stable :(.
fn vertex_interior_index_test<const N: u32, const M: u32>(x: i32, y: i32, z: i32) {
    let tri = SubdividedTriangle::<N>::new();
    let sub_tri = SubdividedTriangle::<M>::new();
    let expected = sub_tri.vertex_index(IVec3::new(x - 1, y - 1, z - 1));
    let actual = tri.vertex_index(IVec3::new(x, y, z));
    
    check!(actual == expected, "Incorrect interior index given for vertex {:?} in triangle with {:?} subdivisions.", IVec3::new(x, y, z), N);
}

fn vertex_adjacency_test<const N: u32>() {
    let tri = SubdividedTriangle::<N>::new();
    let actual_adjacency = tri.vertex_adjacency().collect_vec();
    
    let untrue_edge = actual_adjacency.iter()
        .find(|(i, j)| (tri.vertices[*i] - tri.vertices[*j]).abs().element_sum() != 2);
    
    check!(untrue_edge == None, "Returned false edge ({:?}, {:?}) for triangle with {:?} subdivisions.",
        tri.vertices[untrue_edge.unwrap().0].inner(), tri.vertices[untrue_edge.unwrap().1].inner(),
        N);
    
    let expected_adjacency = tri.triangles.iter()
        .flat_map(|t| [
            (t.u, t.v),
            (t.v, t.w),
            (t.w, t.u)
        ])
        .map(|(a, b)| (a.min(b), a.max(b)))
        .collect::<HashSet<_>>();
    
    let actual_adjacency_set = actual_adjacency.iter()
        .cloned()
        .collect::<HashSet<_>>();
    let difference = actual_adjacency_set
        .symmetric_difference(&expected_adjacency)
        .collect::<HashSet<_>>();
    
    check!(difference == HashSet::new(), "Found edges in triangle with {:?} subdivisions that were not returned by adjacency method.", N);
}

fn row_test<const N: u32>(i: usize) {
    let tri = SubdividedTriangle::<N>::new();
    let row = tri.row(i).collect_vec();
    let row_tris = row.iter()
        .map(|i| tri.triangles[*i].clone())
        .collect_vec();
    let row_tris_verts = row_tris.iter()
        .map(|t| Triangle::new(
            tri.vertices[t.u].inner(),
            tri.vertices[t.v].inner(),
            tri.vertices[t.w].inner()
        ))
        .collect_vec();
    let x = i as i32;
    
    let wrong_coordinate = row_tris_verts.iter()
        .find(|t| t.u.x != x && t.v.x != x && t.w.x != x);
    
    check!(wrong_coordinate == None, "Incorrect calculation of row {:?} for triangle with {:?} subdivisions.", i, N);
    
    let unscaled_centroids = row_tris_verts.iter()
        .map(|t| t.u + t.v + t.w)
        .collect_vec();
    let is_sorted = unscaled_centroids.iter()
        .map(|c| c.y)
        .is_sorted();
    
    check!(is_sorted, "Incorrect sorting of row {:?} for triangle with {:?} subdivisions.\nUnscaled centroids {:?}", i, N, unscaled_centroids);
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
    
    vertex_index_test::<2>(0, 1, 0);
    vertex_index_test::<2>(1, 0, 0);
    vertex_index_test::<2>(0, 0, 1);
    vertex_index_test::<2>(1, 1, 0);
    
    vertex_index_test::<3>(1, 1, 1);
    
    vertex_index_test::<4>(2, 1, 1);
}

#[test]
fn vertex_interior_index() {
    vertex_index_test::<1>(0, 1, 0);
    
    vertex_index_test::<2>(0, 1, 0);
    
    vertex_index_test::<3>(1, 0, 0);
    vertex_index_test::<3>(0, 0, 1);
    vertex_index_test::<3>(1, 1, 0);
    
    vertex_index_test::<4>(1, 1, 1);
    vertex_index_test::<4>(2, 1, 1);
}

#[test]
fn vertex_adjacency() {
    vertex_adjacency_test::<1>();
    
    vertex_adjacency_test::<2>();
    
    vertex_adjacency_test::<3>();
    
    vertex_adjacency_test::<4>();
    
    vertex_adjacency_test::<5>();
}

#[test]
fn row() {
    row_test::<1>(0);
    
    row_test::<2>(0);
    row_test::<2>(1);
    
    row_test::<3>(0);
    row_test::<3>(1);
    row_test::<3>(3);
    
    row_test::<4>(0);
    row_test::<4>(1);
    row_test::<4>(3);
    row_test::<4>(4);
    
    row_test::<5>(0);
    row_test::<5>(1);
    row_test::<5>(3);
    row_test::<5>(4);
    row_test::<5>(5);
}