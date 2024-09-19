fn encode(msg: String, n: i32) -> Vec<i32> {
    msg.chars()
        .zip(n.to_string().chars().cycle())
        .map(|(a, b)| (a as i32) - ('a' as i32) + 1 + (b as i32) - ('0' as i32))
        .collect::<Vec<i32>>()
}

fn main() {
    println!("{:?}", encode("kopytko".to_string(), 1996));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_test() {
        assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);

    }

    #[test]
    fn fixed_test1() {
        assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
    }

    #[test]
    fn fixed_test2() {
        assert_eq!(encode("ziemniak".to_string(), 42), vec![30, 11, 9, 15, 18, 11, 5, 13]);
    }

    #[test]
    fn fixed_test3() {
        assert_eq!(encode("lubieplacki".to_string(), 2137), vec![14, 22, 5, 16, 7, 17, 15, 8, 5, 12, 12]);
    }

    #[test]
    fn fixed_test4() {
        assert_eq!(encode("kopytko".to_string(), 1996), vec![12, 24, 25, 31, 21, 20, 24]);
    }
}