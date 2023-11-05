/*
    Fast Fahrenheit to Celsius conversion
*/

/*
    True conversions
*/

fn round_to_int(x: f64) -> isize {
    x.round() as isize
}

fn true_f_to_c(f: isize) -> isize {
    round_to_int(((f - 32) as f64) * 5.0 / 9.0)
}

fn true_c_to_f(c: isize) -> isize {
    round_to_int(((c as f64) * 9.0 / 5.0) + 32.0)
}

/*
    Heuristics

    This is a useful mental math trick, see e.g.
    https://www.aaamath.com/mea514x3.htm

    It's easiest in the case F is even,
    as the first line becomes integer arithmetic

    A different trick is F = 2C + 30, but it is a much worse
    approximation.
*/

pub fn fast_f_to_c_v1(f: isize) -> isize {
    let x = (f - 32) as f64 / 2.0;
    let dec1 = x / 10.0;
    // shift additional times for better accuracy
    let dec2 = x / 100.0;
    let dec3 = x / 1000.0;
    round_to_int(x + dec1 + dec2 + dec3)
}

// Version without the additional shifting
pub fn fast_f_to_c_v2(f: isize) -> isize {
    let x = (f - 32) as f64 / 2.0;
    let dec1 = x / 10.0;
    round_to_int(x + dec1)
}

/*
    Trick for the other direction is easier
*/

pub fn fast_c_to_f(c: isize) -> isize {
    let x = (c * 2) as f64;
    let shift = x / 10.0;
    round_to_int(x - shift + 32.0)
}

/*
    Tests

    The heuristic works for temperatures from -40 to 212 F
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_to_int() {
        // positive cases
        assert_eq!(round_to_int(0.49), 0);
        assert_eq!(round_to_int(0.5), 1);
        assert_eq!(round_to_int(1.4), 1);
        assert_eq!(round_to_int(1.5), 2);
        assert_eq!(round_to_int(2.0), 2);
        // negative cases
        assert_eq!(round_to_int(-0.4), 0);
        assert_eq!(round_to_int(-0.5), -1);
        assert_eq!(round_to_int(-0.6), -1);
        assert_eq!(round_to_int(-1.499), -1);
        assert_eq!(round_to_int(-1.5), -2);
    }

    fn test_eq(f: isize, c: isize) {
        assert_eq!(true_f_to_c(f), c);
        assert_eq!(true_c_to_f(c), f);
    }

    #[test]
    fn sanity_check() {
        test_eq(32, 0);
        test_eq(212, 100);
        test_eq(99, 37);
        test_eq(100, 38);
        test_eq(0, -18);
        test_eq(-40, -40);
    }

    const MIN_F: isize = -40;
    const MAX_F: isize = 212;
    const MIN_C: isize = -40;
    const MAX_C: isize = 100;

    #[test]
    fn test_f_to_c_v1() {
        for f in MIN_F..=MAX_F {
            assert_eq!(fast_f_to_c_v1(f), true_f_to_c(f), "failed for {}F", f);
        }
    }

    #[test]
    fn test_c_to_f() {
        for c in MIN_C..=MAX_C {
            assert_eq!(fast_c_to_f(c), true_c_to_f(c), "failed for {}C", c);
        }
    }
}

fn main() {
    println!("Enter a temperature in Fahrenheit:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let f: isize = input.trim().parse().unwrap();
    println!(
        "{}F -> {}C    (true value: {}C)",
        f,
        fast_f_to_c_v2(f),
        true_f_to_c(f)
    );

    println!("Enter a temperature in Celsius:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let c: isize = input.trim().parse().unwrap();
    println!(
        "{}C -> {}F    (true value: {}F)",
        c,
        fast_c_to_f(c),
        true_c_to_f(c)
    );
}
