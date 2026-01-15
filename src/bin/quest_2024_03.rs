use std::{collections::HashMap, fs};

const INPUT_FILE_A: &str = "input/everybody_codes_e2024_q03_p1.txt";
const INPUT_FILE_B: &str = "input/everybody_codes_e2024_q03_p2.txt";
const INPUT_FILE_C: &str = "input/everybody_codes_e2024_q03_p3.txt";


fn griddify_input(s : String) -> HashMap<(usize, usize), i32> {
    let mut m = HashMap::new();

    for (yidx, line) in s.split("\n").enumerate() {
        for (xidx, c) in line.chars().enumerate() {
            let tuple = (xidx+1, yidx+1);
            if c == '#' {
                m.insert(tuple, 0);
            }
        }
    }
    m
}

fn get_neighbours8(x : usize, y : usize) -> Vec<(usize, usize)> {
    [(x-1, y-1), (x-1, y), (x-1, y+1), (x, y-1), (x, y+1), (x+1, y-1), (x+1, y), (x+1, y+1)].iter().cloned().collect::<Vec<(usize,usize)>>()
}
fn get_neighbours4(x : usize, y : usize) -> Vec<(usize, usize)> {
    [(x-1, y), (x+1, y), (x, y-1), (x, y+1)].iter().cloned().collect::<Vec<(usize,usize)>>()
}

fn diggy_hole(m : &HashMap<(usize, usize), i32>, x : usize, y : usize, neighbour_fn : &dyn Fn(usize, usize) -> Vec<(usize,usize)>) -> bool {
    let neighbours = neighbour_fn(x,y);
    let mut vs = neighbours.into_iter().map(|pos| m.get(&pos).or(Some(&0)).unwrap());
    let newv : i32 = m.get(&(x,y)).unwrap() + 1;
    vs.all(|vv| newv -1 <= *vv && *vv <= newv + 1)
}

fn dig_cycle(m : &mut HashMap<(usize, usize), i32>, neighbour_fn : &dyn Fn(usize, usize) -> Vec<(usize, usize)>) -> usize {
    let mut count = 0;
    for (x,y) in m.keys().cloned().collect::<Vec<(usize,usize)>>() {
        if diggy_hole(m, x, y, neighbour_fn) {
            let v = m.get(&(x, y)).unwrap() + 1;
            let tuple = (x, y);
            m.insert(tuple, v);
            count = count + 1;
        }

    }
    count
}

fn dig_operation(m : &mut HashMap<(usize, usize), i32>, neighbour_fn : &dyn Fn(usize, usize) -> Vec<(usize, usize)>) -> usize {
    let mut count = 0;
    loop {
        let newcount = dig_cycle(m, neighbour_fn);
        if newcount == 0 {
            break;
        }
        count += newcount;
    }
    count
}


fn main() {
    let contents = fs::read_to_string(INPUT_FILE_A).expect("Should have been able to read the file");
    let mut input_a = griddify_input(contents);
    println!("{}", dig_operation(&mut input_a, &get_neighbours4));

    let contents_b = fs::read_to_string(INPUT_FILE_B).expect("Should have been able to read the file");
    let mut input_b = griddify_input(contents_b);
    println!("{}", dig_operation(&mut input_b, &get_neighbours4));

    let contents_c = fs::read_to_string(INPUT_FILE_C).expect("Should have been able to read the file");
    let mut input_c = griddify_input(contents_c);
    println!("{}", dig_operation(&mut input_c, &get_neighbours8));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dig_cycle() {
        let example = "..........\n\
                             ..###.##..\n\
                             ...####...\n\
                             ..######..\n\
                             ..######..\n\
                             ...####...\n\
                             ..........";
        let mut grid = griddify_input(String::from(example));
        let count1 = dig_cycle(&mut grid, &get_neighbours4);
        assert_eq!(count1, 25);
        let count2 = dig_cycle(&mut grid, &get_neighbours4);
        assert_eq!(count2, 9);
        let count3 = dig_cycle(&mut grid, &get_neighbours4);
        assert_eq!(count3, 1);
        let count4 = dig_cycle(&mut grid, &get_neighbours4);
        assert_eq!(count4, 0);
    }

    #[test]
    fn test_dig_operation() {
        let example = "..........\n\
                             ..###.##..\n\
                             ...####...\n\
                             ..######..\n\
                             ..######..\n\
                             ...####...\n\
                             ..........";
        let mut grid = griddify_input(String::from(example));
        let count = dig_operation(&mut grid, &get_neighbours4);
        assert_eq!(count, 35);
    }

    #[test]
    fn test_dig_cycle8() {
        let example = "..........\n\
                             ..###.##..\n\
                             ...####...\n\
                             ..######..\n\
                             ..######..\n\
                             ...####...\n\
                             ..........";
        let mut grid = griddify_input(String::from(example));
        let count1 = dig_cycle(&mut grid, &get_neighbours8);
        assert_eq!(count1, 25);
        let count2 = dig_cycle(&mut grid, &get_neighbours8);
        assert_eq!(count2, 4);
        let count3 = dig_cycle(&mut grid, &get_neighbours8);
        assert_eq!(count3, 0);
    }


    #[test]
    fn test_dig_operation8() {
        let example = "..........\n\
                             ..###.##..\n\
                             ...####...\n\
                             ..######..\n\
                             ..######..\n\
                             ...####...\n\
                             ..........";
        let mut grid = griddify_input(String::from(example));
        let count = dig_operation(&mut grid, &get_neighbours8);
        assert_eq!(count, 29);
    }

}