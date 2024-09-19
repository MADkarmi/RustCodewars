2fn dig_pow(n: i64, p: i32) -> i64 {
    let k: i64 = n.to_string().chars()
    .map(|c| (c as i64) - 48)
    .enumerate()
    .map(|(i, d)| i64::pow(d, p as u32 + i as u32))
    .sum();

    match k%n {
        0 => k/n,
        _ => -1,
    }
}

fn main() {
    println!("{:?}", dig_pow(1, 0));
}


#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_test() {
        dotest(89, 1, 1);


    }

    #[test]
    fn basic_test1() {
        dotest(92, 1, -1);
    }

    #[test]
    fn basic_test2() {
        dotest(46288, 3, 51);
    }

    #[test]
    fn basic_test3() {
        dotest(46288, 3, 51)
    }

    #[test]
    fn basic_test4() {
        dotest(123, 4, -1)
    }
}
