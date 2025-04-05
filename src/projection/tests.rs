use crate::projection::globe::{ExactFace, Globe};

// Number of vertices, edges, and faces of icosahedron.
const V: usize = 12;
const E: usize = 30;
const F: usize = 20;

#[test]
fn basic_counts() {
    let globe = Globe::<2>::new();
    
    assert_eq!(globe.vertices.len(), F * 4, "Incorrect number of vertices in icosahedron with 2 subdivisions.");
    assert_eq!(globe.faces.len(), V + E, "Incorrect number of faces in icosahedron with 2 subdivisions.");
}

#[test]
fn hexagon_counts() {
    let globe = Globe::<2>::new();
    
    let (hexagons, _): (Vec<_>, Vec<_>) = globe.faces.iter()
        .partition(|f| match f {
            ExactFace::Hexagon(_) => true,
            ExactFace::Pentagon(_) => false
        });
    assert_eq!(hexagons.len(), E, "Incorrect number of hexagons in icosahedron with 2 subdivisions.");
}

#[test]
fn pentagon_counts() {
    let globe = Globe::<2>::new();
    
    let (_, pentagons): (Vec<_>, Vec<_>) = globe.faces.iter()
        .partition(|f| match f {
            ExactFace::Hexagon(_) => true,
            ExactFace::Pentagon(_) => false
        });
    assert_eq!(pentagons.len(), V, "Incorrect number of pentagons in icosahedron with 2 subdivisions.");
}