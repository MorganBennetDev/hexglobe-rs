use crate::subdivision::subdivided_triangle::SubdividedTriangle;

fn edge_length_test<const N: u32>() {
    let subdivided = SubdividedTriangle::<N>::new();
    let n_triangles_edge = (2 * N - 1) as usize;
    
    assert_eq!(subdivided.uv().len(), n_triangles_edge, "Incorrect number of triangles touching edge uv for subdivision level {:?}.", N);
    assert_eq!(subdivided.vw().len(), n_triangles_edge, "Incorrect number of triangles touching edge vw for subdivision level {:?}.", N);
    assert_eq!(subdivided.wu().len(), n_triangles_edge, "Incorrect number of triangles touching edge wu for subdivision level {:?}.", N);
}

#[test]
fn subdivision() {
    let test = SubdividedTriangle::<3>::new();
    
    assert_eq!(test.vertices.iter().len(), 10, "Incorrect number of vertices in subdivision.");
    assert_eq!(test.triangles.len(), 9, "Incorrect number of triangles in subdivision.");
    
    assert_eq!(test.vertices.iter().len(), SubdividedTriangle::<3>::N_VERTICES, "Constant N_VERTICES is incorrect.");
    assert_eq!(test.triangles.len(), SubdividedTriangle::<3>::N_TRIANGLES, "Constant N_TRIANGLES is incorrect.");
}

#[test]
fn edge_length() {
    edge_length_test::<1>();
    edge_length_test::<2>();
    edge_length_test::<3>();
    edge_length_test::<4>();
}