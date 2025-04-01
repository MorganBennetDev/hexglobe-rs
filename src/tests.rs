use crate::triangle::SubdividedTriangle;

#[test]
fn subdivision() {
    let test = SubdividedTriangle::<3>::new();
    
    assert_eq!(test.vertex_count(), 10, "Incorrect number of vertices in subdivision.");
    assert_eq!(test.triangle_count(), 9, "Incorrect number of triangles in subdivision.");
}
