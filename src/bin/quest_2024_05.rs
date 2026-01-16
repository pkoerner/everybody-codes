use std::{collections::{HashMap, HashSet}, fs};

const INPUT_FILE_A: &str = "input/everybody_codes_e2024_q05_p1.txt";
const INPUT_FILE_B: &str = "input/everybody_codes_e2024_q05_p2.txt";
const INPUT_FILE_C: &str = "input/everybody_codes_e2024_q05_p3.txt";

fn parse_input(s : String) -> Vec<Vec<usize>> {
    s.trim().split("\n").map(|line| line.split(" ").map(|num| num.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>()
}

fn transpose<T : Copy>(m : Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res = Vec::new();
    for i in 0..m[0].len() {
        let row = m.iter().map(|col| *col.get(i).unwrap()).collect::<Vec<T>>();
        res.push(row);
    }
    res
}

fn clapper_dance(val : usize, mut v : Vec<usize>) -> Vec<usize> {
    let mut newidx = (val - 1) % (v.len() * 2);
    if newidx > v.len() {
        newidx = (2*v.len()) - newidx
    }
    v.insert(newidx, val);
    v
}

fn clapper_round(round_number : usize, v : & mut Vec<Vec<usize>>) -> usize {
    let idx_from = round_number % v.len();
    let idx_to = (round_number + 1) % v.len();
    let val = v[idx_from][0];
    v[idx_from].remove(0);
    let v_to = &v[idx_to];
    v[idx_to] = clapper_dance(val, v_to.clone());

    let s = v.iter().map(|v| v[0].to_string()).collect::<Vec<String>>().join("");
    s.parse::<usize>().unwrap()
    /* only worked for one-digit numbers
    let mut factor = 1;
    let mut sum = 0;

    for v in v.iter().map(|v| v[0]).rev() {
        sum += v * factor;
        factor *= 10;
    }
    sum*/
}

fn do_clapper_rounds(n : usize, mut v : Vec<Vec<usize>>) -> usize {
    let mut res = 0;
    for idx in 0..n {
        res = clapper_round(idx, &mut v)
    }
    res
}

fn do_clapper_duration(mut v : Vec<Vec<usize>>) -> usize {
    let mut m : HashMap<usize,usize> = HashMap::new();
    let mut round_number = 0;
    loop {
        let res = clapper_round(round_number, &mut v);
        round_number = round_number+1;
        let v = *m.get(&res).or(Some(&0)).unwrap();
        if v+1 == 2024 {
            return res * round_number;
        } else {
            m.insert(res, v+1);
        }
    }
}

fn do_infinite_clapper(mut v : Vec<Vec<usize>>) -> usize {
    let mut m : HashSet<(usize, Vec<Vec<usize>>)> = HashSet::new();
    let mut round_number = 0;
    let mut res_max = 0;
    loop {
        let res = clapper_round(round_number, &mut v);
        round_number += 1;
        
        if res > res_max {
            res_max = res;
        }

        let num_mod = round_number % v.len();
        let vv = v.clone();
        if m.contains(&(num_mod, vv)) {
            return res_max
        }

        m.insert((num_mod, v.clone()));
    }  
}

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE_A).expect("Should have been able to read the file");
    let m = parse_input(contents);
    let m = transpose(m);
    let res = do_clapper_rounds(10,m);
    println!("{}", res);

    let contents =
        fs::read_to_string(INPUT_FILE_B).expect("Should have been able to read the file");
    let m = parse_input(contents);
    let m = transpose(m);
    let res = do_clapper_duration(m);
    println!("{}", res);

        let contents =
        fs::read_to_string(INPUT_FILE_C).expect("Should have been able to read the file");
    let m = parse_input(contents);
    let m = transpose(m);
    let res = do_infinite_clapper(m);
    println!("{}", res)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "2 3 4 5\n\
                           3 4 5 2\n\
                           4 5 2 3\n\
                           5 2 3 4\n".to_string();
        assert_eq!(parse_input(input), vec![vec![2,3,4,5],
                                            vec![3,4,5,2],
                                            vec![4,5,2,3],
                                            vec![5,2,3,4]])
    }

    #[test]
    fn test_transpose() {
        assert_eq!(transpose(vec![vec![2,3,4,5],
                                  vec![3,4,5,2],
                                  vec![4,5,2,3],
                                  vec![5,2,3,4]]),
                   vec![vec![2,3,4,5],
                        vec![3,4,5,2],
                        vec![4,5,2,3],
                        vec![5,2,3,4]]);
    }

    #[test]
    fn test_clapper_dance() {
       assert_eq!(clapper_dance(2, vec![3,4,5,2]), vec![3,2,4,5,2]);
       assert_eq!(clapper_dance(3, vec![4,5,2,3]), vec![4,5,3,2,3]);
       assert_eq!(clapper_dance(4, vec![5,2,3,4]), vec![5,2,3,4,4]);
       assert_eq!(clapper_dance(5, vec![3,4,5]), vec![3,4,5,5]);
    }

    #[test]
    fn test_clapper_round() {
        assert_eq!(clapper_round(0, &mut vec![vec![2,3,4,5],
                                              vec![3,4,5,2],
                                              vec![4,5,2,3],
                                              vec![5,2,3,4]]),
                   3345);
    }

    #[test]
    fn test_clapper_duration() {
        assert_eq!(do_clapper_duration(vec![vec![2,6],
                                            vec![3,7],
                                            vec![4,8],
                                            vec![5,9]]),
                   50877075);
    }

        #[test]
    fn test_infinite_clapper() {
        assert_eq!(do_infinite_clapper(vec![vec![2,6],
                                            vec![3,7],
                                            vec![4,8],
                                            vec![5,9]]),
                   6584);
    }

}