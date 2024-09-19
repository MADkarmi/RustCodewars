fn main() {
  let map1 = "abcdefghijklmnopqrstuvwxyz";
  let map2 = "etaoinshrdlucmfwypvbgkjqxz";
  
  let cipher = Cipher::new(map1, map2);
  
  println!("{}", cipher.decode(cipher.encode("lubie placki").as_str()));
}


struct Cipher {
    alphabet: String,
    cipher: String,
}
  
impl Cipher {
  fn new(map1: &str, map2: &str) -> Cipher {
    Cipher { 
      alphabet: map1.to_string(),
      cipher: map2.to_string() 
    }
  }
      
  fn encode(&self, string: &str) -> String {
    string.chars()
          .map(|char| {
            match self.alphabet.find(char) {
              Some(pos) => self.cipher.chars()
                                             .nth(pos)
                                             .unwrap()
                                             .to_string(),
              None => char.to_string(),
            }
          }).collect()
  }

  fn decode(&self, string: &str) -> String{
    string.chars()
          .map(|char| {
            match self.cipher.find(char){
              Some(pos) => self.alphabet.chars()
                                               .nth(pos)
                                               .unwrap()
                                               .to_string(),
              None => char.to_string(),
            }
          }).collect()
  }
}


#[cfg(test)]
  mod tests {
    use super::*;    
    const MAP1: &str = "abcdefghijklmnopqrstuvwxyz";
    const MAP2: &str = "etaoinshrdlucmfwypvbgkjqxz";
    
    #[test]
    fn test() {      
      let cipher = Cipher::new(MAP1, MAP2);
      
      assert_eq!(cipher.encode("abc"), "eta");      
    }
    
    #[test]
    fn test_1() {      
      let cipher = Cipher::new(MAP1, MAP2);

      assert_eq!(cipher.encode("xyz"), "qxz");
    }

    #[test]
    fn test_2() {      
      let cipher = Cipher::new(MAP1, MAP2);
      
      assert_eq!(cipher.decode("eirfg"), "aeiou");
    }

    #[test]
    fn test_3() {      
      let cipher = Cipher::new(MAP1, MAP2);
      
      assert_eq!(cipher.decode("erlang"), "aikcfu");
    }

    #[test]
    fn test_4() {      
      let cipher = Cipher::new(MAP1, MAP2);

      assert_eq!(cipher.encode("lubie placki"), "ugtri wuealr" )
    }
    
    #[test]
    fn test_5() {      
      let cipher = Cipher::new(MAP1, MAP2);

      assert_eq!(cipher.decode("ugtri wuealr"),"lubie placki")
    }
    
  }

