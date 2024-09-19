fn main() {
    println!("{:?}", reverse_words("alla ma kodga"));
}

fn reverse_words(str: &str) -> String {    
    str.split(" ")
       .map(|ss| ss.chars()
                         .rev()
                         .collect())
       .collect::<Vec<String>>()
       .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
    assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
    }


    #[test]
    fn sample_test_1() {
        assert_eq!(reverse_words("apple"), "elppa");
    }


    #[test]
    fn sample_test_2() {
        assert_eq!(reverse_words("a b c d"),"a b c d");
    }


    #[test]
    fn sample_test_3() {
        assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
    }


    #[test]
    fn sample_test_4() {
        assert_eq!(reverse_words("alla ma kodga"), "alla am agdok");
    }
}