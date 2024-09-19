fn main() {
    println!("{}", last_digit("518","780") );
}

// www.youtube.com/watch?v=cQm__9qnXIw&ab_channel=Experts%27Global

fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" { 
        1 
    }
    else {
        let last_digit = str1.chars()
                                  .last()
                                  .unwrap()
                                  .to_digit(10)
                                  .unwrap();    

        let remainder: u32 = str2.chars()
                                 .fold(0, |acc,c| 
                                    (acc*10 + c.to_digit(10).unwrap()) % 4);
        
        let exp = if remainder == 0 { 4 } else { remainder };
        
        (last_digit.pow(exp) % 10) as i32
    }   
}

#[test]
fn test() {
    assert_eq!(last_digit("518","780"), 6);
}

#[test]
fn test_1() {
    assert_eq!(last_digit("537","3958"), 9);
}

#[test]
fn test_2() {
    assert_eq!(last_digit("38","175"), 2);
}

#[test]
fn test_3() {
    assert_eq!(last_digit("32","1240"), 6);
}

#[test]
fn test_4() {
  assert_eq!(last_digit("4", "1"), 4);
}

#[test]
fn test_5() {
    assert_eq!(last_digit("4", "2"), 6);
}

#[test]
fn test_6() {
    assert_eq!(last_digit("9", "7"), 9);
}

#[test]
fn test_7() {
    assert_eq!(last_digit("10","10000000000"), 0);
}

#[test]
fn test_8() {
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376",
    "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
}

#[test]
fn test_9() {
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723",
     "68819615221552997273737174557165657483427362207517952651"), 7);
}