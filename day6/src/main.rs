use std::fs::File;
use std::io::{prelude::*, BufReader};



fn spawn_lanternfish(input: &Vec<String>, days: u16) -> u64 {

    let mut fish = [0_u64; 9];
    let day_states: Vec<usize> = input[0].split(',').filter_map(|num| num.parse::<usize>().ok()).collect();
    for state in day_states {
        fish[state] += 1;
    }
    for _day in 1..=days{
        let fish0 = fish[0]; 
        for shift in 0..8 {
            fish[shift] = fish[shift+1]
        }
        fish[6] += fish0;
        fish[8] = fish0;
    }
    fish.iter().sum()
}


fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let input: Vec<String> = reader.lines().filter_map(|line|line.ok()).collect();

    println!("Star 1: {}", spawn_lanternfish(&input, 80));
    println!("Star 2: {}", spawn_lanternfish(&input, 256));
}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

        
    
    fn get_input() -> Vec<String>{


        vec!["3,4,3,1,2".to_string()]

    }

    #[test]
    fn star_one1() {
       assert_eq!(spawn_lanternfish(&get_input(), 18), 26);
    }

    #[test]
    fn star_one2() {
        assert_eq!(spawn_lanternfish(&get_input(), 80), 5934);
    }

    //#[test]
    fn star_two() {
        assert_eq!(spawn_lanternfish(&get_input(), 256), 26984457539);
    }


}
        