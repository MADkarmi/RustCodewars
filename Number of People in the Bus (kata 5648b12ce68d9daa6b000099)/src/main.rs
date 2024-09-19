fn main() {
    let x = number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,20)]);
    println!("{}", x)
}

fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops.iter()
             .fold(0, |acc, (pin, pout)| acc + pin - pout)
}

#[test]
fn returns_expected() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
}
#[test]
fn returns_expected_1() {
  assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
}
#[test]
fn returns_expected_2() {
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}
#[test]
fn returns_expected_3() {
  assert_eq!(number(&[(3,0),(9,0),(4,0),(12,0),(6,0)]), 34);
}
#[test]
fn returns_expected_4() {
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,20)]), 9);
}