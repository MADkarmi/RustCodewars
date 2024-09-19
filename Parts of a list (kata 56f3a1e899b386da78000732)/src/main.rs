fn part_list(arr: Vec<&str>) -> String {
    (1..arr.len()).map(|i| format!("({}, {})", arr[..i].join(" "), arr[i..].join(" "))).collect()
}

fn main() {
    println!("{:?}", part_list(vec!["1", "2", "3", "4", "5"]));
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(arr: Vec<&str>, exp: &str) -> () {
        println!("arr: {:?}", arr);
        let ans = part_list(arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn test() {
        dotest(vec!["I", "wish", "I", "hadn't", "come"],
                "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
    }

    #[test]
    fn test_1() {
        dotest(vec!["cdIw", "tzIy", "xDu", "rThG"], 
                "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)");
    }

    #[test]
    fn test_2() {
        dotest(vec!["az", "toto", "picaro", "zone", "kiwi"], 
            "(az, toto picaro zone kiwi)(az toto, picaro zone kiwi)(az toto picaro, zone kiwi)(az toto picaro zone, kiwi)")
    }

    #[test]
    fn test_3() {
        dotest(vec!["Lorem", "ipsum", "dolor", "sit", "amet"],
            "(Lorem, ipsum dolor sit amet)(Lorem ipsum, dolor sit amet)(Lorem ipsum dolor, sit amet)(Lorem ipsum dolor sit, amet)")
    }

    #[test]
    fn test_4() {
        dotest(vec!["1", "2", "3", "4", "5"], 
            "(1, 2 3 4 5)(1 2, 3 4 5)(1 2 3, 4 5)(1 2 3 4, 5)")
    }
}
