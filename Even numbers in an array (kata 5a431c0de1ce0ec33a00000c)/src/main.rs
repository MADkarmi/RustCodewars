fn main() {
    println!("{:?}", even_numbers(&vec![2,5,5,7,9,1,1,1,5], 2));
}

fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    let mut digits = array.iter()
                                 .cloned()
                                 .filter(|d| d%2==0)
                                 .rev()
                                 .take(number)
                                 .collect::<Vec<i32>>();
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample_test() {
        assert_eq!(even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3), vec!(4, 6, 8));
    }   

    
    #[test]
    fn sample_test_1() {
        assert_eq!(even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2), vec!(-8, 26));
    }

    #[test]
    fn sample_test_2() {
        assert_eq!(even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1), vec!(6));
    }

    #[test]
    fn sample_test_3() {
        assert_eq!(even_numbers(&vec![2,5,6,7,9,4,1,2,5], 2), vec!(4,2));
    }

    #[test]
    fn sample_test_4() {
        assert_eq!(even_numbers(&vec![2,5,5,7,9,1,1,1,5], 2), vec!(2));
    }



}