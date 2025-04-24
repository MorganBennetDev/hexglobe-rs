use std::collections::hash_set::Difference;
use std::collections::HashSet;
use std::hash::RandomState;
use assert2::check;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use glam::Vec3;
use itertools::Itertools;
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
    
    check!(edge_faces == n_edge_faces, "Incorrect number of hexagons crossing edges for icosahedron with {:?} subdivisions.", N);
    check!(face_faces == n_face_faces, "Incorrect number of hexagons within faces for icosahedron with {:?} subdivisions.", N);
    
    let pentagon = ExactGlobe::<N>::edge_faces_from_template(&template).position(|v| match v {
        ExactFace::Pentagon(_) => true,
        _ => false
    });
    
    check!(pentagon == None, "Found pentagon crossing edge for icosahedron with {:?} subdivisions.", N);
    
    let pentagon = ExactGlobe::<N>::face_faces_from_template(&template).position(|v| match v {
        ExactFace::Pentagon(_) => true,
        _ => false
    });
    
    check!(pentagon == None, "Found pentagon within face for icosahedron with {:?} subdivisions.", N);
}

fn face_creation_test_pentagons<const N: u32>() {
    let template = SubdividedTriangle::<N>::new();
    let vertex_faces = ExactGlobe::<N>::vertex_faces_from_template(&template).count();
    
    let n_vertex_faces = V;
    
    check!(vertex_faces == n_vertex_faces, "Incorrect number of pentagons on vertices for icosahedron with {:?} subdivisions.", N);
    
    let hexagon = ExactGlobe::<N>::vertex_faces_from_template(&template).position(|v| match v {
        ExactFace::Hexagon(_) => true,
        _ => false
    });
    
    check!(hexagon == None, "Found hexagonal face lying on a vertex for icosahedron with {:?} subdivisions.", N);
}

fn basic_count_test<const N: u32>() {
    let globe = ExactGlobe::<N>::new();
    let n_vertices_expected = F * (N * N) as usize;
    let n_faces = V + E * (N - 1) as usize + F * ((N - 1) * (N.max(2) - 2) / 2) as usize;
    let n_vertices = globe.vertices_f32(None).iter().flatten().count();
    
    check!(n_vertices == n_vertices_expected, "Incorrect number of vertices in icosahedron with {:?} subdivisions.", N);
    check!(globe.faces.len() == n_faces, "Incorrect number of faces in icosahedron with {:?} subdivisions.", N);
}

fn hexagon_count_test<const N: u32>() {
    let globe = ExactGlobe::<N>::new();
    let expected = E * (N - 1) as usize + F * ((N - 1) * (N.max(2) - 2) / 2) as usize;
    
    let hexagons = globe.faces.iter()
        .filter(|f| match f {
            ExactFace::Hexagon(_) => true,
            ExactFace::Pentagon(_) => false
        })
        .collect_vec();
    check!(hexagons.len() == expected, "Incorrect number of hexagons in icosahedron with {:?} subdivisions.", N);
}

fn pentagon_count_test<const N: u32>() {
    let globe = ExactGlobe::<N>::new();
    
    let pentagons = globe.faces.iter()
        .filter(|f| match f {
            ExactFace::Hexagon(_) => false,
            ExactFace::Pentagon(_) => true
        })
        .collect_vec();
    check!(pentagons.len() == V, "Incorrect number of pentagons in icosahedron with {:?} subdivisions.", N);
}

fn difference_to_vec(d: &Difference<(usize, usize), RandomState>) -> Vec<(usize, usize)> {
    d.clone()
        .sorted_by_key(|(a, b)| b * 1000 + a)
        .cloned()
        .collect::<Vec<_>>()
}

