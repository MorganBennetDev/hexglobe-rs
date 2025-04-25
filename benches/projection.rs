use std::hint::black_box;
use divan::Bencher;
use hexglobe::HexGlobe;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(consts = [1, 2, 4, 8, 16, 32, 64])]
fn projection<const N: u32>() {
    let g = HexGlobe::<N>::new();
    
    g.mesh_vertices(&g.centroids(None));
    g.mesh_triangles(&g.mesh_faces());
}

#[divan::bench(consts = [1, 2, 4, 8, 16, 32, 64])]
fn normal<const N: u32>(bencher: Bencher) {
    let g = HexGlobe::<N>::new();
    let centroids = g.centroids(None);
    let vertices = g.mesh_vertices(&centroids);
    
    bencher.bench_local(move || {
        black_box(g.mesh_normals(&vertices));
    });
}