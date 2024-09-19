

fn main() {
    println!("{:?}", hamming(5));
}

fn hamming(n: usize) -> u64 {
    let n = n as usize;
    let mut hamming: Vec<u64> = vec![1];
    let mut exp: [usize; 3] = [0,0,0];
    while hamming.len() <= n {
        let min = vec![hamming[exp[0]] * 2, hamming[exp[1]] * 3, hamming[exp[2]] * 5]
            .into_iter()
            .min()
            .unwrap();
        hamming.push(min);
        if hamming[exp[0]] * 2 == min {
            exp[0] += 1;
        }
        if hamming[exp[1]] * 3 == min {
            exp[1] += 1;
        }
        if hamming[exp[2]] * 5 == min {
            exp[2] += 1;
        }
    }
    hamming[n - 1]
}

#[cfg(test)]
mod tests {
    use super::hamming;

    #[test]
    fn test() {
        assert_eq!(hamming(1), 1);
        assert_eq!(hamming(2), 2);
        assert_eq!(hamming(3), 3);
    }

    #[test]
    fn test_1(){
        assert_eq!(hamming(4), 4);
        assert_eq!(hamming(5), 5);
        assert_eq!(hamming(6), 6);
    }

    #[test]
    fn test_2(){
        assert_eq!(hamming(7), 8);
        assert_eq!(hamming(8), 9);
        assert_eq!(hamming(9), 10);
    }

    #[test]
    fn test_3(){
        assert_eq!(hamming(10), 12);
        assert_eq!(hamming(11), 15);
        assert_eq!(hamming(12), 16);
    }

    #[test]
    fn test_4(){
        assert_eq!(hamming(13), 18);
        assert_eq!(hamming(14), 20);
        assert_eq!(hamming(15), 24);
    }

    #[test]
    fn test_5(){
        assert_eq!(hamming(16), 25);
        assert_eq!(hamming(17), 27);
    }


    #[test]
    fn test_6(){
        assert_eq!(hamming(18), 30);
        assert_eq!(hamming(19), 32);
    }
}