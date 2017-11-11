use emd_c::{SignatureT, DistFeaturesT, emd_light};
use libc::c_float;

fn emd_dumb(_: *mut SignatureT,
            _: *mut SignatureT,
            _: *mut DistFeaturesT
) -> c_float {
    0.0
}

pub fn emd(doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[&[f32]]) -> f32 {
    let sign1 = &mut SignatureT::new(doc_bow1);
    let sign2 = &mut SignatureT::new(doc_bow2);
    let distance = &mut DistFeaturesT::new(distance_matrix);

    unsafe {
        //        emd_dumb(sign1, sign2, distance)
        emd_light(sign1, sign2, distance)
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_emd() {
        const SZ: usize = 6;
        let doc_bow1: [f32; SZ] = [0.1, 0., 1.2, 0., 0., 2.4];
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
    use utils::random_f;
    use test::Bencher;

    fn prepare_data(sz: usize) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let rnd = random_f(0.3f32, 100.0f32);

        let doc_bow1: Vec<f32> = vec![0f32; sz].into_iter().map(|_| rnd()).collect();
        let doc_bow2: Vec<f32> = vec![0f32; sz].into_iter().map(|_| rnd()).collect();

        let mut distance_base: Vec<f32> = vec![0f32; sz * sz];
        {
            let mut distance_matrix: Vec<&mut [f32]> = distance_base.chunks_mut(sz).collect();

            (0..sz).for_each(|i|
                (i + 1..sz).for_each(|j| {
                    let value = rnd();

                    distance_matrix[i][j] = value;
                    distance_matrix[j][j] = value;
                })
            );
        }

        (doc_bow1, doc_bow2, distance_base)
    }

    #[bench]
    fn bench_emd_10(b: &mut Bencher) {
        let sz = 10;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_20(b: &mut Bencher) {
        let sz = 20;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_30(b: &mut Bencher) {
        let sz = 30;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_40(b: &mut Bencher) {
        let sz = 40;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_50(b: &mut Bencher) {
        let sz = 50;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_60(b: &mut Bencher) {
        let sz = 60;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_70(b: &mut Bencher) {
        let sz = 70;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_80(b: &mut Bencher) {
        let sz = 80;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_90(b: &mut Bencher) {
        let sz = 90;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }

    #[bench]
    fn bench_emd_100(b: &mut Bencher) {
        let sz = 100;
        let (doc_bow1, doc_bow2, distance_base) = prepare_data(sz);
        let distance_matrix = distance_base.chunks(sz).collect::<Vec<_>>();

        b.iter(move || emd(&doc_bow1, &doc_bow2, &distance_matrix))
    }
}