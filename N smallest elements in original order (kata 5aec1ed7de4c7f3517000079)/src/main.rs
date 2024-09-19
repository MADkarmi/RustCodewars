use itertools::Itertools;

fn first_n_smallest (arr: &[i32], n: usize) -> Vec<i32> {
    arr.iter()
        .enumerate()
        .sorted_by_key(|(_, &val)| val)
        .take(n)
        .sorted_by_key(|(index, _)| *index)
        .map(|(_, &val)| val)
        .collect()
}

fn main() {
    println!("{:?}", first_n_smallest(&[1,2,3,4,5],3));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first_n_smallest(&[1,2,3,4,5],3), [1,2,3]);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],4), [2,1,2,2]);
    }
    
    #[test]
    fn test_1() {
        assert_eq!(first_n_smallest(&[5,4,3,2,1],3), [3,2,1]);
        assert_eq!(first_n_smallest(&[1,2,3,4,5],5), [1,2,3,4,5]);
    }

    #[test]
    fn test_2() {
        assert_eq!(first_n_smallest(&[1,2,3,1,2],3), [1,2,1]);
        assert_eq!(first_n_smallest(&[1,2,3,4,2],4), [1,2,3,2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(first_n_smallest(&[1,2,3,-4,0],3), [1,-4,0]);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],2), [2,1]);
    }

    #[test]
    fn test_4() {
        assert_eq!(first_n_smallest(&[1,2,3,4,5],0), []);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],3), [2,1,2]);
    }
}
