use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::fmt;


enum Star {
    One,
    Two,
}


#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Dot {
    x: u16,
    y: u16,
}


impl Dot {
    fn new(x: &str, y: &str) -> Self {
        Self {
            x: u16::from_str_radix(x, 10).unwrap(),
            y: u16::from_str_radix(y, 10).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Paper {
    dots: HashSet<Dot>,
    folds: Vec<Dot>,
    size: Dot,
}


impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = Vec::new();
        output.push("".to_string());
        for y in 0..self.size.y {
            let mut line = String::new();
            for x in 0..self.size.x {
                if self.dots.contains(&Dot{x, y}) {
                    line.push('#');
                }
                else {
                    line.push('.');
                }
            }
            output.push(line);
        }
        write!(f, "{}\n", output.join("\n"))
    }
}


fn process_input(input: &Vec<String>) -> Paper {
    let mut paper = Paper{
        dots: HashSet::new(),
        folds: Vec::new(),
        size: Dot{x:0, y: 0},
    };
    
    for line in input {
        //println!("{line}");
        if line.is_empty() {
            continue;
        } 
        else if line.starts_with('f'){
            let mut fold = line.split(' ').last().unwrap().split('=');
            let direction = fold.next().unwrap();
            paper.folds.push(
                    match direction {
                        "x" => Dot::new(fold.last().unwrap(), "0"),
                        "y" => Dot::new("0", fold.last().unwrap()),
                        _ => Dot{x: 0, y: 0},
                    }
            );
        }
        else {
            let mut coords = line.split(',');
            let dot = Dot::new(
                    coords.next().unwrap(), 
                    coords.last().unwrap(),
                );
            if dot.x + 1 > paper.size.x {
                paper.size.x = dot.x + 1;
            }
            if dot.y + 1 > paper.size.y {
                paper.size.y = dot.y + 1;
            }
            paper.dots.insert(dot);
        }
    }
    
    paper
    
}


fn fold_paper1(input: &Vec<String>, star: &Star) -> i32 {
    // prepare
    let mut paper = process_input(&input);
    println!("{paper}");

    let so_many = match star {
        Star::One => 1usize,
        Star::Two => usize::MAX,
    };

    for fold in paper.folds.iter().take(so_many) {
        // y: fold up
        // x: fold left

        // get direction
        if fold.y == 0 {
            paper.size.x = fold.x + 1;
            let pos = fold.x;
            // all dots right of x will be moved by their distance to x to the left
            for dot in paper.dots.iter().cloned().collect::<Vec<Dot>>() {
                if dot.x > pos {
                    paper.dots.take(&dot);
                    paper.dots.insert(Dot{x: 2 * pos - dot.x, y: dot.y});
                }
            }
        }
        else {
            paper.size.y = fold.y + 1;
            let pos = fold.y;
            for dot in paper.dots.iter().cloned().collect::<Vec<Dot>>() {
                if dot.y > pos {
                    paper.dots.take(&dot);
                    paper.dots.insert(Dot{x: dot.x, y: 2 * pos - dot.y});
                }
            } 
        }
    }
    println!("{paper}");

    paper.dots.len().try_into().unwrap()
}


fn fold_paper(input: &Vec<String>, star: Star) -> i32 {
    match star {
        Star::One => fold_paper1(input, &star),
        Star::Two => fold_paper1(input, &star),
        //Star::Two => fold_paper1(input, &star),
    }
}



fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input = reader.lines()
        .filter_map(|line|line.ok())
        .collect::<Vec<String>>();


    println!("Star 1: {}", fold_paper(&input, Star::One));
    println!("Star 2: {}", fold_paper(&input, Star::Two));
}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_input1() -> Vec<String>{
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
        .lines()
        .map(|line|line.trim().to_string())
        .collect()
    }
    

 
    #[test]
    fn star_one1() {
        assert_eq!(fold_paper(&get_input1(), Star::One), 17);
    }

    #[test]
    fn star_two() {
        assert_eq!(fold_paper(&get_input1(), Star::Two), 16);
    }



}
        
