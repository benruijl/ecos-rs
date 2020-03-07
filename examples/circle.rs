use std::ptr;
use std::slice;

extern crate ecos;

fn main() {
    // maximize k_x
    // f1 <= 50
    // |(k_x,k_y) + (2,2)| <= f1
    // expected solution: k_x=48, ky=-2

    // compressed sparse column representation
    /*[[ 0.  0.  1.]
    [ 0.  0. -1.]
    [-1.  0.  0.]
    [ 0. -1.  0.]]
    */
    let mut x = [-1., -1., 1., -1.];
    let mut p: [i64; 4] = [0, 1, 2, 4];
    let mut i: [i64; 4] = [2, 3, 0, 1];

    let mut h = vec![50., 0., 2., 2.];
    let mut c = vec![-1., 0., 0.];
    let mut q = vec![3i64]; // 1 quadratic constraint with 3 equations

    unsafe {
        let workspace = ecos::ECOS_setup(
            3, // num vars
            4, // num constraints
            0, // num eq
            1, // pos octant
            1, // num cones
            &mut q[0] as *mut ecos::idxint,
            0,                     // num exponents
            &mut x[0] as *mut f64, // matrix data
            &mut p[0] as *mut ecos::idxint,
            &mut i[0] as *mut ecos::idxint,
            ptr::null_mut(), // equality matrix
            ptr::null_mut(),
            ptr::null_mut(),
            &mut c[0] as *mut f64,
            &mut h[0] as *mut f64,
            ptr::null_mut(),
        );

        let _state = ecos::ECOS_solve(workspace);

        println!("Solution: {:?}", &slice::from_raw_parts((*workspace).x, 3));

        ecos::ECOS_cleanup(workspace, 0);
    }
}
