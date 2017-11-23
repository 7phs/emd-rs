use libc::{c_float, c_int, c_uint};

#[repr(C)]
pub(crate) struct SignatureT {
    n: c_int,
    features: *const c_int,
    weights: *const c_float,
}

impl SignatureT {
    pub(crate) fn new(doc_bow: &[f32]) -> SignatureT {
        let (features, weights) = bow_to_features_weights(doc_bow);

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
    pub(crate) fn new(distance_matrix: &[&[f32]]) -> DistFeaturesT {
        DistFeaturesT {
            dim: distance_matrix.len() as c_uint,
            distance_matrix: distance_matrix.concat().as_ptr(),
        }
    }
}

extern "C" {
    pub(crate) fn emd_light(
        signature1: *mut SignatureT,
        signature2: *mut SignatureT,
        distance: *mut DistFeaturesT
    ) -> c_float;

    pub(crate) fn emd_dumb(
        signature1: *mut SignatureT,
        signature2: *mut SignatureT,
        distance: *mut DistFeaturesT
    ) -> c_float;
}

fn bow_to_features_weights(doc_bow: &[f32]) -> (Vec<i32>, Vec<f32>) {
    let mut features: Vec<i32> = vec![];
    let weights: Vec<f32> = doc_bow
        .iter()
        .enumerate()
        .filter_map(|(i, w)|
            if w < &1e-6f32 {
                None
            } else {
                features.push(i as i32);
                Some(*w)
            })
        .collect();

    (features, weights)
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_bow_to_features_weights() {
        vec![
            (vec![0f32; 5], vec![0i32; 0], vec![0f32; 0]),
            (
                vec![0.5f32, 0.0f32, 67.0f32, 0.01f32, 0.0f32, 105.567f32],
                vec![0i32, 2i32, 3i32, 5i32],
                vec![0.5f32, 67.0f32, 0.01f32, 105.567f32]
            ),
        ].into_iter()
            .for_each(|(doc_bow, features_expected, weights_expected)| {
                let (features_exist, weights_exist) = bow_to_features_weights(&doc_bow);

                assert_eq!(features_exist.len(), weights_exist.len());
                assert_eq!(features_exist, features_expected);
                assert_eq!(weights_exist, weights_expected);
            });
    }
}
