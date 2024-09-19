fn main() {
    let result = summy("1 2 0 4");
    println!("{result}");
}

fn summy(strng: &str) -> i32 {
    strng.split_whitespace().fold(0, |acc, next| acc + next.parse::<i32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample_test() {
        assert_eq!(summy("1 2 3"), 6);
    }
    
    #[test]
    fn sample_test_1() {
        assert_eq!(summy("1 2 0 4"), 7);
    }
    
    #[test]
    fn sample_test_2() {
        assert_eq!(summy("1 2 3 4 5"), 15);
    }

    #[test]
    fn sample_test_3() {
        assert_eq!(summy("10 10"), 20);
    }

    #[test]
    fn sample_test_4() {
        assert_eq!(summy("0 0"), 0);
    }
}