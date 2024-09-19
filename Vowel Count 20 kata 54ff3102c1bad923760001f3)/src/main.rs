fn main() {
    let result = get_count("addabu");
    println!("{result}");
}

fn get_count(string: &str) -> usize {
    string.matches(|c| "aeiou".contains(c)).count()
//   let mut s : String = string.to_string();
//   s.retain(|c| "aeiou".contains(c)); 
//   s.len()
}



#[test]
fn my_test() {
  assert_eq!(get_count("abracadabra"), 5);
}

#[test]
fn my_test1() {
  assert_eq!(get_count("addabu"), 3);
}

#[test]
fn my_test2() {
  assert_eq!(get_count("rrttwwweeeyyyy"), 3);
}

#[test]
fn my_test3() {
  assert_eq!(get_count("    "), 0);
}

#[test]
fn my_test4() {
  assert_eq!(get_count("aeiouy"), 5);
}