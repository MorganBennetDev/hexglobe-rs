#[cfg(test)]
mod tests;

use std::fmt::Debug;
use glam::IVec3;
use itertools::Itertools;
use crate::denominator::ImplicitDenominator;
use crate::subdivision::triangle::Triangle;

const fn compute_vertex_index_unchecked(n: u32, v: IVec3) -> usize {
    /*
    Vertex list is the subset [0,N]x[0,N]x[0,N] where x + y + z = N
    Disregard z when calculating index since it is determined entirely by x and y.
    */
    let x_offset = (v.x * (2 * (n + 1) as i32 + 1 - v.x) / 2) as usize;
    let y_offset = v.y as usize;
    
    x_offset + y_offset
}

/// Represents a triangle which has been subdivided `N` times using rational barycentric coordinates for precision.
#[derive(Clone, Debug)]
pub struct SubdividedTriangle<const N: u32> {
    pub vertices: Vec<ImplicitDenominator<IVec3, N>>,
    pub triangles: Vec<Triangle<usize>>,
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
        
        let du = ImplicitDenominator::<_, N>::wrap(IVec3::new(-1, 1, 0));
        let dv = ImplicitDenominator::<_, N>::wrap(IVec3::new(-1, 0, 1));
        
        let triangles_up = vertices.iter()
            .filter(|v| v.x > 0 && v.y < N as i32 && v.z < N as i32)
            .map(|v| Triangle::new(
                Self::compute_vertex_index_unchecked(v.inner()),
                Self::compute_vertex_index_unchecked((v + du).inner()),
                Self::compute_vertex_index_unchecked((v + dv).inner())
            ));
        let triangles_down = vertices.iter()
            .filter(|v| v.x < N as i32 && v.y > 0 && v.z > 0)
            .map(|v| Triangle::new(
                Self::compute_vertex_index_unchecked(v.inner()),
                Self::compute_vertex_index_unchecked((v - du).inner()),
                Self::compute_vertex_index_unchecked((v - dv).inner()),
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
            0..0
        } else {
            let k = N as usize - i;
            let start = Self::N_TRIANGLES_UP - k * (k + 1) / 2;
            let end = start + k;
            start..end
        }
    }
    
    fn downward_row(&self, i: usize) -> impl Iterator<Item = usize> {
        if i >= N as usize - 1 {
            0..0
        } else {
            let k = N as usize - 1 - i;
            let start = Self::N_TRIANGLES_UP + Self::N_TRIANGLES_DOWN - k * (k + 1) / 2;
            let end = start + k;
            start..end
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
    
    /// Get the unnormalized barycentric coordinates of the vertex with index `i`.
    pub fn vertex_denominator(&self, i: usize) -> IVec3 {
        self.vertices[i].inner()
    }
    
    /// Index of the `u` vertex of this triangle (`(1,0,0)` in barycentric coordinates).
    pub const fn u(&self) -> usize {
        Self::N_TRIANGLES_UP - 1
    }
    
    /// Index of the `v` vertex of this triangle (`(0,1,0)` in barycentric coordinates).
    pub const fn v(&self) -> usize {
        N as usize - 1
    }
    
    /// Index of the `w` vertex of this triangle (`(0,0,1)` in barycentric coordinates).
    pub const fn w(&self) -> usize {
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
    
    /// Tuples representing undirected edges between vertices in the subdivision.
    pub fn vertex_adjacency(&self) -> impl Iterator<Item = (usize, usize)> {
        self.triangles.iter()
            .flat_map(|t| [
                (t.u, t.v),
                (t.v, t.w),
                (t.w, t.u)
            ])
            .map(|(a, b)| (a.min(b), a.max(b)))
            .unique()
    }
    
    /// Takes in a vertex which lies in the interior of this triangle and returns its index within the interior. Used to
    /// compute offsets in [ExactGlobe::adjacency] for faces that do not lie along edges of the seed polyhedron. Returns
    /// `None` if the vertex lies along an edge and [compute_vertex_interior_index_unchecked] otherwise.
    pub const fn compute_vertex_interior_index(v: IVec3) -> Option<usize> {
        if v.x == 0 || v.y == 0 || v.z == 0 {
            None
        } else {
            Some(Self::compute_vertex_interior_index_unchecked(v))
        }
    }
    
    /// Computes the index of a vertex in the interior of a subdivision without first checking that the vertex is in the
    /// interior. Use only if you can guarantee inputs will be valid, otherwise use [compute_vertex_interior_index],
    /// which will handle input validation for you.
    pub const fn compute_vertex_interior_index_unchecked(v: IVec3) -> usize {
        compute_vertex_index_unchecked(N - 3, IVec3::new(v.x - 1, v.y - 1, v.z - 1))
    }
    
    /// Computes the index of the given vertex in a subdivision's list of vertices. Uses knowledge of how the vertex
    /// list is structured to be faster than searching the [Vec]. Performs input validation to ensure vertex coordinates
    /// are valid. To skip input validation, use [compute_vertex_index_unchecked]. This is a static method so that it
    /// can be used in [new] to eliminate expensive lookups while constructing triangles.
    pub const fn compute_vertex_index(v: IVec3) -> Option<usize> {
        if v.x < 0 || v.y < 0 || v.z < 0 || v.x + v.y + v.z != N as i32 {
            None
        } else {
            Some(Self::compute_vertex_index_unchecked(v))
        }
    }
    
    /// Computes the index of the given vertex in a subdivision's list of vertices without checking if the coordinates
    /// are valid. Use only if you can guarantee inputs will be valid, otherwise use [compute_vertex_index], which will
    /// handle input validation for you.
    pub const fn compute_vertex_index_unchecked(v: IVec3) -> usize {
        compute_vertex_index_unchecked(N, v)
    }
    
    /// Gets the index of the given vertex in this subdivision's list of vertices using [compute_vertex_index].
    pub const fn vertex_index(&self, v: IVec3) -> Option<usize> {
        Self::compute_vertex_index(v)
    }
    
    /// Gets the index of the given vertex in this subdivision's list of vertices using
    /// [compute_vertex_index_unchecked]. Use only if you can guarantee inputs will be valid, otherwise use
    /// [vertex_index], which will handle input validation for you.
    pub const fn vertex_index_unchecked(&self, v: IVec3) -> usize {
        compute_vertex_index_unchecked(N, v)
    }
    
    /// Converts the given vertex index to an interior index using [compute_vertex_interior_index].
    pub const fn vertex_interior_index(&self, v: IVec3) -> Option<usize> {
        Self::compute_vertex_interior_index(v)
    }
    
    /// Converts the given vertex index to an interior index using [compute_vertex_interior_index_unchecked]. Use this
    /// method only if you can guarantee inputs will be valid, otherwise use [vertex_interior_index], which will handle
    /// input validation for you.
    pub const fn vertex_interior_index_unchecked(&self, v: IVec3) -> usize {
        Self::compute_vertex_interior_index_unchecked(v)
    }
}
