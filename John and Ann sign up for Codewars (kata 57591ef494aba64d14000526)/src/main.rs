fn john(n: i32) -> Vec<i32> {
    catas(n).1
}

fn ann(n: i32) -> Vec<i32> {
    catas(n).0
}

fn sum_john(n: i32) -> i32 {
    john(n).iter().sum()
}

fn sum_ann(n: i32) -> i32 {
    ann(n).iter().sum()
}

fn catas(n: i32) -> (Vec<i32>, Vec<i32>){
    let mut ann_catas = vec![1];
    let mut john_catas = vec![0];

    for i in 1..n{
        john_catas.push(i - ann_catas[*john_catas.last().unwrap() as usize]);
        ann_catas.push(i - john_catas[*ann_catas.last().unwrap() as usize]);
    }

    (ann_catas, john_catas)
}

fn main() {
    println!("{:?}", john(1));
    println!("{:?}", ann(1));
    println!("{:?}", sum_john(34));
    println!("{:?}", sum_ann(34));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_john() {
        assert_eq!(john(11), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
    }

    #[test]
    fn test_john_1() {
        assert_eq!(john(14), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
    }

    #[test]
    fn test_ann() {
        assert_eq!(ann(6), vec![1, 1, 2, 2, 3, 3]);
        
    }
    
    #[test]
    fn test_ann_1() {
        assert_eq!(ann(15), vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
    }

    #[test]
    fn test_sum_john() {
        assert_eq!(sum_john(75), 1720);

    }
    
    #[test]
    fn test_sum_john_1() {
        assert_eq!(sum_john(78), 1861);
    }

    #[test]
    fn test_sum_ann() {
        assert_eq!(sum_ann(115), 4070);
    }
    
    #[test]
    fn test_sum_ann_1() {
        assert_eq!(sum_ann(150), 6930);
    }
}