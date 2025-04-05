#![feature(generic_const_exprs)]
use hexglobe::projection::globe::Globe;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(consts = [1, 2, 4, 8, 16, 32, 64])]
fn projection<const N: u32>() where
    [(); (3 * N) as usize] : Sized {
    Globe::<N>::new();
}
