#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt::Debug;
use glam::IVec3;
use itertools::Itertools;
use crate::denominator::ImplicitDenominator;
use crate::subdivision::triangle::Triangle;

/// Represents a triangle which has been subdivided `N` times using rational barycentric coordinates for precision.
#[derive(Clone, Debug)]
pub struct SubdividedTriangle<const N: u32> {
    pub vertices: Vec<ImplicitDenominator<IVec3, N>>,
    triangles: Vec<Triangle<usize>>,
}

impl<const N: u32> SubdividedTriangle<N> {
    /// The total number of triangles in the subdivision.
    pub const N_TRIANGLES: usize = (N * N) as usize;
    /// The total number of vertices in the subdivision.
    pub const N_VERTICES: usize = ((N + 1) * (N + 2) / 2) as usize;
    /// The number of "upward pointing" triangles (`u.x > v.x`, `u.x > w.x`, and `v.x = w.x`). Used for optimization
    /// purposes.
    const N_TRIANGLES_UP: usize = (N * (N + 1) / 2) as usize;
    /// The number of "downward pointing" triangles (complement of upward facing set). Used for optimization purposes.
    const N_TRIANGLES_DOWN: usize = (N * (N - 1) / 2) as usize;
    
    pub fn new() -> Self {
        assert_ne!(N, 0, "Number of subdivisions must be nonzero.");
        
        let axis = 0..(N + 1);
        let vertices_iter = axis.clone()
            .cartesian_product(axis.clone())
            .cartesian_product(axis.clone())
            .filter(|((x, y), z)| x + y + z == N)
            .map(|((x, y), z)| ImplicitDenominator::wrap(IVec3::new(x as i32, y as i32, z as i32)));
            
        // Vertices are in ascending lexicographic order
        let vertices = vertices_iter.clone()
            .collect::<Vec<_>>();
        
        let vertices_map = vertices_iter.clone()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect::<HashMap<_, _>>();
        
        let du = ImplicitDenominator::<_, N>::wrap(IVec3::new(-1, 1, 0));
        let dv = ImplicitDenominator::<_, N>::wrap(IVec3::new(-1, 0, 1));
        
        let triangles_up = vertices.iter()
            .filter(|v| v.x > 0 && v.y < N as i32 && v.z < N as i32)
            .map(|v| Triangle::new(
                vertices_map.get(v).unwrap().clone(),
                vertices_map.get(&(v + &du)).unwrap().clone(),
                vertices_map.get(&(v + &dv)).unwrap().clone()
            ));
        let triangles_down = vertices.iter()
            .filter(|v| v.x < N as i32 && v.y > 0 && v.z > 0)
            .map(|v| Triangle::new(
                vertices_map.get(v).unwrap().clone(),
                vertices_map.get(&(v - &du)).unwrap().clone(),
                vertices_map.get(&(v - &dv)).unwrap().clone(),
            ));
        let triangles = triangles_up.chain(triangles_down)
            .collect::<Vec<_>>();
        
        Self {
            vertices,
            triangles,
        }
    }
    
    fn upward_row(&self, i: usize) -> impl Iterator<Item = usize> {
        if i >= N as usize {
            (0..0).rev()
        } else {
            let k = N as usize - i;
            let start = Self::N_TRIANGLES_UP - k * (k + 1) / 2;
            let end = start + k;
            (start..end).rev()
        }
    }
    
    fn downward_row(&self, i: usize) -> impl Iterator<Item = usize> {
        if i >= N as usize - 1 {
            (0..0).rev()
        } else {
            let k = N as usize - 1 - i;
            let start = Self::N_TRIANGLES_UP + Self::N_TRIANGLES_DOWN - k * (k + 1) / 2;
            let end = start + k;
            (start..end).rev()
        }
    }
    
    /// Iterator of indices of triangles with at least one vertex whose `x` coordinate is `i` sorted by increasing `y`
    /// coordinate of their centroids.
    pub fn row(&self, i: usize) -> impl Iterator<Item = usize> {
        self.upward_row(i)
            .interleave(self.downward_row(i))
    }
    
    /// Iterator over all triangles in this subdivision.
    pub fn triangles(&self) -> impl Iterator<Item = Triangle<ImplicitDenominator<IVec3, N>>> {
        self.triangles.iter()
            .map(|t| Triangle::new(
                self.vertices[t.u],
                self.vertices[t.v],
                self.vertices[t.w]
            ))
    }
    
    /// Index of the `u` vertex of this triangle (`(1,0,0)` in barycentric coordinates).
    pub fn u(&self) -> usize {
        Self::N_TRIANGLES_UP - 1
    }
    
    /// Index of the `v` vertex of this triangle (`(0,1,0)` in barycentric coordinates).
    pub fn v(&self) -> usize {
        N as usize - 1
    }
    
    /// Index of the `w` vertex of this triangle (`(0,0,1)` in barycentric coordinates).
    pub fn w(&self) -> usize {
        0
    }
    
    /// Indices of all triangles which have at least one vertex lying along the `uv` edge (`z=0` in barycentric
    /// coordinates) of the parent triangle sorted by descending centroid `x` coordinate.
    pub fn uv(&self) -> Vec<usize> {
        (0..N as usize).into_iter()
            .map(|i| Self::N_TRIANGLES_UP - i * (i + 1) / 2 - 1)
            .interleave(
                (0..(N as usize - 1)).into_iter()
                    .map(|i| Self::N_TRIANGLES - i * (i + 1) / 2 - 1)
            )
            .collect::<Vec<_>>()
    }
    
    /// Indices of all triangles which have at least one vertex lying along the `vw` edge (`x=0` in barycentric
    /// coordinates) of the parent triangle sorted by descending centroid `y` coordinate.
    pub fn vw(&self) -> Vec<usize> {
        self.row(0).collect::<Vec<_>>()
    }
    
    /// Indices of all triangles which have at least one vertex lying along the `wu` edge (`y=0` in barycentric
    /// coordinates) of the parent triangle sorted by descending centroid `z` coordinate.
    pub fn wu(&self) -> Vec<usize> {
        let k = (0..N as usize).rev()
            .map(|i| (i + 1) * (i + 2) / 2);
        
        k.clone()
            .map(|k_i| Self::N_TRIANGLES_UP - k_i)
            .interleave(
                k.clone()
                    .skip(1)
                    .map(|k_i| Self::N_TRIANGLES - k_i)
            )
            .collect::<Vec<_>>()
    }
    
    /// Tuples representing undirected edges between triangles in the subdivision. Exploits the way triangles are
    /// ordered for efficient computation.
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
