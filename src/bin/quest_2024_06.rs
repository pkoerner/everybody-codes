use std::{collections::HashMap, fs};

const INPUT_FILE_A: &str = "input/everybody_codes_e2024_q06_p1.txt";
const INPUT_FILE_B: &str = "input/everybody_codes_e2024_q06_p2.txt";
const INPUT_FILE_C: &str = "input/everybody_codes_e2024_q06_p3.txt";

#[derive(Debug, PartialEq,Clone)]
enum Tree {
    Leaf,
    Branch(String, Vec<Tree>)
}

fn parse_to_map(s : String) -> HashMap<String, Vec<String>> {
    let mut m = HashMap::new();
    for line in s.trim().split("\n") {
        let x : Vec<&str> = line.split(":").collect();
        let k = x[0].to_string();
        let vs : Vec<String> = x[1].split(",").map(|x| x.to_string()).collect();
        m.insert(k, vs);
    }
    m
}

fn map_to_tree(m : &HashMap<String, Vec<String>>, root : String, modify_name : &dyn Fn(String) -> String) -> Tree {
    if root == String::from("@") {
        leaf()
    } else if root == String::from("NOPE") || root == String::from("ANT") || root == String::from("BUG") {
        /* I feel like this is cheating, but not sure if allowed */
        Tree::Branch(root, vec![])
    } else {
        let children : Vec<String> = m.get(&root).or(Some(&vec![String::from("NOPE")])).unwrap().clone();
        let children : Vec<Tree> = children.iter().map(|x| map_to_tree(&m, x.clone(), modify_name)).collect();
        Tree::Branch(modify_name(root), children)
    }
}

fn parse_tree(s : String, modify_name : &dyn Fn(String) -> String) -> Tree {
    let m = parse_to_map(s);
    map_to_tree(&m, String::from("RR"), modify_name)
}

fn prefix_successors(prefix : &String, t : Tree) -> Vec<(String, Tree)> {
    match t {
        Tree::Leaf => vec![],
        Tree::Branch(name, children) 
          => children.into_iter().map(|t| (format!("{prefix}{name}"),t)).collect() //vec![],
    }
}

fn identity<T>(s : T) -> T {
    s
}

fn first_char(s : String) -> String {
    s.chars().nth(0).unwrap().to_string()
}


fn unique_leaf_depth_path(t : Tree) -> String {
    /* TODO: should collect path properly here instead of string */
    let mut frontier = vec![(String::from(""),t)];
    while !frontier.is_empty() {
        frontier = frontier.iter().map(|(prefix,node)| prefix_successors(prefix, node.clone())).flatten().collect();

        let leafs = frontier.iter().filter_map(|(prefix,t)| match t {Tree::Leaf => Some(prefix.clone()), Tree::Branch(_,_ ) => None}).collect::<Vec<String>>();
        if leafs.len() == 1 {
            return format!("{}@", leafs[0])
        }
    }
    return String::from("")
}

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE_A).expect("Should have been able to read the file");
    let result = unique_leaf_depth_path(parse_tree(contents, &identity));
    println!("{result}");

    let contents =
        fs::read_to_string(INPUT_FILE_B).expect("Should have been able to read the file");
    let result = unique_leaf_depth_path(parse_tree(contents, &first_char));
    println!("{result}");

    let contents =
        fs::read_to_string(INPUT_FILE_C).expect("Should have been able to read the file");
    let result = unique_leaf_depth_path(parse_tree(contents, &first_char));
    println!("{result}");
}

fn leaf() -> Tree {
    Tree::Leaf
}

#[cfg(test)]
mod tests {
    use super::*;

    fn branch(name : &str, children : Vec<Tree>) -> Tree {
        Tree::Branch(String::from(name), children)
    }

    #[test]
    fn test_parse() {
        let example = "RR:A,B,C\n\
                             A:D,E\n\
                             B:F,@\n\
                             C:G,H\n\
                             D:@\n\
                             E:@\n\
                             F:@\n\
                             G:@\n\
                             H:@\n";
        assert_eq!(parse_tree(example.to_string(),&identity), 
                   branch("RR", vec![branch("A", vec![branch("D", vec![leaf()]),
                                                      branch("E", vec![leaf()])]),
                                     branch("B", vec![branch("F", vec![leaf()]),
                                                      leaf()]),
                                     branch("C", vec![branch("G", vec![leaf()]),
                                                      branch("H", vec![leaf()])])]));                   
    }

    #[test]
    fn test_unique_leaf_depth_path() {
        let t = branch("RR", vec![branch("A", vec![branch("D", vec![leaf()]),
                                                      branch("E", vec![leaf()])]),
                                     branch("B", vec![branch("F", vec![leaf()]),
                                                      leaf()]),
                                     branch("C", vec![branch("G", vec![leaf()]),
                                                      branch("H", vec![leaf()])])]);
        assert_eq!(unique_leaf_depth_path(t), "RRB@");
    }

}