#![allow(non_camel_case_types)]

extern crate libc;
use libc::size_t;
use libc::{c_void,c_double};

// Helper function that is passed to GSL
// Retrieves and calls the `user_data` closure
extern fn passed_func(x: c_double, user_data: *mut c_void) -> f64 {
    let cls_ptr = user_data as *mut |f64| -> f64;
    let cls: &mut |f64| -> f64 = unsafe { &mut *cls_ptr };
    (*cls)(x) as c_double
}

pub fn cquad(func: |f64| -> f64, a: f64, b: f64) -> (f64, f64, u64) {
    let result: &f64 = &0f64;
    let abserr: &f64 = &0f64;
    let nevals: &size_t = &0u64;

    unsafe {
        let ws = gsl_integration_cquad_workspace_alloc(100);

        // Some boxing and casting magic to convert the closure to *mut c_void
        let mut func_on_heap = box func;
        let user_data = &mut *func_on_heap as *mut |f64| -> f64 as *mut c_void;

        let f: gsl_function = gsl_function {
            // Helper function is passed as a function pointer to C
            function: Some(passed_func as extern fn(c_double, *mut c_void) -> c_double),
            // Actual closure from user is passed as user_data to GSL
            params: user_data as *mut c_void
        };

        gsl_integration_cquad(&f, a, b, 0f64, 1e-5f64, ws, result, abserr, nevals);
        gsl_integration_cquad_workspace_free(ws);
    }
    return (*result, *abserr, *nevals);
}

#[repr(C)]
struct gsl_integration_cquad_workspace;

#[repr(C)]
struct gsl_function {
    function: Option<extern "C" fn(c_double, *mut c_void) -> c_double>,
    params: *mut c_void
}

#[link(name="gsl")]
#[link(name="gslcblas")]
extern {
    fn gsl_integration_cquad_workspace_alloc(n: size_t) -> &gsl_integration_cquad_workspace;
    fn gsl_integration_cquad_workspace_free(ws: &gsl_integration_cquad_workspace);
    fn gsl_integration_cquad(f: &gsl_function, a: f64, b: f64, epsabs: f64, epsrel: f64,
                             ws: &gsl_integration_cquad_workspace, result: &f64, abserr: &f64, nevals: &size_t);
}

