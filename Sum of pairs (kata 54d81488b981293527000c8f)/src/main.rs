fn main() {
    println!("{:?}", sum_pairs(&[11, 3, 7, 5],10));
}

use std::collections::HashSet;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut visited = HashSet::new();
    
    for &int in ints {
        if visited.contains(&(s - int)) {
            return Some((s - int, int));
        }
        visited.insert(int);
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test()
    {
        assert_eq!(sum_pairs(&[1, 4, 8, 7, 3, 15], 8), Some((1, 7)));
    }

    #[test]
    fn test_1()
    {
        assert_eq!(sum_pairs(&[1, -2, 3, 0, -6, 1], -6), Some((0, -6)));
    }

    #[test]
    fn test_2()
    {
        assert_eq!(sum_pairs(&[20, -13, 40], -7), None);
    }

    #[test]
    fn test_3()
    {
        assert_eq!(sum_pairs(&[1, 2, 3, 4, 1, 0], 2), Some((1, 1)));
    }

    #[test]
    fn test_4()
    {
        assert_eq!(sum_pairs(&[10, 5, 2, 3, 7, 5], 10), Some((3, 7)));
    }

    #[test]
    fn returns_expected() {        
        let l6 = [4, -2, 3, 3, 4];
        let l7 = [0, 2, 0];
        let l8 = [5, 9, 13, -3];
        assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
        assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
        assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    }

}