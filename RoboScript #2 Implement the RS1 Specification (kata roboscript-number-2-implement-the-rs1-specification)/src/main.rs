enum Direction {
    Left,
    Right,
    Up,
    Down   
}

enum Commands{
    TurnLeft(u32),
    TurnRight(u32),
    MoveForward(u32)
}

use regex::Regex;

fn parse(code: &str) -> Vec<Commands>{
    let mut parsed_commands : Vec<Commands> = vec![];

    for commands in  Regex::new(r#"([FLR])(\d*)"#).unwrap().captures_iter(code){
        let command = commands.get(1).unwrap().as_str();
        let times = commands.get(2).unwrap().as_str();
        
        match parse_command(command, times){
            Some(commands) => {
                parsed_commands.push(commands)
            }
            None => {}
        }
    }

    parsed_commands    
}

fn parse_command(command : &str, number_of_times : &str) -> Option<Commands>{
    
    let times: u32 =  match number_of_times {
        "" => 1,
        n => n.parse::<u32>().unwrap(),
    };

    match command{
        "F" => Some(Commands::MoveForward(times)),
        "L" => Some(Commands::TurnLeft(times)),
        "R" => Some(Commands::TurnRight(times)),
        _ => None,
    }
}

fn generate_path(commands: Vec<Commands>) -> Vec<(i32,i32)> {
    let mut direction = Direction::Right;
    let mut path = vec![(0,0)];
    let mut position = path[0];

    for command in commands.into_iter(){
        match command {
            Commands::MoveForward(times) => {
                for _ in 0..times {
                    position = move_forward(position, &direction);
                    path.push(position);
                }
            }
            
            Commands::TurnLeft(times) => {
                for _ in 0..times {
                    direction = turn_left(direction);
                }
            }
            
            Commands::TurnRight(times) => {
                for _ in 0..times {
                    direction = turn_right(direction)
                }
            }
        }
    }
    
    path
}

fn move_forward(position : (i32, i32), direction : &Direction) -> (i32, i32){
    let step = match direction {
        Direction::Left  => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Up    => (0, 1),
        Direction::Down  => (0, -1),
    };

    (position.0 + step.0, position.1 + step.1)
}

fn turn_right(direction : Direction) -> Direction {
    match direction {
        Direction::Left  => Direction::Down,
        Direction::Right => Direction::Up,
        Direction::Up    => Direction::Left,
        Direction::Down  => Direction::Right,
    }
}

fn turn_left(direction : Direction) -> Direction {
    match direction {
        Direction::Left  => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Up    => Direction::Right,
        Direction::Down  => Direction::Left,
    }
}

use std::cmp::min;
use std::cmp::max;

fn display_path(path : Vec<(i32,i32)>) -> String{
    let (x_min, x_max) : (i32,i32) = 
        path.iter().fold((0,0), 
                      |(min_x, max_x), (x, _)|{
                            (min(min_x, *x), max(max_x, *x))
                        });
    
    let (y_min, y_max) : (i32,i32) = 
        path.iter().fold((0,0), 
                      |(min_y, max_y), (_, y)|{
                            (min(min_y, *y), max(max_y, *y))
                        });

    let grid_dimensions:(usize, usize) = 
    (
        (x_min.abs() + x_max + 1) as usize, 
        (y_min.abs() + y_max + 1) as usize
    );

    let mut grid = vec![ vec![' '; grid_dimensions.0];  grid_dimensions.1];

    let mut pos_x : usize;
    let mut pos_y : usize;

    for (x,y) in path.iter(){
        pos_x = (x+x_min.abs()) as usize;
        pos_y = (y + y_min.abs()) as usize;
        grid[pos_y][pos_x] = '*';
    }

    grid.iter()
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\r\n")    
}

pub fn execute(code: &str) -> String {    
    display_path(generate_path(parse(code)))    
}

fn main() {
    println!("{}", execute("L2F5RF3R"));
    println!("{}",execute("F5LF20"));
    println!("{}",execute("F5RF2"));
    println!("{}",execute("FFFFFLFFFFFLFFFFFLFFFFFL"));
    println!("{}",execute("LFFFFFRFFFRFFFRFFFFFFF"));
}

  

#[cfg(test)]
macro_rules! expect_equal {
  ($actual:expr, $expected:expr $(,)*) => {{
    let actual = $actual;
    let expected = $expected;
    assert_eq!(actual, expected, "\ngot:\n{}\n\nexpected:\n{}\n", actual, expected);
  }};
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn test() {
        expect_equal!(execute(""), "*");
    }

    #[test]
    fn test1(){
        expect_equal!(execute("FFFFF"), "******");
    }

    #[test]
    fn test2(){
        expect_equal!(
            execute("FFFFFLFFFFFLFFFFFLFFFFFL"),
            "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******",
        );
    }

    #[test]
    fn test3(){
        expect_equal!(
            execute("LFFFFFRFFFRFFFRFFFFFFF"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
    }

    #[test]
    fn test4(){
        expect_equal!(
            execute("LF5RF3RF3RF7"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
}
}