use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

use itertools::Itertools;


enum Star {
    One,
    Two,
}



//const DIGITS: HashMap<u8, str> {
//    0: "abcefg",
//    1: "cf",
//    2: "acdeg",
//    3: "acdfg",
//    4: "bcdf",
//    5: "abdfg",
//    6: "abdefg",
//    7: "acf",
//    8: "abcdefg",
//    9: "abcdfg",
//}
//
//const star1_cnt_segments: HashMap<u8, u8> {
//    2: 1,
//    4: 4,
//    3: 7,
//    7: 8,
//}

const ATOG: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];


fn single_out_char(word: &String, replace: &HashMap<char, char>) -> char {
    // prepare replace array
    let expr = replace
        .keys()
        .join("|");

    //println!("Input: {word}, {expr:?}");

    let re = Regex::new(&expr).unwrap();
    
    let replaced = re.replace_all(word, "");
    
    //println!("replaced: {replaced:?}");
    
    //.chars().next().unwrap()
    replaced.chars().next().unwrap()
}


fn deduct_wiring(input: &(Vec<String>, Vec<String>)) -> u32 {
    // get counts of signal-wires
    // b kommt 6 mal vor
    // e kommt 4 mal vor
    // f kommt 9 mal vor
    // c aus 1(2) -f
    // a aus 7(3) - f-c
    // d aus 4(4) - b-c-f
    // g aus 8(7) - a-b-c-d-e-f    

    //println!("signals: {:?}", input.0);

    let codes = HashMap::from([
        ("abcefg", '0'),
        ("cf", '1'),
        ("acdeg", '2'),
        ("acdfg", '3'),
        ("bcdf", '4'),
        ("abdfg", '5'),
        ("abdefg", '6'),
        ("acf", '7'),
        ("abcdefg", '8'),
        ("abcdfg", '9'),
    ]);

    let mut count_sigs = HashMap::new();
    
    for signal in &input.0 {
        for t in ATOG {
            if signal.contains(t) {
                if count_sigs.contains_key(&t) {
                    *count_sigs.get_mut(&t).unwrap() += 1;
                } else {
                    count_sigs.insert(t, 1);
                }                
            }
            
        }
    }
    //println!("count_sigs: {count_sigs:?}");

    let mut transform = HashMap::new();

    for (segment, cnt) in count_sigs {
        match cnt {
            4 => transform.insert(segment, 'e'),
            6 => transform.insert(segment, 'b'),
            9 => transform.insert(segment, 'f'),
            _ => Some('\0'),
        };
    }
    //println!("transform: {transform:?}");

    // first in signals is 1
    let one = &input.0[0];
    transform.insert(
        single_out_char(&one, &transform),
        'c',
    );
    //println!("transform: {transform:?}");
    
    let seven = &input.0[1];
    transform.insert(
        single_out_char(&seven, &transform),
        'a'
    );
    //println!("transform: {transform:?}");
    
    let four = &input.0[2];
    transform.insert(
        single_out_char(&four, &transform),
        'd'
    );
    //println!("transform: {transform:?}");

    let eight = &input.0[9];
    transform.insert(
        single_out_char(&eight, &transform),
        'g'
    );
    //println!("transform: {transform:?}")
    
    let mut number = "".to_string();

    for digit in &input.1 {
        let mut deduct = Vec::new();
        for c in digit.chars() {
            deduct.push(*transform.get(&c).unwrap());
        }
        let x = &deduct.iter().sorted().collect::<String>()[..];
        let c = *codes.get(x).unwrap();

        println!("deduct: {x}, digit: {c}");

        number.push(c);
    
    }

    number.parse::<u32>().unwrap()
    


}




fn read_input(input: &Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    // return array of len of the 4 digits
    let mut output = Vec::new();
        
    for line in input {
        let splitline = line.split_once('|').unwrap();

        let mut signals: Vec<String> = splitline.0
            .trim()
            .split_ascii_whitespace()
            .map(|part| part
                .chars()
                .sorted()
                .collect::<String>()
            )
            .collect();
        
        signals.sort_by(|a, b| a.len().cmp(&b.len()));

        //println!("{signals:?}");

        let digits = splitline.1
            .trim()
            .split_ascii_whitespace()
            .map(|part| part
                .chars()
                .sorted()
                .collect::<String>()
            )
            .collect();

            
        output.push((signals, digits));
    }


    output

}


fn count_digits(input: &Vec<String>, star: Star) -> u32 {
    
    let data = read_input(input);

    if let Star::One = star {
        let mut count = 0;
        for (_, digits) in data {
            for d in digits {
                count += match d.len() {
                    2|3|4|7 => 1,
                    _ => 0,
                };
            }
        }
        return count;
    } else {
        let mut res = 0u32;
        for d in data {
           let s = deduct_wiring(&d);
           println!("{:?}: {s}", d.1);
           res += s;
        }
        return res;
    }
    
}

fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input = reader.lines()
        .filter_map(|line|line.ok())
        .collect::<Vec<String>>();


    println!("Star 1: {}", count_digits(&input, Star::One));
    println!("Star 2: {}", count_digits(&input, Star::Two));

}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_input1() -> Vec<String>{
        vec![
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
            .to_string()]
    }
    
    
    fn get_input2() -> Vec<String>{
       "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        .lines()
        .map(|line|line.to_string())
        .collect()
    }

    //#[test]
    fn star_one1() {
        assert_eq!(count_digits(&get_input1(), Star::One), 0);
    }

    //#[test]
    fn star_one2() {
        assert_eq!(count_digits(&get_input2(), Star::One), 26);
    }


    #[test]
    fn star_two1() {
        assert_eq!(count_digits(&get_input1(), Star::Two), 5353);
    }


    #[test]
    fn star_two2() {
        assert_eq!(count_digits(&get_input2(), Star::Two), 61229);
    }


}
        