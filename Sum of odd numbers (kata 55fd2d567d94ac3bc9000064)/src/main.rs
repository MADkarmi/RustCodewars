fn main() {
    let x = row_sum_odd_numbers(5);
    println!("{}", x)
}

fn row_sum_odd_numbers(n:i64) -> i64 {
    /*
    1 is the product of 111 = 1
    8 is the product of 222 = 8
    27 is the product of 333 = 27
    The sum of any row of odd numbers is just the row number cubed
     */

    n*n*n
}

#[test]
fn returns_expected() {
  assert_eq!(row_sum_odd_numbers(1), 1);
}

#[test]
fn returns_expected_1() {
    assert_eq!(row_sum_odd_numbers(42), 74088);
}

#[test]
fn returns_expected_2() {
  assert_eq!(row_sum_odd_numbers(3), 27);
}

#[test]
fn returns_expected_3() {
  assert_eq!(row_sum_odd_numbers(4), 64);
}

#[test]
fn returns_expected_4() {
  assert_eq!(row_sum_odd_numbers(5), 125);
}

#[test]
fn returns_expected_5() {
  assert_eq!(row_sum_odd_numbers(6), 216);
}

#[test]
fn returns_expected_6() {
  assert_eq!(row_sum_odd_numbers(7), 343);
}

#[test]
fn returns_expected_7() {
  assert_eq!(row_sum_odd_numbers(8), 512);
}

#[test]
fn returns_expected_8() {
  assert_eq!(row_sum_odd_numbers(9), 729);
}
#[test]
fn returns_expected_9() {
  assert_eq!(row_sum_odd_numbers(10), 1000);
}