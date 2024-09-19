fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    let mut memory = vec![vec![false; width]; height];
    let (mut row, mut col) = (0 as usize, 0 as usize);
    let mut count = 0;
    let mut code_pointer: usize = 0;
    let code = code.chars()
        .filter(|&c| "eswn*[]".contains(c))
        .collect::<Vec<_>>();

    while count < iterations && code_pointer < code.len(){
        match code[code_pointer]{
            'n' => row = if row == 0 { height - 1 } else { row - 1 },
            'e' => col = if col == width - 1 { 0 } else { col + 1 },
            's' => row = if row == height - 1 { 0 } else { row + 1 },
            'w' => col = if col == 0 { width - 1 } else { col - 1 },
            '*' => memory[row][col] = !memory[row][col],
            '[' => 
                if !memory[row][col]{
                    let mut nesting = 1;
                    while nesting > 0 {
                        code_pointer+=1;
                        match code[code_pointer] {
                            '[' => nesting+=1,
                            ']' => nesting-=1,
                            _ => {},
                        }
                    }
                },
            ']' => 
                if memory[row][col]{
                    let mut nesting = 1;
                    while nesting > 0 {
                        code_pointer-=1;
                        match code[code_pointer]{
                            '[' => nesting -=1,
                            ']' => nesting +=1,
                            _ => {},
                        }
                    }
                },
            _ => { continue }
        }
        code_pointer += 1;
        count += 1;   
    }

    memory.iter().map(|row|{
        row.iter().map(|&cell|{
            if cell {'1'} else {'0'}
        }).collect::<String>()
    }).collect::<Vec<String>>().join("\r\n")
}


fn main() {    
    println!("{}", interpreter("*ne*s*[e*se*]wnnnewwwsswse", 7, 6, 9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9)), display_expected("000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should initialize all cells in the datagrid to 0");
    }

    #[test]
    fn simple_case_1() {
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9)), display_expected("111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should adhere to the number of iterations specified");
    }
    
    #[test]
    fn simple_case_2() {
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9)), display_expected("111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should traverse the 2D datagrid correctly");
    }
    
    #[test]
    fn simple_case_3() {
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
    }
    
    #[test]
    fn simple_case_4() {
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
    }

    /// Prints representation of datagrid - 0's are black and 1's are white.
    /// Note: it only works properly if your interpreter returns a representation
    /// of the datagrid in the correct format.
    fn pretty_print(datagrid: &str) -> &str {
        let rows = datagrid.split("\r\n");
        let mut output = String::new();
        output += "<pre>";
        for row in rows {
            for cell in row.chars() {
                output += "<span style=\"color:"; 
                output += if cell == '0' { "black" } else { "white" };
                output += ";background-color:"; 
                output += if cell == '0' { "black" } else { "white" };
                output += "\">xx</span>";
            }
            output += "<br />";
        }
        output += "</pre>";
        println!("{}", output);
        datagrid
    }

    /// Displays the grid the interpreter returns
    fn display_actual(actual: &str) -> &str {
        println!("You returned:");
        pretty_print(actual)
    }

    /// Displays the expected final state of datagrid
    fn display_expected(expected: &str) -> &str {
        println!("Expected final state of data grid:");
        pretty_print(expected)
    }   
}