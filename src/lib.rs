#![feature(generic_const_exprs)]
#[cfg(test)]
mod tests;
pub mod fixed;
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// struct BarycentricVector {
//     x: Rational32,
//     y: Rational32,
//     z: Rational32
// }
//
// #[derive(Clone, Debug)]
// struct BarycentricMesh {
//     vertices: Vec<BarycentricVector>,
//     triangles: Vec<[usize; 3]>
// }
//
// impl BarycentricMesh {
//     fn subdivide(&mut self) {
//         // BarycentricTriangle {
//         //     u: self.u,
//         //     v: (self.u + self.v) / 2,
//         //     w: (self.u + self.w) / 2
//         // },
//         // BarycentricTriangle {
//         //     u: (self.v + self.u) / 2,
//         //     v: self.v,
//         //     w: (self.v + self.w) / 2
//         // },
//         // BarycentricTriangle {
//         //     u: (self.w + self.u) / 2,
//         //     v: (self.w + self.v) / 2,
//         //     w: self.w
//         // },
//         // BarycentricTriangle {
//         //     u: (self.u + self.v) / 2,
//         //     v: (self.v + self.w) / 2,
//         //     w: (self.w + self.u) / 2
//         // }
//         self.triangles.append(
//             self.triangles.iter()
//                 .flat_map()
//         )
//     }
// }
