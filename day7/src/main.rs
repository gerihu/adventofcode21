use std::fs::File;
use std::io::{prelude::*, BufReader};


enum Star {
    One,
    Two,
}

enum Direction {
    Up,
    Down,
}

fn fuel_use2(positions: &[u16; 2000], min: &usize, max: &usize, target: usize) -> u32 {
    let mut fuel_use= 0usize;
    for (pos, cnt) in positions[*min..=*max].iter().enumerate() {
        if cnt > &0 {
            let delta = (pos as i32 - target as i32).abs() as usize;
            fuel_use += (*cnt as usize)*(delta.pow(2) + delta)/2;
            println!("pos {pos}, target {target}, cnt {cnt}, fuel {fuel_use}");
        }
    }
    fuel_use as u32
}

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




fn read_input(input: &Vec<String>) -> ([u16;2000], usize, usize, usize) {
    let mut positions = [0u16; 2000];
    let start_positions: &Vec<usize> = &input[0].split(',').filter_map(|num| num.parse::<usize>().ok()).collect();
    
    let mut min = usize::MAX;
    let mut max = 0usize;    // use i32 because we want to be able to substract

    let mut means = 0usize;
    for pos in start_positions {
        positions[*pos] += 1;
        means += *pos;
        if *pos > max {
            max = *pos;
        } else if *pos < min {
            min = *pos;
        }
    }
    means /= start_positions.len();
    (positions, min, max, means)

}


fn optimize_position(input: &Vec<String>, star: Star) -> u32 {
    
    let (positions, min, max, means) = read_input(input);
 
    let mut min_fuel_use = u32::MAX;

    let mut target = means;
    let mut direction = Direction::Up;

    while target > min && target < max {
        let f = match star {
            Star::One => fuel_use(&positions, &min, &max, target),
            Star::Two => fuel_use2(&positions, &min, &max, target),
        };

        if f < min_fuel_use {
            min_fuel_use = f;
            target = match direction {
                Direction::Up => target +1,
                _ => target -1,
            };
        } else {
            if target == means + 1 {
                //distance goes up, change direction!
                direction = Direction::Down;
                target = means -1; 
            } else {
                break;
            }
        } 
    }

    min_fuel_use
}

fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input: Vec<String> = reader.lines().filter_map(|line|line.ok()).collect();


    println!("Star 1: {}", optimize_position(&input, Star::One));
    println!("Star 2: {}", optimize_position(&input, Star::Two));
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
        assert_eq!(optimize_position(&get_input(), Star::One), 37);
    }

    #[test]
    fn star_two() {
        assert_eq!(optimize_position(&get_input(), Star::Two), 168);

    }

    #[test]
    fn star_two2() {
        let (positions, min, max, means) = read_input(&get_input());
        assert_eq!(fuel_use2(&positions, &min, &max, 2), 206); 
    }


}
        