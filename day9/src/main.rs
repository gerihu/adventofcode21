#![feature(iter_collect_into)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

use itertools::Itertools;

enum Star {
    One,
    Two,
}





fn process_input(input: &Vec<String>) -> Vec<Vec<u8>> {
    // return array of len of the 4 digits
    // add borders with height 9 to always be able to check all 4 directions
    let horizontal_border = vec![9_u8; input[0].len()+2];
    let mut output = Vec::new();
    
    output.push(horizontal_border.clone());

    for line in input {
        let mut oline = vec!['9'];
        line.chars()
            .collect_into(&mut oline);
        oline.push('9');
        output.push(oline
            .iter()
            .map(|c|
                c
                .to_digit(10)
                .unwrap()
                as u8
            )
            .collect()
        );
    }
    //println!("{output:?}");

    output.push(horizontal_border);



    output

}

fn adjacent_point(p1: &(usize,usize), points: &Vec<(usize, usize)>) -> bool {
    let mut res = false;
    for p2 in points {
        let mut d;
        if p1.1 == p2.1 {
            if p1.0 > p2.0 {
                d = p1.0 - p2.0;
            } else {
                d = p2.0 - p1.0;
            }
            if d == 1 {
                res = true;
                break;
            }
        }

        if p1.0 == p2.0 {
            if p1.1 > p2.1 {
                d = p1.1 - p2.1;
            } else {
                d = p2.1 - p1.1;
            }
            if d == 1 {
                res = true;
                break;
            }
        }
    }
    //println!("Point: {p1:?}, {res}");

    res
}


fn find_lowpoints(input: &Vec<String>, star: Star) -> u32 {
    let point_map = process_input(&input);


    let mut risk: u32 = 0;
    let mut lowpoints = Vec::new();

    for v in 1..point_map.len()-1 {
        for h in 1..point_map[0].len()-1 {
            let cur = point_map[v][h];
            if cur < point_map[v-1][h] 
                    && cur < point_map[v+1][h]
                    && cur < point_map[v][h-1]
                    && cur < point_map[v][h+1] {
                //println!("low: {cur} {},{}", h-1, v-1);
                lowpoints.push((h,v));
                risk = risk + Into::<u32>::into(cur) + 1_u32;
            }

        }
    }
    if let Star::One = star {
        return risk;
    }
    //println!("{point_map:?}");
    //println!("{lowpoints:?}");

    // find basins = all adjacdent points lower9
   
    let mapsize = (point_map[0].len(), point_map.len());

    // walk the map and find points < 9
    let mut basin_points = Vec::new();
    for v in 1..mapsize.1 {
        for h in 1..mapsize.0 {
            if point_map[v][h] < 9 {
                basin_points.push((h, v));
            }
        }
    }

    
    // walk the basin_map and search adjactent points
    let mut basins = Vec::new();
    loop {
        let mut adjacent: Vec<(usize,usize)> = Vec::new();
        loop {
            let start_size = basin_points.len();
            let mut non_ajacent: Vec<(usize,usize)> = Vec::new();
            while let Some(point) = basin_points.pop() {
                if adjacent.len() == 0 {
                    adjacent.push(point);
                    continue;
                }
                if adjacent_point(&point, &adjacent) {
                    adjacent.push(point);            
                } else {
                    non_ajacent.push(point);
                }
            }
            let end_size = non_ajacent.len();
            if end_size > 0 {
                basin_points.append(&mut non_ajacent);
            } 
            
            // check if any points where found - otherwise break
            if end_size == start_size {
                println!("len: {}, {adjacent:?}", adjacent.len());
                println!("basins before: {basins:?}");
                basins.push(adjacent);
                println!("basins after push: {basins:?}");
                break;
            }
        }
        if basin_points.len() == 0 {
            break;
        }
    }


    basins
        .iter()
        .map(|n|n.len() as u32)
        .sorted()
        .rev()
        .take(3)
        .fold(
            1,
            |total, next|
                total*next
            )

}



fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input = reader.lines()
        .filter_map(|line|line.ok())
        .collect::<Vec<String>>();


    //println!("Star 1: {}", find_lowpoints(&input, Star::One));
    println!("Star 2: {}", find_lowpoints(&input, Star::Two));

}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_input1() -> Vec<String>{
       "2199943210
        3987894921
        9856789892
        8767896789
        9899965678"
        .lines()
        .map(|line|line.trim().to_string())
        .collect()
    }
    


    //#[test]
    fn star_one1() {
        assert_eq!(find_lowpoints(&get_input1(), Star::One), 15);
    }


    #[test]
    fn star_two1() {
        assert_eq!(find_lowpoints(&get_input1(), Star::Two), 1134);
    }


}
        