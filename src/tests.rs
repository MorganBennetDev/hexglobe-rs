extern crate test;

use test::bench::Bencher;

use crate::triangle::SubdividedTriangle;

#[test]
fn subdivision() {
    let test = SubdividedTriangle::<3>::new();
    
    assert_eq!(test.vertex_count(), 10, "Incorrect number of vertices in subdivision.");
    assert_eq!(test.triangle_count(), 9, "Incorrect number of triangles in subdivision.");
}

macro_rules! subdivision_bench {
    ($n: expr, $name: ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| SubdividedTriangle::<$n>::new())
        }
    };
}

subdivision_bench!(0, subdivision_0_bench);
subdivision_bench!(1, subdivision_1_bench);
subdivision_bench!(2, subdivision_2_bench);
subdivision_bench!(4, subdivision_4_bench);
// subdivision_bench!(8, subdivision_8_bench);
// subdivision_bench!(16, subdivision_16_bench);
// subdivision_bench!(32, subdivision_32_bench);
// subdivision_bench!(64, subdivision_64_bench);
// subdivision_bench!(128, subdivision_128_bench);
// subdivision_bench!(256, subdivision_256_bench);
// subdivision_bench!(512, subdivision_512_bench);
// subdivision_bench!(1024, subdivision_1024_bench);
// subdivision_bench!(2048, subdivision_2048_bench);
// subdivision_bench!(4096, subdivision_4096_bench);