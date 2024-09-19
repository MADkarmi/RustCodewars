fn main() {
    let result = gimme_the_letters("a-i");
    println!("{result}");
}

fn gimme_the_letters(sp: &str) -> String {
    (sp.chars().nth(0).unwrap()..=sp.chars().nth(2).unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::gimme_the_letters;
         
    fn dotest(sp: &str, expected: &str) {
        let actual = gimme_the_letters(sp);
        assert!(actual == expected, 
            "With sp = \"{sp}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }
 
    #[test]
    fn fixed_test() {
        dotest("a-z", "abcdefghijklmnopqrstuvwxyz");
        dotest("h-o", "hijklmno");
    }
    #[test]
    fn fixed_test_1() {
        dotest("Q-Z", "QRSTUVWXYZ");
        dotest("J-J", "J");
    }
    #[test]
    fn fixed_test_2() {
        dotest("a-b", "ab");
        dotest("A-A", "A");
    }

    #[test]
    fn fixed_test_3() {
        dotest("g-i", "ghi");
        dotest("H-I", "HI");
    }

    #[test]
    fn fixed_test_4() {
        dotest("y-z", "yz");
        dotest("e-k", "efghijk");
    }

    #[test]
    fn fixed_test_5(){
        dotest("a-q", "abcdefghijklmnopq");
        dotest("F-O", "FGHIJKLMNO");
    }

}