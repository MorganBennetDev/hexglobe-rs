use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use glam::IVec3;
use itertools::Itertools;
use petgraph::graph::UnGraph;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Triangle {
    u: Rc<IVec3>,
    v: Rc<IVec3>,
    w: Rc<IVec3>
}

#[derive(Debug)]
pub struct SubdividedTriangle<const N: usize> {
    vertices: HashMap<IVec3, Rc<IVec3>>,
    triangles: Vec<Rc<Triangle>>,
    adjacency: UnGraph<u32, ()>
}

impl<const N: usize> SubdividedTriangle<N> {
    pub fn new() -> Self {
        assert_ne!(N, 0, "Number of subdivisions must be nonzero.");
        
        let axis = 0..(N + 1);
        let vertices = axis.clone()
            .cartesian_product(axis.clone())
            .cartesian_product(axis.clone())
            .filter(|((x, y), z)| x + y + z == N)
            .map(|((x, y), z)| IVec3::new(x as i32, y as i32, z as i32))
            .map(|v| (v, Rc::new(v)))
            .collect::<HashMap<_, _>>();
        
        let du = IVec3::new(-1, 1, 0);
        let dv = IVec3::new(-1, 0, 1);
        let triangles_up = vertices.keys()
            .filter(|v| v.x > 0 && v.y < N as i32 && v.z < N as i32)
            .map(|v| Triangle {
                u: vertices.get(&(*v + du)).unwrap().clone(),
                v: vertices.get(&(*v + dv)).unwrap().clone(),
                w: vertices.get(v).unwrap().clone()
            });
        let triangles_down = vertices.keys()
            .filter(|v| v.x < N as i32 && v.y > 0 && v.z > 0)
            .map(|v| Triangle {
                u: vertices.get(&(*v - du)).unwrap().clone(),
                v: vertices.get(&(*v - dv)).unwrap().clone(),
                w: vertices.get(v).unwrap().clone()
            });
        let triangles = triangles_up.chain(triangles_down)
            .map(|t| Rc::new(t))
            .collect::<Vec<_>>();
        
        let n_triangles_up = N * (N + 1) / 2;
        
        let adjacency = UnGraph::from_edges(
            triangles[0..n_triangles_up].iter()
                .enumerate()
                .cartesian_product(
                    triangles[n_triangles_up..triangles.len()].iter()
                        .enumerate()
                )
                .filter(|((_, t_i), (_, t_j))|
                    t_i.v == t_j.v && t_i.w == t_j.w ||
                    t_i.u == t_j.v && t_i.w == t_j.u ||
                    t_i.u == t_j.w && t_i.v == t_j.u
                )
                .map(|((i, _), (j, _))| (i as u32, j as u32 + n_triangles_up as u32))
                .collect::<Vec<(_, _)>>()
        );
        
        Self {
            vertices,
            triangles,
            adjacency
        }
    }
    
    pub fn vertex_count(&self) -> usize {
        self.vertices.iter().count()
    }
    
    pub fn triangle_count(&self) -> usize {
        self.triangles.iter().count()
    }
}
