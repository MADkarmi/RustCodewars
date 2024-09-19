fn main() {
    println!("{:?}", capitalize("hello, world!"));
}

fn capitalize(s: &str) -> Vec<String> {
    let (even_upper_res, odd_upper_res) : (String, String) =
             s.chars()
             .enumerate()
             .map(|(i,char)| 
                {
                    if i % 2 == 0 {
                        (char.to_ascii_uppercase(), char.to_ascii_lowercase())
                    } else {
                        (char.to_ascii_lowercase(), char.to_ascii_uppercase())
                    }
                })
             .unzip();
    vec![even_upper_res, odd_upper_res]
}

// fn capitalize(s: &str) -> Vec<String> {
//     let mut res = Vec::new();
//     let mut even_upper_res = String::new();
//     let mut odd_upper_res = String::new();

//     for (i, char) in s.chars().enumerate() {
//         if i%2 == 0{
//             even_upper_res.push(char.to_ascii_uppercase());
//             odd_upper_res.push(char.to_ascii_lowercase());
//         }
//         else {
//             even_upper_res.push(char.to_ascii_lowercase());
//             odd_upper_res.push(char.to_ascii_uppercase());
//         }
//     }
//     res.push(even_upper_res);
//     res.push(odd_upper_res);
//     res
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(capitalize("abcdef"),["AbCdEf", "aBcDeF"]);
    }

    
    #[test]
    fn test_1() {
        assert_eq!(capitalize("codewars"),["CoDeWaRs", "cOdEwArS"]);
    }
    
    #[test]
    fn test_2() {
        assert_eq!(capitalize("abracadabra"),["AbRaCaDaBrA", "aBrAcAdAbRa"]);
    }
    
    #[test]
    fn test_3() {
        assert_eq!(capitalize("codewarriors"),["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
    }
    
    #[test]
    fn test_4() {
        assert_eq!(capitalize("hello, world!"), ["HeLlO, wOrLd!", "hElLo, WoRlD!"]);
    }
}
