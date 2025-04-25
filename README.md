Hexglobe
========

[<img alt="github" src="https://img.shields.io/badge/github-morganbennetdev/hexglobe--rs-a?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MorganBennetDev/hexglobe-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/hexglobe?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/hexglobe)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-hexglobe-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/hexglobe)

Library to generate hexagonal tessellations of a sphere (also known as Goldberg polyhedra) quickly and accurately. Every tessellation is the dual of an icosahedron whose faces have been subdivided one or more times. Multipoint spherical linear interpolation (slerp) is used when calculating the centroids of triangles to ensure the resulting faces have as uniform a shape and size as practical.

## Purpose

This library is primarily intended to address performance and precision issues I had using other similar libraries for a personal project. It avoids floating point arithmetic wherever possible since floats are slow and finicky. This library additionally exploits specific aspects of the problem (icosahedral symmetry, subdivisions being constant) to reduce computational complexity in many places.

## Warnings

Since this library was created primarily to support a personal project, there may be some oversights and design quirks which need to be fixed for certain use cases.

Additionally, the documentation is sparser than I'd like, but making it more comprehensive is a low priority task as I'm the only user right now.

## Contributing

I welcome any issues, suggestions, or pull requests. The only caveat is that I would like to keep this project focused on generating (specific kinds of) Goldberg polyhedra quickly and accurately.
