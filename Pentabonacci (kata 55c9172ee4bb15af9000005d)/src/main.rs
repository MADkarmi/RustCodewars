fn main() {    
    for i in 0..20 {
        println!("{i}, {:?}, {:?}", pentabonacci(i), count_odd_pentafib(i) );        
    }
    
}

fn count_odd_pentafib(n: u16) -> u16 {
    //println!("{:?}, {:?}", (n-1)/6, (n-2)/6);
    match n {
        0 => 0,
        _ => n.saturating_sub(1)/6 + n.saturating_sub(2)/6 + 1 
    }
}

fn pentabonacci(n : u16) -> i32{
    let mut pent = vec![0,1,1,2,4];
    for i in 5..=n {
        let z = i as usize;
        pent.push(pent[z-1]+pent[z-2]+pent[z-3]+pent[z-4]+pent[z-5]);
    }

    pent[n as usize]
    // match n{
    //     0 => 0,
    //     1 => 1,
    //     2 => 1,
    //     3 => 2,
    //     4 => 4,
    //     z => pentabonacci(z-1)+ pentabonacci(z-2) + pentabonacci(z-3) + pentabonacci(z-4) + pentabonacci(z-5)
    // }
}

#[cfg(test)]
mod tests {
    use super::count_odd_pentafib;

    #[test]
    fn basic_test() {
        assert_eq!(count_odd_pentafib(5), 1);
    }

    #[test]
    fn basic_test_1() {
        assert_eq!(count_odd_pentafib(10), 3);
    }
    
    #[test]
    fn basic_test_2() {
        assert_eq!(count_odd_pentafib(15), 5);
    }

    #[test]
    fn basic_test_3() {
        assert_eq!(count_odd_pentafib(45), 15);
    }

    #[test]
    fn basic_test_4() {
        assert_eq!(count_odd_pentafib(69), 23);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(count_odd_pentafib(0), 0);
        assert_eq!(count_odd_pentafib(1), 1);
        assert_eq!(count_odd_pentafib(2), 1);
    }
}