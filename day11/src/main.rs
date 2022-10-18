use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::fmt;

enum Star {
    One,
    Two,
}


#[derive(Default, Debug)]
struct Grid {
    total_flashes: i32,
    points: [[(u8, Vec<(usize,usize)>); Self::SIZE]; Self::SIZE],
}

impl Grid {
    const SIZE: usize = 10;

    fn synchronized_flash(&self) -> bool {
        let mut count = 0;
        for pline in self.points.iter() {
            for p in pline {
                if p.0 == 0 {
                    count += 1;
                }
            }
        }
        match count {
            100 => true,
            _ => false,
        }
    }

    fn add_one(&mut self) -> Vec<(usize,usize)> {
        let mut neighbours = Vec::new();
        for pline in self.points.iter_mut() {
            for p in pline {
                p.0 += 1;
                if p.0 == 10 {
                    p.0 = 11;
                    neighbours.append(&mut p.1.clone());
                    self.total_flashes += 1;
                }
            }
        }
        neighbours
    }

    fn normalize(&mut self) {
        for pline in self.points.iter_mut() {
            for p in pline {
                if p.0 > 10 {
                    p.0 = 0;
                }
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for y in 0..Self::SIZE {
            for x in 0..Self::SIZE {
                match char::from_digit(
                        self.points[x][y].0.into(),
                        10) 
                {
                    Some('0') => output.push_str("\x1b[93m0\x1b[0m"),
                    Some(ch) => output.push(ch),
                    None => output.push_str("\x1b[93mx\x1b[0m"),
                }
            }
            output.push('\n');
            
        }
        write!(f, "{output}")
    }
}





fn build_neighbours(grid: &mut Grid) {
     // add neighbours
    for y in 0..Grid::SIZE {
        for x in 0..Grid::SIZE {
           let left = match x.checked_sub(1) {
                Some(v) => v,
                _ => 0,
            };
            let top = match y.checked_sub(1) {
                Some(v) => v,
                _ => 0,
            };
            let right = if x+1 == Grid::SIZE {
                    x + 1
                } else {
                    x + 2
            };
            let bot = if y+1 == Grid::SIZE {
                    y + 1
                } else {
                    y + 2
            };
            let neighbours = &mut grid.points[x][y].1;
            for ny in top..bot {
                for nx in left..right {
                    if nx != x || ny != y {
                        //println!("point {x},{y}: {left}, {right}, {top}, {bot}");
                        neighbours.push((nx,ny))
                    }
                }
            }
            //println!("{:?} #neigh: {:?}", (x,y), neighbours.len());
        }
    }
}


fn process_input(input: &Vec<String>) -> Grid {
    let mut grid: Grid = Grid::default();
    
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.points[x][y].0 = u8::try_from(c.to_digit(10).unwrap()).unwrap();
        }
    }

    // populate neighbours
    build_neighbours(&mut grid); 
    //println!("{grid:?}");
    grid

}

fn add_one(grid: &mut Grid, neighbours: &mut Vec<(usize,usize)>) {
    let mut cache = Vec::new();
    while let Some(n) = neighbours.pop() {
        let p = &mut grid.points[n.0][n.1];
        p.0 += 1;
        if p.0 == 10 {
            p.0 = 11;
            cache.append(&mut p.1.clone());
            grid.total_flashes += 1;
        }
    }
    neighbours.append(&mut cache);
}

fn step_flashes1(input: &Vec<String>, steps: i32) -> i32 {
    // prepare
    let mut grid = process_input(&input);
    println!("{grid}");


    for step in 0..steps {
        let mut neighbours = grid.add_one();
        
        while neighbours.len() > 0 {
            add_one(&mut grid, &mut neighbours);
        }

        grid.normalize();
        
        println!("After step {}\n{grid}", step + 1);

        if grid.synchronized_flash() {
            return step + 1;
        }

    }
    grid.total_flashes
}


fn count_flashes(input: &Vec<String>, star: Star) -> i32 {
    match star {
        Star::One => step_flashes1(input, 100),
        Star::Two => step_flashes1(input, 999999),
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
    


    //#[test]
    fn star_one1() {
        assert_eq!(count_flashes(&get_input1(), Star::One), 1656);
    }


    #[test]
    fn star_two1() {
        assert_eq!(count_flashes(&get_input1(), Star::Two), 195);
    }


}
        
