use emd_c::{SignatureT, DistFeaturesT, emd_call};

pub fn emd(doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[f32]) -> f32 {
    let (features1, weights1) = bow_to_features_weights(doc_bow1);
    let (features2, weights2) = bow_to_features_weights(doc_bow2);

    unsafe {
        emd_call(
            &mut SignatureT::new(features1.as_slice(), weights1.as_slice()),
            &mut SignatureT::new(features2.as_slice(), weights2.as_slice()),
            &mut DistFeaturesT::new(distance_matrix)
        )
    }
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

    #[cfg(not(feature = "dumb"))]
    #[test]
    fn test_emd() {
        let doc_bow1: &[f32] = &[0.5, 0.5, 0.0, 0.0];
        let doc_bow2: &[f32] = &[0.0, 0.0, 0.5, 0.5];

        let distance_matrix: &[f32] = &[
            0., 0., 16.03984642, 22.11830902,
            0., 0., 17.83054543, 14.92696762,
            0., 0., 0., 0.,
            0., 0., 0., 0.,
        ];

        let exist = emd(&doc_bow1, &doc_bow2, &distance_matrix);
        let expected = 15.483407f32;

        assert_eq!(exist, expected);
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    use utils::random_f;
    use test::Bencher;

    fn prepare_data(sz: usize) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let rnd = random_f(0.3f32, 100.0f32);

        let doc_bow1: Vec<f32> = vec![0f32; sz].into_iter().map(|_| rnd()).collect();
        let doc_bow2: Vec<f32> = vec![0f32; sz].into_iter().map(|_| rnd()).collect();

        let mut distance_matrix: Vec<f32> = vec![0f32; sz * sz];
        {
            let mut distance_2d: Vec<&mut [f32]> = distance_matrix.chunks_mut(sz).collect();

            (0..sz).for_each(|i|
                (i + 1..sz).for_each(|j| {
                    let value = rnd();

                    distance_2d[i][j] = value;
                    distance_2d[j][j] = value;
                })
            );
        }

        (doc_bow1, doc_bow2, distance_matrix)
    }

    fn bench_emd(sz: usize, b: &mut Bencher) {
        let (doc_bow1, doc_bow2, distance_matrix) = prepare_data(sz);

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_10(b: &mut Bencher) {
        bench_emd(10, b);
    }

    #[bench]
    fn bench_emd_20(b: &mut Bencher) {
        bench_emd(20, b);
    }

    #[bench]
    fn bench_emd_30(b: &mut Bencher) {
        bench_emd(30, b);
    }

    #[bench]
    fn bench_emd_40(b: &mut Bencher) {
        bench_emd(40, b);
    }

    #[bench]
    fn bench_emd_50(b: &mut Bencher) {
        bench_emd(50, b);
    }

    #[bench]
    fn bench_emd_60(b: &mut Bencher) {
        bench_emd(60, b);
    }

    #[bench]
    fn bench_emd_70(b: &mut Bencher) {
        bench_emd(70, b);
    }

    #[bench]
    fn bench_emd_80(b: &mut Bencher) {
        bench_emd(80, b);
    }

    #[bench]
    fn bench_emd_90(b: &mut Bencher) {
        bench_emd(90, b);
    }

    #[bench]
    fn bench_emd_100(b: &mut Bencher) {
        bench_emd(100, b);
    }
}