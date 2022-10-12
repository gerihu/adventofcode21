#![feature(iter_collect_into)]


use std::fs::File;
use std::io::{prelude::*, BufReader};



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
    let mut basins = 1u32;
    let mapsize = (point_map[0].len(), point_map.len());
    for p in lowpoints {
        let mut basin = 0;
        //walk down
        for h in (1..p.0+1).rev() {

            if point_map[p.1][h] < 9 {
                println!("h-: {h}, {}: {}", p.1, point_map[p.1][h]);
                basin += 1;
            } else {
                break;
            }

            for v in (1..p.1).rev() {
                if point_map[v][h] < 9 {
                    println!("h-v-: {h}, {v}: {}", point_map[v][h]);
                    basin += 1;
                } else {
                    break;
                }
            }
            for v in p.1+1..mapsize.1 {
                if point_map[v][h] < 9 {
                    println!("h-v+: {}, {v}: {}", h, point_map[v][h]);
                    basin += 1;
                } else {
                    break;
                }
            }
        
        }
        for h in p.0+1..mapsize.0 {
            if point_map[p.1][h] < 9 {
                println!("h+: {h}, {}: {}", p.1, point_map[p.1][h]);
                basin += 1;
            } else {
                break;
            }

            for v in (1..p.1).rev() {
                if point_map[v][h] < 9 {
                    println!("h+v-: {}, {v}: {}", h, point_map[v][h]);
                    basin += 1;
                } else {
                    break;
                }
            }
            for v in p.1+1..mapsize.1 {
                if point_map[v][h] < 9 {
                    println!("h+v+: {}, {v}: {}", h, point_map[v][h]);
                    basin += 1;
                } else {
                    break;
                }
            }        
        }
        println!("point: {p:?}: {basin}");
        basins *= basin;

    }

    basins

}



fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input = reader.lines()
        .filter_map(|line|line.ok())
        .collect::<Vec<String>>();


    println!("Star 1: {}", find_lowpoints(&input, Star::One));
    //println!("Star 2: {}", count_digits(&input, Star::Two));

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
        