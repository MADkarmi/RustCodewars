use std::cmp::min;

fn main() {
    println!("{}",zoom(5));
}

fn zoom(n: i32) -> String {
    let mut result = Vec::new();
    for row in 0..n{
        for column in 0..n{
            let even = min(
                         min(row, n-row-1),
                         min(column, n-column-1)
                         ) % 2 == n / 2 % 2;
            // println!("{x}, {y}, {row}, {column}, {even}", x=n/2, y=n/2%2);
            result.push(if even { "■" } else { "□" });
        }
        // println!();
        result.push("\n");
    } 
    result.pop();
    result.join("")
}

#[test]
fn basic_test_1() {
  assert_eq!(zoom(1), "■");
}

#[test]
fn basic_test_2() {
  assert_eq!(zoom(3), "\
□□□
□■□
□□□"
  );
}

#[test]
fn basic_test_3() {
  assert_eq!(zoom(5), "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■"
  );
}

#[test]
fn basic_test_4() {
  assert_eq!(zoom(7), "\
□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□"
  );
}

#[test]
fn basic_test_5() {
  assert_eq!(zoom(9), "\
■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■"
  );
}