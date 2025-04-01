use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::rc::Rc;
use itertools::Itertools;
use num_rational::Rational32;
use petgraph::graph::UnGraph;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, Debug)]
struct RationalVector {
    x: Rational32,
    y: Rational32,
    z: Rational32
}

impl RationalVector {
    const fn new(x: Rational32, y: Rational32, z: Rational32) -> Self {
        Self { x, y, z }
    }
}

impl Add for RationalVector {
    type Output = RationalVector;
    
    fn add(self, rhs: Self) -> Self::Output {
        RationalVector::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl Sub for RationalVector {
    type Output = RationalVector;
    
    fn sub(self, rhs: Self) -> Self::Output {
        RationalVector::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl PartialOrd for RationalVector {
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
    u: Rc<RationalVector>,
    v: Rc<RationalVector>,
    w: Rc<RationalVector>
}

#[derive(Debug)]
pub struct SubdividedTriangle<const N: usize> {
    vertices: HashMap<RationalVector, Rc<RationalVector>>,
    triangles: Vec<Rc<Triangle>>,
    adjacency: UnGraph<u32, ()>
}

impl<const N: usize> SubdividedTriangle<N> {
    pub fn new() -> Self {
        let axis = (0..(N+1)).map(|i| Rational32::new(i as i32, N as i32));
        let vertices = axis.clone()
            .cartesian_product(axis.clone())
            .cartesian_product(axis.clone())
            .filter(|((x, y), z)| x + y + z == Rational32::ONE)
            .map(|((x, y), z)| RationalVector { x, y, z })
            .map(|v| (v, Rc::new(v)))
            .unique()
            .collect::<HashMap<_, _>>();
        
        let d = Rational32::new(1, N as i32);
        let du = RationalVector::new(
            -d,
            d,
            Rational32::ZERO
        );
        let dv = RationalVector::new(
            -d,
            Rational32::ZERO,
            d
        );
        let triangles_up = vertices.keys()
            .filter(|v| v.x > Rational32::ZERO && v.y < Rational32::ONE && v.z < Rational32::ONE)
            .map(|v| Triangle {
                u: vertices.get(&(*v + du)).unwrap().clone(),
                v: vertices.get(&(*v + dv)).unwrap().clone(),
                w: vertices.get(v).unwrap().clone()
            });
        let triangles_down = vertices.keys()
            .filter(|v| v.x < Rational32::ONE && v.y > Rational32::ZERO && v.z > Rational32::ZERO)
            .map(|v| Triangle {
                u: vertices.get(&(*v - du)).unwrap().clone(),
                v: vertices.get(&(*v - dv)).unwrap().clone(),
                w: vertices.get(v).unwrap().clone()
            });
        let triangles = triangles_up.chain(triangles_down)
            .map(|t| Rc::new(t))
            .collect::<Vec<_>>();
        
        let mut adjacency = UnGraph::from_edges(
            triangles.iter()
                .enumerate()
                .tuple_combinations()
                .filter(|((i, t_i), (j, t_j))| HashSet::from([
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
