#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use glam::IVec3;
use itertools::Itertools;
use crate::denominator::ImplicitDenominator;
use crate::subdivision::triangle::Triangle;

#[derive(Clone, Debug)]
pub struct SubdividedTriangle<const N: u32> {
    pub vertices: HashMap<ImplicitDenominator<IVec3, N>, Rc<ImplicitDenominator<IVec3, N>>>,
    pub triangles: Vec<Rc<Triangle<ImplicitDenominator<IVec3, N>>>>,
}

impl<const N: u32> SubdividedTriangle<N> {
    pub const N_TRIANGLES: usize = (N * N) as usize;
    pub const N_VERTICES: usize = ((N + 1) * (N + 2) / 2) as usize;
    const N_TRIANGLES_UP: usize = (N * (N + 1) / 2) as usize;
    
    pub fn new() -> Self {
        assert_ne!(N, 0, "Number of subdivisions must be nonzero.");
        
        let axis = 0..(N + 1);
        // Vertices are in ascending lexicographic order
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
                vertices.get(&ImplicitDenominator::wrap(v.0 + du)).unwrap().clone(),
                vertices.get(&ImplicitDenominator::wrap(v.0 + dv)).unwrap().clone()
            ));
        let triangles_down = vertices.keys()
            .filter(|v| v.x < N as i32 && v.y > 0 && v.z > 0)
            .map(|v| Triangle::new(
                vertices.get(v).unwrap().clone(),
                vertices.get(&ImplicitDenominator::wrap(v.0 - du)).unwrap().clone(),
                vertices.get(&ImplicitDenominator::wrap(v.0 - dv)).unwrap().clone(),
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
    
    pub fn upward_triangle_indices(&self) -> impl Iterator<Item = usize> {
        0..Self::N_TRIANGLES_UP
    }
    
    pub fn downward_triangles(&self) -> &[Rc<Triangle<ImplicitDenominator<IVec3, N>>>] {
        &self.triangles[Self::N_TRIANGLES_UP..Self::N_TRIANGLES]
    }
    
    pub fn downward_triangle_indices(&self) -> impl Iterator<Item = usize> {
        Self::N_TRIANGLES_UP..Self::N_TRIANGLES
    }
    
    // Iterator of indices of triangles with at least one vertex whose x coordinate is x sorted by increasing y
    pub fn level_x(&self, x: ImplicitDenominator<u32, N>) -> impl Iterator<Item = usize> {
        let length = (N - x.0) as usize;
        let start = Self::N_TRIANGLES_UP + 1 - length * (length + 1) / 2;
        
        (start..(start + length + 1)).interleave(
            (start..(start + length)).map(move |i| i + Self::N_TRIANGLES_UP + 1 - x.0 as usize)
        )
    }
    
    pub fn u(&self) -> usize {
        self.upward_triangles().iter()
            .position(|t| t.u.y == 0 && t.u.z == 0)
            .unwrap()
    }
    
    pub fn v(&self) -> usize {
        self.upward_triangles().iter()
            .position(|t| t.v.x == 0 && t.v.z == 0)
            .unwrap()
    }
    
    pub fn w(&self) -> usize {
        self.upward_triangles().iter()
            .position(|t| t.w.x == 0 && t.w.y == 0)
            .unwrap()
    }
    
    pub fn uv(&self) -> impl DoubleEndedIterator<Item = usize> {
        self.upward_triangles().iter()
            .enumerate()
            .filter(|(_, t)| t.u.z == 0)
            .sorted_by_key(|(_, t)| t.u.y)
            .map(|(i, _)| i)
            .interleave(
                self.downward_triangles().iter()
                    .enumerate()
                    .filter(|(_, t)| t.w.z == 0)
                    .sorted_by_key(|(_, t)| t.u.y)
                    .map(|(i, _)| i + Self::N_TRIANGLES_UP)
            )
            .collect::<Vec<_>>()
            .into_iter()
    }
    
    pub fn vw(&self) -> impl DoubleEndedIterator<Item = usize> {
        self.upward_triangles().iter()
            .enumerate()
            .filter(|(_, t)| t.v.x == 0)
            .sorted_by_key(|(_, t)| t.v.z)
            .map(|(i, _)| i)
            .interleave(
                self.downward_triangles().iter()
                    .enumerate()
                    .filter(|(_, t)| t.u.x == 0)
                    .sorted_by_key(|(_, t)| t.v.z)
                    .map(|(i, _)| i + Self::N_TRIANGLES_UP)
            )
            .collect::<Vec<_>>()
            .into_iter()
    }
    
    pub fn wu(&self) -> impl DoubleEndedIterator<Item = usize> {
        self.upward_triangles().iter()
            .enumerate()
            .filter(|(_, t)| t.u.y == 0)
            .sorted_by_key(|(_, t)| t.w.x)
            .map(|(i, _)| i)
            .interleave(
                self.downward_triangles().iter()
                    .enumerate()
                    .filter(|(_, t)| t.v.y == 0)
                    .sorted_by_key(|(_, t)| t.w.x)
                    .map(|(i, _)| i + Self::N_TRIANGLES_UP)
            )
            .collect::<Vec<_>>()
            .into_iter()
    }
    
    // Tuples representing undirected edges between triangles in the subdivision. Exploits the way triangles are
    // ordered for efficient computation.
    pub fn adjacency(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..((N - 1) as usize)).flat_map(|i|
            (0..(N as usize - 1 - i))
                .map(move |j| Self::N_TRIANGLES_UP + i + j)
                .flat_map(move |j| [
                    (j, j - (10 - i)),
                    (j, j - (9 - i)),
                    (j, j - 6)
                ])
        )
    }
}
