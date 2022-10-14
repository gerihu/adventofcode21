#![feature(iter_collect_into)]

use std::fs::File;
use std::io::{prelude::*, BufReader};




enum Star {
    One,
    Two,
}


fn median(v: &[u64]) -> u64 {
    assert!(v.len() > 0);
    let mut scratch: Vec<&u64> = Vec::with_capacity(v.len());
    scratch.extend(v.iter());
    scratch.sort();
    
    let mid = scratch.len() / 2;
    if scratch.len() % 2 == 1 {
        *scratch[mid]
    } else {
        (*scratch[mid] + *scratch[mid-1]) / 2
    }

}


fn score1(c: char) -> u64 {
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

fn score2(c: &char) -> u64 {
    let score = match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    };
    //println!("score: {score}");
    score
}


fn process_syntax_line1(line: &String) -> u64 {
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
                return score1(c);
            }
        }
        else {
            break;
        }   
    }

    0
}


fn complete_line(stack: &mut Vec<char>) -> Vec<char> {
    let mut complete = Vec::new();
    while let Some(c) = stack.pop() {
        let comp = 
            match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => '\0',
        };
        complete.push(comp);
    }
    complete
}


fn process_syntax_line2(line: &String) -> Option<u64> {
    let mut stack = Vec::new();
    //println!("{line}");
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
                    _ => '\0',
                    } 
            {
                //println!("stack: '{}', pop '{ret_c}', next '{c}'", stack.iter().collect::<String>());
                return None;
            }
        }
        else {
            return None;
        }   
    }
    let complete = complete_line(&mut stack);
    let complete_str: &str = &complete.clone().iter().collect::<String>();
    let score = complete.iter().fold(0, |mult, val| 5 * mult + score2(val));
    
    println!("'{line}', complete: '{complete_str}', score: {score}");
    Some(score)
    
}



fn score_syntax(input: &Vec<String>, star: Star) -> u64 {
    let mut score = 0;
    match star {
        Star::One => {
            for line in input {
                score += process_syntax_line1(line);
            }
        },
        Star::Two => {
            let mut scores = Vec::new();

            for line in input {
                if let Some(s)   = process_syntax_line2(line){
                    scores.push(s);
                }
            }
            println!("{scores:?}");
            score = median(&scores[..]); 
        },
        _ => (),
    };
    
    score
}



fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input = reader.lines()
        .filter_map(|line|line.ok())
        .collect::<Vec<String>>();


    //println!("Star 1: {}", score_syntax(&input, Star::One));
    println!("Star 2: {}", score_syntax(&input, Star::Two));

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
    


    //#[test]
    fn star_one1() {
        assert_eq!(score_syntax(&get_input1(), Star::One), 26397);
    }


    #[test]
    fn star_two1() {
        assert_eq!(score_syntax(&get_input1(), Star::Two), 288957);
    }


}
        
