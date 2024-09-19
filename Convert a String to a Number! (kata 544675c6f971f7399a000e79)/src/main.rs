fn main() {    
    println!("{x}",x=string_to_number("42"));
}

fn string_to_number(s: &str) -> i32 {
    let num = s.parse::<i32>().unwrap();
    num
}

#[cfg(test)]
mod tests {
    use super::string_to_number;
    use rand::*;

    #[test]
    fn test1() {
      assert_eq!(string_to_number("1234"), 1234);
    }
    #[test]
    fn test2() {
      assert_eq!(string_to_number("605"), 605);
    }
    #[test]
    fn test3() {
      assert_eq!(string_to_number("1405"), 1405);
    }
    #[test]
    fn test4() {
      assert_eq!(string_to_number("-7"), -7);
    }

    #[test]
    fn test5() {
        let mut rng = thread_rng();
        for _ in 0..5 {
            let num : i32 = rng.gen();
            let input = num.to_string();
            assert_eq!(string_to_number(&input), num);
        }        
    }
}