#![feature(test)]
#![feature(rand)]

extern crate rand;
extern crate libc;
extern crate test;

use libc::{c_float, c_int, c_uint};

#[repr(C)]
struct SignatureT {
    n: c_int,
    features: *const c_int,
    weights: *const c_float,
}

impl SignatureT {
    pub fn new(doc_bow: &[f32]) -> SignatureT {
        let (features, weights) = bow_to_features_weights(doc_bow);

        SignatureT {
            n: features.len() as c_int,
            features: features.as_ptr(),
            weights: weights.as_ptr(),
        }
    }
}

#[repr(C)]
struct DistFeaturesT {
    dim: c_uint,
    distance_matrix: *const c_float,
}

impl DistFeaturesT {
    pub fn new(distance_matrix: &[&[f32]]) -> DistFeaturesT {
        DistFeaturesT {
            dim: distance_matrix.len() as c_uint,
            distance_matrix: distance_matrix.concat().as_ptr(),
        }
    }
}

extern "C" {
    fn emd_light(
        signature1: *mut SignatureT,
        signature2: *mut SignatureT,
        distance: *mut DistFeaturesT
    ) -> c_float;
}

use rand::Rng;

pub fn random_f(probability_zero: f32, max: f32) -> Box<Fn() -> f32> {
    Box::new(move || {
        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < probability_zero {
            0.0f32
        } else {
            rng.gen_range::<f32>(0.0, max)
        }
    })
}

pub fn bow_to_features_weights(doc_bow: &[f32]) -> (Vec<i32>, Vec<f32>) {
    let mut features: Vec<i32> = vec![];
    let weights: Vec<f32> = doc_bow
        .into_iter()
        .enumerate()
        .filter_map(|(i, w)|
            if w != &0.0f32 {
                features.push(i as i32);
                Some(*w)
            } else {
                None
            })
        .collect();

    (features, weights)
}

pub fn emd(doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[&[f32]]) -> f32 {
    let sign1 = &mut SignatureT::new(doc_bow1);
    let sign2 = &mut SignatureT::new(doc_bow2);
    let distance = &mut DistFeaturesT::new(distance_matrix);

    unsafe {
        emd_light(sign1, sign2, distance)
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_random_f() {
        let expect = 99.0f32;

        assert!(random_f(0.0f32, expect)() < expect);

        assert!(random_f(1.0f32, expect)() == 0.0f32)
    }

    #[test]
    fn test_bow_to() {
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

    #[test]
    fn test_emd() {
        const SZ: usize = 6;
        let doc_bow1: [f32; SZ]  = [0.1, 0., 1.2, 0., 0., 2.4];
        let doc_bow2: [f32; SZ] = [0., 0.4, 0., 2., 0., 0.];

        let distance_matrix: &[&[f32]] = &[
            &[0., 1., 2., 1.5, 5.6, 3.],
            &[1., 0., 12., 0.5, 0.6, 13.],
            &[2., 12., 0., 1.5, 6.5, 8.],
            &[1.5, 0.5, 1.5, 0., 0.66, 33.],
            &[5.6, 0.6, 6.5, 0.66, 0., 3.],
            &[3., 13., 8., 33., 3., 0.],
        ];

        let exist = emd(&doc_bow1, &doc_bow2, &distance_matrix);
        let expected = 11.812499f32;

        assert_eq!(exist, expected);
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_emd_wrapper(b: &mut test::Bencher) {
        let sz = 100;

        let rnd = random_f(0.3f32, 100.0f32);

        let doc_bow1: Vec<f32> = vec![0f32; sz].into_iter().map(|_| rnd()).collect();
        let doc_bow2: Vec<f32> = vec![0f32; sz].into_iter().map(|_| rnd()).collect();

        let mut distance_base: Vec<f32> = vec![0f32; sz * sz];
        let mut distance_matrix: Vec<_> = distance_base.as_mut_slice().chunks_mut(sz).collect();

        (0..sz).for_each(|i| {
            (i + 1..sz).for_each(|j| {
                let value = rnd();

                distance_matrix[i][j] = value;
                distance_matrix[j][i] = value;
            })
        });

        let distance_base_: Vec<f32> = distance_matrix.concat();
        let distance_matrix_: Vec<_> = distance_base_.as_slice().chunks(sz).collect();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix_))
    }
}