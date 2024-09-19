fn main() {
    println!("{:?}", expanded_form(2137));
}

fn expanded_form(n: u64) -> String {
    let number = n.to_string();
    let len = number.len();
    let mut expanded = vec![];

    for(i, dig) in number.chars().enumerate(){
        if dig != '0'{
            expanded.push(format!("{}{}", dig, "0".repeat(len-i-1).to_string()));
        }
    }
    expanded.join(" + ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(expanded_form(12), "10 + 2");
    }

    #[test]
    fn test_1() {
        assert_eq!(expanded_form(42), "40 + 2");
    }
    
    #[test]
    fn test_2() {
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
    
    #[test]
    fn test_3() {
        assert_eq!(expanded_form(1984), "1000 + 900 + 80 + 4");
    }
    
    #[test]
    fn test_4() {
        assert_eq!(expanded_form(2137), "2000 + 100 + 30 + 7")
    }
}