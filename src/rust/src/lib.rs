use extendr_api::prelude::*;

/// Calculate Euclidean distance matrix
/// @export
#[extendr]
fn euclidean_dist(a: ArrayView2<f64>) -> Vec<f64> {
    let nrow = a.nrows();
    let outsize = nrow * (nrow - 1) / 2;
    let mut out = Vec::<f64>::with_capacity(outsize);

    for x in 0..(nrow - 1) {
        for y in (x + 1)..nrow {
            let mut z = &a.slice(s![x, ..]) - &a.slice(s![y, ..]);
            z = &z * &z;
            let b = z.sum();
            out.push(b.sqrt());
        }
    }
    out
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod examplerust;
    fn euclidean_dist;
}
