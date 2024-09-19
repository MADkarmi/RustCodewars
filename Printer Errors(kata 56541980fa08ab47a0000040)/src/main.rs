fn main() {
    println!("{x}", x = printer_error(""));
}

fn printer_error(s: &str) -> String {
    let control_string_length = s.len();
    let error_chars_count = s.chars().filter(|&c| c < 'a' || c > 'm').count();
    format!("{}/{}", error_chars_count, control_string_length)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(&printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "3/56");
    }

    #[test]
    fn test2() {        
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"), "11/65");
    }
    
    #[test]
    fn test3() {
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "6/60");
    }

    #[test]
    fn test4() {
        assert_eq!(&printer_error("aaabbbbhaijjjm"), "0/14");
    }
    
    #[test]
    fn test5() {
        assert_eq!(&printer_error("aaaxbbbbyyhwawiwjjjwwm"), "8/22");
    }
}