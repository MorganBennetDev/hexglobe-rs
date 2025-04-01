use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::rc::Rc;
use itertools::Itertools;
use petgraph::graph::UnGraph;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, Debug)]
struct RationalVector<const N: usize> {
    x: i32,
    y: i32,
    z: i32
}

impl<const N: usize> RationalVector<N> {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl<const N: usize> Add for RationalVector<N> {
    type Output = RationalVector<N>;
    
    fn add(self, rhs: Self) -> Self::Output {
        RationalVector::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl<const N: usize> Sub for RationalVector<N> {
    type Output = RationalVector<N>;
    
    fn sub(self, rhs: Self) -> Self::Output {
        RationalVector::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl<const N: usize> PartialOrd for RationalVector<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y && self.z == other.z {
            Some(Ordering::Equal)
        } else {
            let gt_other = self.x > other.x ||
                (self.x == other.x && (self.y > other.y ||
                    (self.y == other.y && self.z > other.z)));
            
            if gt_other {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Triangle {
    u: Rc<RationalVector<3>>,
    v: Rc<RationalVector<3>>,
    w: Rc<RationalVector<3>>
}

#[derive(Debug)]
pub struct SubdividedTriangle<const N: usize> {
    vertices: HashMap<RationalVector<3>, Rc<RationalVector<3>>>,
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
            .map(|((x, y), z)| RationalVector { x: x as i32, y: y as i32, z: z as i32 })
            .map(|v| (v, Rc::new(v)))
            .collect::<HashMap<_, _>>();
        
        let du = RationalVector::new(-1, 1, 0);
        let dv = RationalVector::new(-1, 0, 1);
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
        
        let adjacency = UnGraph::from_edges(
            triangles.iter()
                .enumerate()
                .tuple_combinations()
                .filter(|((_, t_i), (_, t_j))| HashSet::from([
                    t_i.u.clone(), t_i.v.clone(), t_i.w.clone(),
                    t_j.u.clone(), t_j.v.clone(), t_j.w.clone()
                ]).iter().count() == 2)
                .map(|((i, _), (j, _))| (i as u32, j as u32))
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
