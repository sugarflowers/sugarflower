use rand::prelude::*;

fn get_rng() -> ThreadRng {
    rand::thread_rng()
}

pub fn random() -> f64 {
    let mut rng = get_rng();
    rng.gen()
}

pub fn get_i32(range_limit: i32) -> i32 {
    let r_f64 = random();
    (r_f64 * range_limit as f64) as i32
}

pub fn get_bool() -> bool {
    let mut rng = get_rng();
    rng.gen()
}

/*
 * About running tests. 
 *
 * $ cargo test – --nocapture 
 *
 *
 * Or if you want to test each function, run 
 *
 * $ cargo test <function name> – --nocapture 
 *
 *
 * Please note that the test code always returns true, 
 * so you need to see the real execution result.
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random() {
        println!("random {:?}", random());
        assert_eq!(true, true);
    }

    #[test]
    fn get_random_i32() {
        for _ in 1..20 {
            println!("get_i32: {:?}", get_i32(10));
        }
        assert_eq!(true, true);
    }
    #[test]
    fn get_random_bool() {
        println!("get_bool {:?}", get_bool());
        assert_eq!(true, true);
    }
}
