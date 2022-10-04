use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};



#[derive(Debug)]
struct VentField (
    HashMap<u32, u8>,
);


impl VentField {
    fn insert(&mut self, x: u16, y: u16) {
        // try to insert and return count
        let key = u32::from(x) | (u32::from(y) << 16);
        if self.0.contains_key(&key) {
            *self.0.get_mut(&key).unwrap() += 1;
        } else {
            self.0.insert(key, 1);
        }
    }

    fn get(&self, x: u16, y: u16) -> u8 {
        let key = u32::from(x) | (u32::from(y) << 16);
        match self.0.get(&key) {
            Some(v) => *v,
            _ => 0,
        }
    }
}



fn get_range(from: u16, to: u16) -> Vec<u16> {
    if from <= to {
        (from..=to).collect()
    } else {
        (to..=from).rev().collect()
    }

}

fn process_line(line: &String, vent_field: &mut VentField, allow_diagonal: bool ) {
    // we use u16 for the coords as they are limited to 0..1000
    let coords: Vec<u16> = line.split([' ', ',']).filter_map(|f| f.parse::<u16>().ok()).collect();

    // 0,9 -> 5,9 horiz
    // 8,0 -> 0,8 ignore
    // 1,1 -> 1,3 vert

    let x_range = get_range(coords[0], coords[2]);
    let y_range = get_range(coords[1], coords[3]);
    
    if coords[0] == coords[2] { // vertical
        for i in y_range.iter() {
            vent_field.insert(coords[0], *i);
        }
    } else if coords[1] == coords[3] { // horizontal
        for i in x_range.iter() {
            vent_field.insert(*i, coords[1]);
        }
    } else if allow_diagonal && x_range.len() == y_range.len() {
        for (x, y) in x_range.iter().zip(y_range.iter()) {
            vent_field.insert(*x, *y);
        }
    }
}



fn find_crossing_vents(input: &Vec<String>, star2: bool) -> u32 {
    // read input and write to vent points
    let mut vent_field = VentField(
        HashMap::new(),
    );
    
    for line in input {
        process_line(line, &mut vent_field, star2);
    }

    let mut line = String::new();

    for y in 0..=9 {
        for x in 0..=9 {
            let val = vent_field.get(x,y);
            line.push_str(&match val {
                0 => ".".to_string(),
                _ => val.to_string(),
            });
        }
        println!("{y}:{line}");
        line = String::new();
    }
    

    let mut overlaps = 0u32;

    for (_k, v) in vent_field.0 {
        if v > 1 {
            overlaps += 1;
        }
    }
    


    println!("{overlaps}");
    
    overlaps

}



fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let input: Vec<String> = reader.lines().filter_map(|line|line.ok()).collect();


    println!("Star 1 {}",find_crossing_vents(&input, false));
    println!("Star 2 {}",find_crossing_vents(&input, true));

}





#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

        
    
    fn get_input() -> Vec<String>{


        "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2".split('\n').map(|line|line.to_string()).collect()

    }

    #[test]
    fn star_one() {
        assert_eq!(find_crossing_vents(&get_input(), false), 5);
    }

    #[test]
    fn star_two() {
        assert_eq!(find_crossing_vents(&get_input(), true), 12);
    }


}
        