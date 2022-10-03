use std::ops::RangeInclusive;
use std::fs::File;
use std::io::{prelude::*, BufReader};



#[derive(Debug)]
struct VentField {
    horizontal: Vec<Vec<u8>>,
    vertical: Vec<Vec<u8>>,
}


impl VentField {
    fn overlaps(&self) -> u32 {
        let mut count_overlap = 0u32;
        for i in 0..self.horizontal.len() {
            for j in 0..self.horizontal.len() {
                if self.horizontal[i][j] + self.vertical[i][j] > 1{
                    count_overlap += 1;
                }
            }
        }
        count_overlap
    }
}

fn get_range(from: usize, to: usize) -> RangeInclusive<usize> {
    if from > to {
        to..=from
    } else {
        from..=to
    }
}


fn process_line(line: &String, vent_field: &mut VentField) {
    let coords: Vec<usize> = line.split([' ', ',']).filter_map(|f| f.parse::<usize>().ok()).collect();

    // 0,9 -> 5,9 horiz
    // 8,0 -> 0,8 ignore
    // 1,1 -> 1,3 vert
    
    if coords[0] == coords[2] { // vertical
        for i in get_range(coords[1], coords[3]){
            vent_field.vertical[i][coords[0]] += 1;
        }
    } else if coords[1] == coords[3] { // horizontal
        for i in get_range(coords[0], coords[2]) {
            vent_field.horizontal[coords[1]][i] += 1;
        }
    }
}


fn find_crossing_vents(input: &Vec<String>, field_size: usize, star2: bool) -> u32 {
    // read input and write to vertical and horizontal matrices
    let mut vent_field = VentField {
        horizontal: vec![vec![0_u8; field_size]; field_size],
        vertical: vec![vec![0_u8; field_size]; field_size],
    };

    for line in input {
        process_line(line, &mut vent_field);
    }

    let overlaps = vent_field.overlaps();
    println!("{overlaps}");
    
    overlaps

}



fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let input: Vec<String> = reader.lines().filter_map(|line|line.ok()).collect();


    println!("Star 1 {}",find_crossing_vents(&input, 1000, false));

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
        assert_eq!(find_crossing_vents(&get_input(), 10, false), 5);
    }

    //#[test]
    fn star_two() {
        assert_eq!(find_crossing_vents(&get_input(), 10, true), 0);
    }


}
        