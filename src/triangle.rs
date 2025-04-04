use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;
use glam::IVec3;
use itertools::Itertools;
use crate::denominator::ImplicitDenominator;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Triangle<T: Clone> {
    pub u: Rc<T>,
    pub v: Rc<T>,
    pub w: Rc<T>
}

impl<T: Clone> Triangle<T> {
    pub const fn new(u: Rc<T>, v: Rc<T>, w: Rc<T>) -> Self {
        Self { u, v, w }
    }
}

#[derive(Clone, Debug)]
pub struct SubdividedTriangle<const N: u32> {
    pub vertices: HashMap<ImplicitDenominator<IVec3, N>, Rc<ImplicitDenominator<IVec3, N>>>,
    pub triangles: Vec<Rc<Triangle<ImplicitDenominator<IVec3, N>>>>,
}

impl<const N: u32> SubdividedTriangle<N> {
    pub const N_TRIANGLES: usize = (N * N) as usize;
    pub const N_VERTICES: usize = (N * (N + 1) / 2) as usize;
    const N_TRIANGLES_UP: usize = (N * (N + 1) / 2) as usize;
    
    pub fn new() -> Self {
        assert_ne!(N, 0, "Number of subdivisions must be nonzero.");
        
        let axis = 0..(N + 1);
        let vertices = axis.clone()
            .cartesian_product(axis.clone())
            .cartesian_product(axis.clone())
            .filter(|((x, y), z)| x + y + z == N)
            .map(|((x, y), z)| ImplicitDenominator::wrap(IVec3::new(x as i32, y as i32, z as i32)))
            .map(|v| (v.clone(), Rc::new(v)))
            .collect::<HashMap<_, _>>();
        
        let du = IVec3::new(-1, 1, 0);
        let dv = IVec3::new(-1, 0, 1);
        let triangles_up = vertices.keys()
            .filter(|v| v.x > 0 && v.y < N as i32 && v.z < N as i32)
            .map(|v| Triangle::new(
                vertices.get(v).unwrap().clone(),
                vertices.get(&ImplicitDenominator::wrap(*v.deref() + du)).unwrap().clone(),
                vertices.get(&ImplicitDenominator::wrap(*v.deref() + dv)).unwrap().clone()
            ));
        let triangles_down = vertices.keys()
            .filter(|v| v.x < N as i32 && v.y > 0 && v.z > 0)
            .map(|v| Triangle::new(
                vertices.get(&ImplicitDenominator::wrap(*v.deref() - dv)).unwrap().clone(),
                vertices.get(&ImplicitDenominator::wrap(*v.deref() - du)).unwrap().clone(),
                vertices.get(v).unwrap().clone()
            ));
        let triangles = triangles_up.chain(triangles_down)
            .map(|t| Rc::new(t))
            .collect::<Vec<_>>();
        
        Self {
            vertices,
            triangles,
        }
    }
    
    pub fn upward_triangles(&self) -> &[Rc<Triangle<ImplicitDenominator<IVec3, N>>>] {
        &self.triangles[0..Self::N_TRIANGLES_UP]
    }
    
    pub fn downward_triangles(&self) -> &[Rc<Triangle<ImplicitDenominator<IVec3, N>>>] {
        &self.triangles[Self::N_TRIANGLES_UP..Self::N_TRIANGLES]
    }
}
