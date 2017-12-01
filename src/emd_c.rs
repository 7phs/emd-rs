use libc::{c_float, c_int, c_uint};

#[repr(C)]
pub(crate) struct SignatureT {
    n: c_int,
    features: *const c_int,
    weights: *const c_float,
}

impl SignatureT {
    pub(crate) unsafe fn new(features: &[i32], weights: &[f32]) -> SignatureT {
        SignatureT {
            n: features.len() as c_int,
            features: features.as_ptr(),
            weights: weights.as_ptr(),
        }
    }
}

#[repr(C)]
pub(crate) struct DistFeaturesT {
    dim: c_uint,
    distance_matrix: *const c_float,
}

impl DistFeaturesT {
    pub(crate) unsafe fn new(distance_matrix: &[f32]) -> DistFeaturesT {
        DistFeaturesT {
            dim: (distance_matrix.len() as f32).sqrt() as c_uint,
            distance_matrix: distance_matrix.as_ptr(),
        }
    }
}

extern "C" {
    #[cfg(not(feature = "dumb"))]
    pub(crate) fn emd_light(
        signature1: *mut SignatureT,
        signature2: *mut SignatureT,
        distance: *mut DistFeaturesT
    ) -> c_float;

    #[cfg(feature = "dumb")]
    pub(crate) fn emd_dumb(
        signature1: *mut SignatureT,
        signature2: *mut SignatureT,
        distance: *mut DistFeaturesT
    ) -> c_float;
}

#[cfg(not(feature = "dumb"))]
pub(crate) unsafe fn emd_call(signature1: *mut SignatureT, signature2: *mut SignatureT, distance: *mut DistFeaturesT) -> c_float {
    emd_light(signature1, signature2, distance)
}

#[cfg(feature = "dumb")]
pub(crate) unsafe fn emd_call(signature1: *mut SignatureT, signature2: *mut SignatureT, distance: *mut DistFeaturesT) -> c_float {
    emd_dumb(signature1, signature2, distance)
}
