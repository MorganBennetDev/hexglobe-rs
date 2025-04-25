use hexglobe::globe::ExactGlobe;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(consts = [1, 2, 4, 8, 16, 32, 64])]
fn adjacency<const N: u32>() {
    let g = ExactGlobe::<N>::new();
    
    g.adjacency();
}