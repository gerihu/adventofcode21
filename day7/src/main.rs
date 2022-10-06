use std::fs::File;
use std::io::{prelude::*, BufReader};

fn fuel_use(positions: &[u16; 2000], min: &usize, max: &usize, target: usize) -> u32 {
    let mut fuel_use= 0usize;
    for (pos, cnt) in positions[*min..=*max].iter().enumerate() {
        if cnt > &0 {
            fuel_use += (*cnt as usize)*((pos as i32 - target as i32).abs() as usize);
            println!("pos {pos}, target {target}, cnt {cnt}, fuel {fuel_use}");
        }
    }
    fuel_use as u32
}


fn optimize_position(input: &Vec<String>, _some: u16) -> u32 {
    let mut positions = [0u16; 2000];
    let start_positions: &Vec<usize> = &input[0].split(',').filter_map(|num| num.parse::<usize>().ok()).collect();
    
    let mut min = usize::MAX;
    let mut max = 0usize;

    let mut ar_means = 0;
    for pos in start_positions {
        positions[*pos] += 1;
        ar_means += pos;
        if *pos > max {
            max = *pos;
        } else if *pos < min {
            min = *pos;
        }
    }

    let len = start_positions.len();

    ar_means /= len;
 
    let mut min_fuel_use = u32::MAX;

    for target in min..=max {
        let f = fuel_use(&positions, &min, &max, target);
        if f < min_fuel_use {
            min_fuel_use = f;
        } else {
            break;
        }
    }

    min_fuel_use
}

fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input: Vec<String> = reader.lines().filter_map(|line|line.ok()).collect();

    println!("Star 1: {}", optimize_position(&input, 80));
    //println!("Star 2: {}", optimize_position(&input, 256));
}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

        
    
    fn get_input() -> Vec<String>{


        vec!["16,1,2,0,4,2,7,1,2,14".to_string()]

    }

    #[test]
    fn star_one() {
        assert_eq!(optimize_position(&get_input(), 0), 37);
    }

    //#[test]
    fn star_two() {
        assert_eq!(optimize_position(&get_input(), 1), 0);
    }


}
        