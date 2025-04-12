# Hexglobe

Library to generate hexagonal tessellations of a sphere (also known as Goldberg polyhedra) quickly and accurately. Every tessellation is the dual of an icosahedron whose faces have been subdivided one or more times. Multipoint spherical linear interpolation (slerp) is used when calculating the centroids of triangles to ensure the resulting faces have as uniform a shape and size as practical.

## Purpose

This library is primarily intended to address performance and precision issues I had using other similar libraries for a personal project. It avoids floating point arithmetic wherever possible since floats are slow and finicky. This library additionally exploits specific aspects of the problem (icosahedral symmetry, subdivisions being constant) to reduce computational complexity in many places.

## Warnings

Since this library was created primarily to support a personal project, there may be some oversights and design quirks which need to be fixed for certain use cases.

Additionally, the documentation is sparser than I'd like, but making it more comprehensive is a low priority task as I'm the only user right now.

## Contributing

I welcome any issues, suggestions, or pull requests. The only caveat is that I would like to keep this project focused on generating (specific kinds of) Goldberg polyhedra quickly and accurately.
