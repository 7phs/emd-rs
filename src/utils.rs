use rand;
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

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_random_f() {
        let expect = 99.0f32;

        assert!(random_f(0.0f32, expect)() < expect);

        assert_eq!(random_f(1.0f32, expect)(), 0.0f32);
    }
}