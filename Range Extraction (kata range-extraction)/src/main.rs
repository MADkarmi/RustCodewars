
pub mod solution {
    use itertools::Itertools;

    pub fn range_extraction(a: &[i32]) -> String {
        a.iter().map(|&i| (i,i))
                .coalesce(|a, b|{
                    if a.1 + 1 == b.0 {
                        Ok((a.0, b.1))
                    } else {
                        Err((a,b))
                    }
                }).map( |(beginning, end)| {
                    match end-beginning {
                        0 => format!("{}", end),
                        1 => format!("{},{}", beginning, end),
                        _ => format!("{}-{}", beginning, end),
                    }
                }).join(",")
    }
}

use solution::range_extraction;

fn main() {
    println!("{:?}", range_extraction(&[1,3,4,6,7,8,9,11,13,14]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]), "-6,-3-1,3-5,7-11,14,15,17-20");	
        
    }

    #[test]
    fn test_1(){
        assert_eq!(solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]), "-3--1,2,10,15,16,18-20");
    }
    
    #[test]
    fn test_2(){
        assert_eq!(solution::range_extraction(&[-10, -9, -8, -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]), "-10--8,-6,-3-1,3-5,7-11,14,15,17-20");
    }
    
    #[test]
    fn test_3(){
        assert_eq!(solution::range_extraction(&[1,3,4,6,7,8,9,11,13,14]), "1,3,4,6-9,11,13,14");
    }
    
    #[test]
    fn test_4(){
        assert_eq!(solution::range_extraction(&[-16,-4,-3,-2,-1,0,1,3,4,6,7,8,9,11,13,14]), "-16,-4-1,3,4,6-9,11,13,14")
    }
}
