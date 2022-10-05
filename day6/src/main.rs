use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero, One};

use std::fs::File;
use std::io::{prelude::*, BufReader};



fn spawn_fish(start_day: u16, days:u16) -> BigUint{
    //step n days until zero days and spawn fish for days - n
    //println!("starting with {start_day} for {days} days.");
    let steps = (start_day..days).step_by(7);
    // count myself here only
    let mut my_spawns = 1_u32.to_biguint().unwrap();
    for d in steps {
        my_spawns += spawn_fish(9, days-d);
    }
    
    my_spawns

}


fn spawn_lanternfish(input: &Vec<String>, days: u16) -> BigUint {

    let fish_states: Vec<u16> = input[0].split(',').filter_map(|num| num.parse::<u16>().ok()).collect();
    let mut total: BigUint = Zero::zero();
    
    for state in fish_states {
        total += spawn_fish(state, days);
        println!("{total}");}
    println!("Total: {}", &total);

    total

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
    
    use num_bigint::ToBigUint;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

        
    
    fn get_input() -> Vec<String>{


        vec!["3,4,3,1,2".to_string()]

    }

    #[test]
    fn star_one() {
        assert_eq!(spawn_lanternfish(&get_input(), 80), 5934.to_biguint().unwrap());
        //assert_eq!(spawn_lanternfish(&get_input(), 18), 26.to_biguint().unwrap());
    }

    #[test]
    fn star_two() {
        assert_eq!(spawn_lanternfish(&get_input(), 256), 26984457539_u64.to_biguint().unwrap());
    }


}
        