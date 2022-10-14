#![feature(iter_collect_into)]

use std::fs::File;
use std::io::{prelude::*, BufReader};
//use std::collections::HashMap;
//use itertools::Itertools;

enum Star {
    One,
    Two,
}



fn score(c: char) -> i32 {
    let score = match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };
    println!("score: {score}");
    score
}


fn process_syntax_line(line: &String) -> i32 {
    let mut stack = Vec::new();
    println!("{line}");
    for c in line.chars() {
        //println!("stack '{}', next: {c}", stack.iter().collect::<String>());
        if "([{<".contains(c) {
            stack.push(c);
        }
        else if let Some(ret_c) = stack.pop() {
            if c != match ret_c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => ret_c,
                    } 
            {
                //println!("stack: '{}', pop '{ret_c}', next '{c}'", stack.iter().collect::<String>());
                return score(c);
            }
        }
        else {
            break;
        }   
    }

    0
}








fn score_syntax(input: &Vec<String>, star: Star) -> i32 {
    let mut score = 0;
    for line in input {
        score += process_syntax_line(line);
    }
    score
}



fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input = reader.lines()
        .filter_map(|line|line.ok())
        .collect::<Vec<String>>();


    println!("Star 1: {}", score_syntax(&input, Star::One));
    //println!("Star 2: {}", score_syntax(&input, Star::Two));

}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_input1() -> Vec<String>{
       "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]"
        .lines()
        .map(|line|line.trim().to_string())
        .collect()
    }
    


    #[test]
    fn star_one1() {
        assert_eq!(score_syntax(&get_input1(), Star::One), 26397);
    }


    //#[test]
    fn star_two1() {
        assert_eq!(score_syntax(&get_input1(), Star::Two), 1134);
    }


}
        
