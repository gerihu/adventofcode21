use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Star {
    One,
    Two,
}





#[derive(Default, Debug)]
struct Graph {
    resources: HashMap<String, (String, Vec<String>)>,
}




fn process_input(input: &Vec<String>) -> Graph {
    let mut graph: Graph = Graph::default();
    
    for line in input {
        let nodes: Vec<String> = line.split('-')
            .map(|x|x.to_string()).collect();
        
        println!("{graph:?}");
        let r = &mut graph.resources;
        
        if let Some(n0) = r.get_mut(&nodes[0]) {
            //println!("push n1 {:?} to {:?}", nodes, n0.1);
            n0.1.push(nodes[1].clone());
        }
        else {
            r.insert(nodes[0].clone(), (nodes[0].clone(), vec![nodes[1].clone()]));
        }
        if let Some(n1) = r.get_mut(&nodes[1]) {
            //println!("push n0 {:?} to {:?}", nodes, n1.1);
            n1.1.push(nodes[0].clone());
        }
        else {
            r.insert(nodes[1].clone(), (nodes[1].clone(), vec![nodes[0].clone()]));
        }
    }


    //println!("{grid:?}");
    graph

}


fn build_linear_path(graph: &Graph, entry_node: &String, path: String) -> i32 {
    if path.len() > 100
        || (path.len() > 0 && *entry_node == "start".to_string()) 
        || (entry_node.to_lowercase() == *entry_node && path.contains(entry_node))
    {
        return 0
    }
    if *entry_node == "end".to_string() {
        println!("{path},{entry_node}");
        return 1;
    }
    let mut new_path = path.clone();
    if new_path.len() > 0 {
        new_path.push(',');
    }
    new_path.push_str(&entry_node.clone());

    let mut count = 0;

    if let Some(node) = graph.resources.get(entry_node) {
        for n in &node.1 {
            count += build_linear_path(graph, n, new_path.clone());
        }
    }
    count
}


fn find_path1(input: &Vec<String>) -> i32 {
    // prepare
    let graph = process_input(&input);

    //first node
    build_linear_path(
        &graph, 
        &"start".to_string(),
        "".to_string())
        
    //println!("{graph:?}");


    
}


fn find_pathes(input: &Vec<String>, star: Star) -> i32 {
    match star {
        Star::One => find_path1(input),
        Star::Two => find_path1(input),
    }
}



fn main() {

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let input = reader.lines()
        .filter_map(|line|line.ok())
        .collect::<Vec<String>>();


    println!("Star 1: {}", find_pathes(&input, Star::One));
    println!("Star 2: {}", find_pathes(&input, Star::Two));
}



#[cfg(test)]
mod tests {
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_input1() -> Vec<String>{
        "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc"
        .lines()
        .map(|line|line.trim().to_string())
        .collect()
    }
    

    fn get_input2() -> Vec<String>{
        "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs"
        .lines()
        .map(|line|line.trim().to_string())
        .collect()
    }
 
    //#[test]
    fn star_one1() {
        assert_eq!(find_pathes(&get_input1(), Star::One), 19);
    }


    #[test]
    fn star_one2() {
        assert_eq!(find_pathes(&get_input2(), Star::One), 226);
    }


}
        