fn adjacency_test<const N: u32>() {
    let globe = ExactGlobe::<N>::new();
    let computed_adjacency = globe.adjacency().into_iter()
        .map(|(a, b)| (a.min(b), a.max(b)))
        .collect::<HashSet<_>>();
    let expected_adjacency = globe.faces.iter()
        .enumerate()
        .flat_map(|(i, f)| match f {
            ExactFace::Pentagon(v) => vec![
                (i, (v[0], v[1])),
                (i, (v[1], v[2])),
                (i, (v[2], v[3])),
                (i, (v[3], v[4])),
                (i, (v[4], v[0]))
            ],
            ExactFace::Hexagon(v) => vec![
                (i, (v[0], v[1])),
                (i, (v[1], v[2])),
                (i, (v[2], v[3])),
                (i, (v[3], v[4])),
                (i, (v[4], v[5])),
                (i, (v[5], v[0]))
            ]
        })
        .map(|(i, (a, b))| (i, (a.min(b), a.max(b))))
        .unique()
        .tuple_combinations::<(_, _)>()
        .filter_map(|((i, (a, b)), (j, (c, d)))| if (a == c && b == d) || (a == d && b == c) {
            Some((i, j))
        } else {
            None
        })
        .map(|(a, b)| (a.min(b), a.max(b)))
        .collect::<HashSet<_>>();
    
    check!(computed_adjacency.is_subset(&expected_adjacency),
        "Not all computed adjacencies are real for globe with {:?} subdivisions.\nComputed {:?}\nActual {:?}\nComputed - Actual {:?}\nActual - Computed {:?}",
        N,
        computed_adjacency.iter().count(),
        expected_adjacency.iter().count(),
        difference_to_vec(&computed_adjacency.difference(&expected_adjacency)), difference_to_vec(&expected_adjacency.difference(&computed_adjacency)));
    check!(expected_adjacency.is_subset(&computed_adjacency),
        "Not all adjacencies are computed for globe with {:?} subdivisions.\nComputed {:?}\nActual {:?}\nComputed - Actual {:?}\nActual - Computed {:?}",
        N,
        computed_adjacency.iter().count(),
        expected_adjacency.iter().count(),
        difference_to_vec(&computed_adjacency.difference(&expected_adjacency)), difference_to_vec(&expected_adjacency.difference(&computed_adjacency)));
}

fn vertex_index_to_face_index_test<const N: u32>(f: usize, i: usize) {
    let globe = ExactGlobe::<N>::new();
    
    let actual = globe.vertex_index_to_face_index(f, i);
    let actual_face = globe.faces[actual];
    let matches = match actual_face {
        ExactFace::Pentagon(v) => v.into_iter().collect::<Vec<_>>(),
        ExactFace::Hexagon(v) => v.into_iter().collect::<Vec<_>>()
    }.iter()
        .map(|v| globe.subdivision.triangles[v.subdivision()].clone())
        .position(|t| t.u == i || t.v == i || t.w == i);
    
    check!(matches.is_some(), "Did not get expected face index for globe with {:?} subdivisions.", N);
}

fn normal_test<const N: u32>() {
    let globe = ExactGlobe::<N>::new();
    let vertices = globe.mesh_vertices(None);
    let faces = globe.mesh_faces();
    let normals = globe.mesh_normals(&vertices);
    let triangles = globe.mesh_triangles(&faces);
    
    triangles.iter()
        .chunks(3)
        .into_iter()
        .map(|i| i.map(|j| (*j as usize, Vec3::from_array(vertices[*j as usize]))).collect_vec())
        .for_each(|t| {
            let ((i, u), (j, v), (k, w)) = (t[0], t[1], t[2]);
            
            let uv = v - u;
            let vw = w - v;
            let wu = u - w;
            
            let n_u = Vec3::from_array(normals[i]);
            let n_v = Vec3::from_array(normals[j]);
            let n_w = Vec3::from_array(normals[k]);
            // Allow a maximum error of +/-0.573 degrees in angle between normal and face.
            let e = 0.01;
            
            check!(n_u.dot(uv) <= e, "Found invalid normal for u, uv in globe with {:?} subdivisions (index {:?}).", N, i);
            check!(n_u.dot(-wu) <= e, "Found invalid normal for u, wu in globe with {:?} subdivisions (index {:?}).", N, i);
            check!(n_v.dot(-uv) <= e, "Found invalid normal for v, uv in globe with {:?} subdivisions (index {:?}).", N, j);
            check!(n_v.dot(vw) <= e, "Found invalid normal for v, vw in globe with {:?} subdivisions (index {:?}).", N, j);
            check!(n_w.dot(-vw) <= e, "Found invalid normal for w, vw in globe with {:?} subdivisions (index {:?}).", N, k);
            check!(n_w.dot(wu) <= e, "Found invalid normal for w, wu in globe with {:?} subdivisions (index {:?}).", N, k);
        });
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

#[test]
fn adjacency() {
    adjacency_test::<1>();
    
    adjacency_test::<2>();
    
    adjacency_test::<3>();
    
    adjacency_test::<4>();
    
    adjacency_test::<5>();
}

#[test]
fn vertex_to_face_index() {
    // Edge
    vertex_index_to_face_index_test::<2>(1, 2);
    // Interior
    vertex_index_to_face_index_test::<3>(11, 5);
    
    vertex_index_to_face_index_test::<4>(0, 6);
    vertex_index_to_face_index_test::<4>(1, 7);
    vertex_index_to_face_index_test::<4>(2, 10);
    vertex_index_to_face_index_test::<4>(1, 1)
}

#[test]
fn normal() {
    normal_test::<1>();
    
    normal_test::<2>();
    
    normal_test::<3>();
    
    normal_test::<4>();
    
    normal_test::<5>();
}
