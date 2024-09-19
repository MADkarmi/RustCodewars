struct Sudoku{
    data: Vec<Vec<u32>>,
}


impl Sudoku{
    fn is_valid(&self) -> bool {
        let dim = self.data.len();
        let proper: Vec<u32> = (1..=(dim as u32)).collect();

        if self.data.iter()
                    .map(|row| row.len())
                    .any(|len| len != dim){
            return false;
        }

        for row in &self.data {
            let mut rw = row.clone();
            rw.sort();
            if rw != proper {
                return false;
            }
        }

        for col in 0..dim{
            let mut cl = self.data.clone()
                                                        .into_iter()
                                                        .map(|vector| vector[col])
                                                        .collect::<Vec<u32>>();
            cl.sort();
            if cl != proper {
                return false;
            }
        }

        let n = (dim as f64).sqrt() as usize;
        for sx in 0..n{
            for sy in 0..n{
                let mut little_squares_values = vec![];
                for x in 0..n{
                    for y in 0..n{
                        little_squares_values.push(self.data[sx * n + x][sy * n + y]);
                    }
                }
                little_squares_values.sort();
                if little_squares_values != proper{
                    return false;
                }
            }
        }
        true
    }
}


fn main() {

    let sudoku = Sudoku{
        data: vec![
                vec![1, 4,  2, 3],
                vec![3, 2,  4, 1],
                vec![4, 1,  3, 2],
                vec![2, 3,  1, 4],
            ]
    };

    println!("{}", sudoku.is_valid());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_sudoku() {
        let good_sudoku = Sudoku{
            data: vec![
                    vec![7,8,4, 1,5,9, 3,2,6],
                    vec![5,3,9, 6,7,2, 8,4,1],
                    vec![6,1,2, 4,3,8, 7,5,9],

                    vec![9,2,8, 7,1,5, 4,6,3],
                    vec![3,5,7, 8,4,6, 1,9,2],
                    vec![4,6,1, 9,2,3, 5,8,7],

                    vec![8,7,6, 3,9,4, 2,1,5],
                    vec![2,4,3, 5,6,1, 9,7,8],
                    vec![1,9,5, 2,8,7, 6,3,4]
                ]
        };

        assert!(good_sudoku.is_valid());
    }

    #[test]
    fn good_sudoku_2() {
        let good_sudoku = Sudoku{
            data: vec![
                    vec![1, 4,  2, 3],
                    vec![3, 2,  4, 1],
                    vec![4, 1,  3, 2],
                    vec![2, 3,  1, 4],
                ]
        };
        assert!(good_sudoku.is_valid());
    }

    #[test]
    fn bad_sudoku() {
        let bad_sudoku_1 = Sudoku{
            data: vec![
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],

                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],

                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                ]
        };


        assert!(!bad_sudoku_1.is_valid());

    }

    #[test]
    fn bad_sudoku_2() {
        let bad_sudoku = Sudoku{
            data: vec![
                    vec![1,2,3,4,5],
                    vec![1,2,3,4],
                    vec![1,2,3,4],
                    vec![1],
                ]
        };

        assert!(!bad_sudoku.is_valid());
    }

    #[test]
    fn good_sudoku_3() {
        let good_sudoku= Sudoku {
            data: vec![
                vec![1],
            ],
        };
        assert!(good_sudoku.is_valid());
    }

    #[test]
    fn bad_sudoku_3() {
        let bad_sudoku = Sudoku{
            data: vec![
                vec![1,2,3,4,5,6,7,8,9],
                vec![9,1,2,3,4,5,6,7,8],
                vec![8,9,1,2,3,4,5,6,7],
                vec![7,8,9,1,2,3,4,5,6],
                vec![6,7,8,9,1,2,3,4,5],
                vec![5,6,7,8,9,1,2,3,4],
                vec![4,5,6,7,8,9,1,2,3],
                vec![3,4,5,6,7,8,9,1,2],
                vec![2,3,4,5,6,7,8,9,1],
                vec![1,2,3,4,5,6,7,8,9]
            ]
        };
        assert!(!bad_sudoku.is_valid());
    }
}