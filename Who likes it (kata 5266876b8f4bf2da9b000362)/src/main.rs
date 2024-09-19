
// fn likes(names: &[&str]) -> String {
//     //println!("{:?}", names.len());
//     let result = String::from("a likes this");
    
//     match names.len() {
//         0 => {
//             return result.replace("a","no one")
//         },
//         1 => {
//             return result.replace("a", names[0])
//         },
//         2 => {
//             let replace = names[0].to_string() + " and " + names[1] + " like";
//             return result.replace("a likes", replace.as_str())
//         },
//         3 => { 
//             let replace = names[0].to_string() +", " + names[1] + " and " + names[2] + " like";
//             return result.replace("a likes", replace.as_str())
//         },
//         _  => { 
//             let replace = names[0].to_string() +", " + names[1] + " and " + (names.len()-2).to_string().as_str() + " others like";
//             return result.replace("a likes", replace.as_str())
//         }, 
//     }
// }
fn likes(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}
fn main() {
    println!("{:?}", likes(&["Peter"]));
}
#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
    }
    #[test]
    fn example_tests1() {
        assert_eq!(
            likes(&["Alex", "Jacob", "Rex", "Mark", "Max"]),
            "Alex, Jacob and 3 others like this"
        );
    }
    
    #[test]
    fn example_tests2() {
        assert_eq!(likes(&["Peter"]), "Peter likes this");
    }
    #[test]
    fn example_tests3() {
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    }
    #[test]
    fn example_tests4() {
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
    }
    #[test]
    fn example_tests5() {
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}