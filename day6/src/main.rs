use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Fish {
    timer: u8,
}

impl Fish {

    fn step_day(&mut self) -> Option<Fish> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(Fish{timer: 8});
        }
        self.timer -= 1;
        None
    }
}

fn get_fish_states(fish: &Vec<Fish>) -> String {
    let mut all_states = Vec::new();
    for f in fish {
        all_states.push(f.timer);
    }
    all_states.iter().map(|s|s.to_string()).collect::<Vec<String>>().join(&",")
}


fn spawn_lanternfish(input: &Vec<String>, days: u16, star2: bool) -> u64 {

    let fish_states: Vec<u8> = input[0].split(',').filter_map(|num| num.parse::<u8>().ok()).collect();
    let mut fish = Vec::new();
    
    for state in &fish_states {
        fish.push(Fish{timer: *state});
    }

    if fish_states.len() < 10 {
        println!("Initial state: {}", get_fish_states(&fish));
    }
    // step days
    for _i in 0..days {
        let mut new_fish = Vec::new();
        for f in &mut fish {
            if let Some(spawn) = f.step_day() {
                new_fish.push(spawn);
            }
        }
        fish.append(&mut new_fish);
        if fish_states.len() < 10 && _i < 18 {
            println!("After {:>2} day{} {}", _i+1, if _i == 0{": "}else{"s:"}, get_fish_states(&fish));
        }
        println!("Day {_i}: {}", fish.len());
    }

    fish.len().try_into().unwrap()
}


fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let input: Vec<String> = reader.lines().filter_map(|line|line.ok()).collect();

    println!("Star 1: {}", spawn_lanternfish(&input, 80, false));
}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

        
    
    fn get_input() -> Vec<String>{


        vec!["3,4,3,1,2".to_string()]

    }

    #[test]
    fn star_one() {
        assert_eq!(spawn_lanternfish(&get_input(), 80, false), 5934);
    }

    #[test]
    fn star_two() {
        assert_eq!(spawn_lanternfish(&get_input(), 256, false), 26984457539);
    }


}
        