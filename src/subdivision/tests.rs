use crate::subdivision::triangle::SubdividedTriangle;

#[test]
fn subdivision() {
    let test = SubdividedTriangle::<3>::new();
    
    assert_eq!(test.vertices.iter().len(), 10, "Incorrect number of vertices in subdivision.");
    assert_eq!(test.triangles.len(), 9, "Incorrect number of triangles in subdivision.");
    
    assert_eq!(test.vertices.iter().len(), SubdividedTriangle::<3>::N_VERTICES, "Constant N_VERTICES is incorrect.");
    assert_eq!(test.triangles.len(), SubdividedTriangle::<3>::N_TRIANGLES, "Constant N_TRIANGLES is incorrect.");
}