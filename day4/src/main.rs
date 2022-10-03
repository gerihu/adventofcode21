use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let input: Vec<String> = reader.lines().filter_map(|line|line.ok()).collect();


    println!("Star 1 {}",play_bingo(&input, false));
    println!("Star 2 {}",play_bingo(&input, true));

}

#[derive(Debug)]
struct BingoBoard {
    mark_map: u32, //this is a reversed map of marked fields
    mark_count: u32,
    fields: [u32; 25],
}

impl BingoBoard {

    const SIZE: usize = 25;

    const TEST_PATTERNS: [u32; 10]= [
        0b1111100000000000000000000, 0b0000011111000000000000000, 0b0000000000111110000000000,
        0b0000000000000001111100000, 0b0000000000000000000011111, 
        0b1000010000100001000010000, 0b0100001000010000100001000, 0b0010000100001000010000100,
        0b0001000010000100001000010, 0b0000100001000010000100001, ];


    fn new(fields: [u32; 25]) -> Self {
        Self {
            mark_map: 0,
            mark_count: 0,
            fields,
        }
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0_u32;
        for i in 0_usize..25 {
            let bitpos = 1 << i;
            if self.mark_map & bitpos != bitpos {
                sum += self.fields[i];
            }
        }
        sum
    }

    fn mark(&mut self, number: u32) -> Option<u32> {
        // returns unmarked sum (>0) if winner else 0
        for i in 0usize..25 {
            if self.fields[i] == number {
                println!("{:0>25b}", 1_u32 << i);
                self.mark_map |= (1_u32 << i);
                self.mark_count += 1;
                break;
            }
        }
        println!("{number}: Board: {:?} {:0>25b}", self.fields, self.mark_map);

        // there must be at least 5 marks!
        if self.mark_count < 5 {
            return None;
        } else {
            for p in Self::TEST_PATTERNS {
                if self.mark_map & p == p {
                    // return Unmarked Sum
                    return Some(self.unmarked_sum());
                }
            }
            
            return None;
        }
    
    }


}


fn play_bingo(input: &Vec<String>, find_last_board: bool) -> u32 {
    // first line is random numbers
    // from third line is boards 
    let random_numbers: Vec<u32> = input[0].split(',').filter_map(|num| num.parse::<u32>().ok()).collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut fields: Vec<u32> = Vec::new();
    

    for line in &input[2..] {
        println!("line: {line}");

        let field_batch = line
            .split_ascii_whitespace()
            .filter_map(|num|num.trim().parse::<u32>().ok());

        fields.extend(field_batch);

        if fields.len() == BingoBoard::SIZE {
            boards.push(BingoBoard::new(fields.try_into().unwrap()));
            fields = Vec::new();
        }

    }

    for num in random_numbers {
        let mut remove_index: Vec<usize> = Vec::new();
        for i in 0..boards.len() {
            let board = &mut boards[i];
            if let Some(sum) = board.mark(num) {
                println!("Winner: {:?} {}", board.fields, sum*num);
                if ! find_last_board || boards.len() == 1 {
                    return sum*num
                }
                remove_index.push(i);
            }
        }
        for i in remove_index {
            boards.remove(i);
        }
    }

    0

}




#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

        
    
    fn get_input() -> Vec<String>{


        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
       21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
       19  8  7 25 23
       20 11 10 24  4
       14 21 16 12  6

       14 21 17 24  4
       10 16 15  9 19
       18  8 23 26 20
       22 11 13  6  5
        2  0 12  3  7".split('\n').map(|line|line.to_string()).collect();

        input

        

    }

    #[test]
    fn star_one() {
        assert_eq!(play_bingo(&get_input(), false), 4512);
    }

    #[test]
    fn star_two() {
        assert_eq!(play_bingo(&get_input(), true), 1924);
    }


}
        