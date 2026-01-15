use std::fs;

const INPUT_FILE_A: &str = "input/everybody_codes_e2024_q04_p1.txt";
const INPUT_FILE_B: &str = "input/everybody_codes_e2024_q04_p2.txt";
const INPUT_FILE_C: &str = "input/everybody_codes_e2024_q04_p3.txt";

fn to_numbers(s : &str) -> Vec<i32> {
    s.trim().split("\n").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn count_strikes(v : Vec<i32>) -> i32 {
    let min = v.iter().min().unwrap();
    v.iter().map(|x| x - *min).sum()
}

fn median(v : &Vec<i32>) -> i32 {
    let mut v = v.clone();
    v.sort();
    v[v.len()/2]
}

fn count_strikes2(v : Vec<i32>) -> i32 {
    let avg = median(&v);
    v.iter().map(|x| (x - avg).abs()).sum()
}

fn main() {
    let contents = fs::read_to_string(INPUT_FILE_A).expect("Should have been able to read the file");
    let nums = to_numbers(contents.as_str());
    let res = count_strikes(nums);
    println!("{}", res);
  

    let contents = fs::read_to_string(INPUT_FILE_B).expect("Should have been able to read the file");
    let nums = to_numbers(contents.as_str());
    let res = count_strikes(nums);
    println!("{}", res);

    let contents = fs::read_to_string(INPUT_FILE_C).expect("Should have been able to read the file");
    let nums = to_numbers(contents.as_str());
    let res = count_strikes2(nums);
    println!("{}", res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_numbers() {
        assert_eq!(to_numbers("3\n4\n7\n8\n"), vec![3,4,7,8])
    }

    #[test]
    fn test_count_strikes() {
        assert_eq!(count_strikes(vec![3,4,7,8]),10)
    }
    
    #[test]
    fn test_count_strikes2() {
        assert_eq!(count_strikes2(vec![2,4,5,6,8]),8)
    }

    #[test]
    fn test_calc_target() {
        assert_eq!(median(&vec![2,4,5,6,8]),5)
    }

    #[test]
    fn test_calc_target2() {
        assert_eq!(median(&vec![2,2,5]),2)
    }
}