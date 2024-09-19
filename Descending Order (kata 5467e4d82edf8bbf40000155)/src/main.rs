fn main() {
    println!("{:?}", descending_order(1254859723));
    
}

// int to string, iterate over, push to vec and sort i nlogn
fn descending_order(x: u64) -> u64 {  
    let mut vec : Vec<u64> = 
       x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    vec.sort();    
    vec.iter().rev().fold(0, |res, number| res * 10 + number)    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test() {
        assert_eq!(descending_order(0), 0);
    }

    #[test]
    fn test_1() {
        assert_eq!(descending_order(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(descending_order(15), 51);
    }

    #[test]
    fn test_3() {
        assert_eq!(descending_order(1021), 2110);
    }

    #[test]
    fn test_4() {
        assert_eq!(descending_order(123456789), 987654321);
    }

    #[test]
    fn test_5() {
        assert_eq!(descending_order(145263), 654321);
    }

    #[test]
    fn test_6() {
        assert_eq!(descending_order(1254859723), 9875543221);
    }

    
}