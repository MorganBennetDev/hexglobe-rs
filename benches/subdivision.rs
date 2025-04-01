
use hexglobe::triangle::SubdividedTriangle;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench(consts = [1, 2, 4, 8, 16, 32, 64])]
fn subdivision<const N: usize>() {
    SubdividedTriangle::<N>::new();
}
