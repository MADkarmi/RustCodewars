fn main() {
    let result = longest("alamakota", "aoskarpsa" );
    println!("{result}");
}

fn longest(a1: &str, a2: &str) -> String {
    ('a'..='z').filter(|c| (a1.contains(*c) || (a2.contains(*c))))
               .collect()
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
    }
    
    #[test]
    fn basic_tests_1() {
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
    }

    #[test]
    fn basic_tests_2() {        
        testing("alamakota", "aoskarpsa", "aklmoprst");
    }

    #[test]
    fn basic_tests_3() {
        testing("sześćrazysześć", "gdysięwitamymówimycześć", "acdegimrstwyz");
    }

    #[test]
    fn basic_tests_4() {
        testing("hamas", "zasas", "ahmsz");
    }
}