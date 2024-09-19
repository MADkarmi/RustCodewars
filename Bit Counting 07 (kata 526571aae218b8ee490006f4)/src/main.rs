fn main() {
    let result = count_bits(1234).to_string();
    println!("{result}");
}

fn count_bits(n: i64) -> u32 {
    n.count_ones()
}


#[test]
fn test() {
    assert_eq!(count_bits(0), 0);
}

#[test]
fn test_1() {
    assert_eq!(count_bits(4), 1);
}

#[test]
fn test_2() {
    assert_eq!(count_bits(7), 3);
}

#[test]
fn test_3() {
    assert_eq!(count_bits(9), 2);
}

#[test]
fn test_4() {
    assert_eq!(count_bits(10), 2);
}