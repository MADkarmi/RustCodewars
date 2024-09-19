fn print(n: i32) -> Option<String> {
    if n < 0 || n%2 == 0{
        return None;
    }
    let n = n as usize;
    let diamond:String = (1..=n)
                .chain((1..n).rev())
                .step_by(2)
                .map(|times| format!("{}{}\n", " ".repeat((n - times)/2), "*".repeat(times)))
                .collect();

    Some(diamond)
}


fn main() {
    println!("{x}", x=print(5).unwrap() );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
    }

    #[test]
    fn basic_test1() {
        assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()) );
    }

    #[test]
    fn basic_test2() {
        assert_eq!(print(-3),None);
    }

    #[test]
    fn basic_test3() {
        assert_eq!(print(2),None);
    }

    #[test]
    fn basic_test4() {
        assert_eq!(print(0),None);
    }

    #[test]
    fn basic_test5() {
        assert_eq!(print(1), Some("*\n".to_string()) );
    }
}