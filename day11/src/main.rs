use std::fs::File;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::ops::Add;
use itertools::Itertools;

enum Star {
    One,
    Two,
}



#[derive(Debug, Clone, Copy, Default)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coords(
    u8,
    u8,
);

#[derive(Debug, Clone, Default)]
struct Point<'a> {
    coords: Coords,
    state: u8,
    neighbours: Vec<&'a Point<'a>>,
}

#[derive(Debug)]
struct Grid<'a> {
    points: HashMap<Coords, Point<'a>>,
}

impl Grid<'_>{

    const SIZE: u8 = 10;
    const NBS: usize = 8;

    fn new() -> Self {
        let mut grid = Self {
            points:  HashMap::with_capacity((Self::SIZE*Self::SIZE) as usize),
        };
        for y in 0..Self::SIZE {
            for x in 0..Self::SIZE {
                let nb: Point = Point::default();
                let co: Coords = Coords(x,y);
                grid.points.insert(
                    co,
                    Point{
                        coords: co,
                        state: 0,
                        neighbours: Vec::with_capacity(Self::NBS),
                    }
                );
            }
        }
        grid
    }
    
    fn build_neighbours(&mut self) {
         // add neighbours
        for p in self.points.values() {
            let left: u8 = match p.coords.0.checked_sub(1) {
                Some(v) => v,
                _ => p.coords.0,
            };
            let top: u8 = match p.coords.1.checked_sub(1) {
                Some(v) => v,
                _ => p.coords.1,
            };
            let right: u8 = match p.coords.0.add(2) {
                11 => Self::SIZE,
                v => v,
            };
            let bot: u8 = match p.coords.1.add(2) { 
                11 => Self::SIZE,
                v => v,
            };

            for y in top..bot {
                for x in left..right {
                    let nb = self.get(x, y);
                    if nb.coords != p.coords {
                        *p.neighbours.push(nb);
                    }
                }
            }
        }
   }

    fn get(&self, x: u8, y: u8) -> &Point {
        self.points.get(&Coords(x, y)).unwrap()
    }


}

impl fmt::Display for Grid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for y in 0..Self::SIZE {
            for x in 0..Self::SIZE + 1 {
                match char::from_digit(
                        self.get(x,y).state.into(),
                        10) 
                {
                    Some('0') => output.push_str("\x1b[93m0\x1b[0m"),
                    Some(ch) => output.push(ch),
                    None => output.push('x'),
                }
            }
            output.push('\n');
            
        }
        write!(f, "{output}")
    }
}



fn process_input(input: &Vec<String>) -> Grid {
    // return array of len of the 4 digits
    // add borders with height 9 to always be able to check all 4 directions
    let mut grid = Grid::new();
    
    for (y, line) in input.iter().enumerate() {

        for (x, c) in line.chars().enumerate() {
            let mut state = grid.points.get_mut(
                &Coords(
                    x.try_into().unwrap(), 
                    y.try_into().unwrap()
                )
            ).unwrap().state;
            state = u8::try_from(c.to_digit(10).unwrap()).unwrap();
        }
    }
    println!("{grid}");
    grid

}



fn step_flashes1(input: &Vec<String>, steps: u32) -> u32 {
    // prepare
    let mut grid = process_input(&input);
    println!("{grid}");


    for step in 0..steps {
        println!("After step {}\n{grid}", step+1);
    }
    0
}


fn count_flashes(input: &Vec<String>, star: Star) -> u32 {
    match star {
        Star::One => step_flashes1(input, 3),
        Star::Two => 0,
    }
}



fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input = reader.lines()
        .filter_map(|line|line.ok())
        .collect::<Vec<String>>();


    println!("Star 1: {}", count_flashes(&input, Star::One));
    println!("Star 2: {}", count_flashes(&input, Star::Two));
}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_input1() -> Vec<String>{
       "5483143223
       2745854711
       5264556173
       6141336146
       6357385478
       4167524645
       2176841721
       6882881134
       4846848554
       5283751526"
        .lines()
        .map(|line|line.trim().to_string())
        .collect()
    }
    


    #[test]
    fn star_one1() {
        assert_eq!(count_flashes(&get_input1(), Star::One), 26397);
    }


    //#[test]
    fn star_two1() {
        assert_eq!(count_flashes(&get_input1(), Star::Two), 288957);
    }


}
        
