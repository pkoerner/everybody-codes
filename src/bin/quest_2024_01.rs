use std::{fs, usize};

const INPUT_FILE_A: &str = "input/everybody_codes_e2024_q01_p1.txt";
const INPUT_FILE_B: &str = "input/everybody_codes_e2024_q01_p2.txt";
const INPUT_FILE_C: &str = "input/everybody_codes_e2024_q01_p3.txt";

fn determine_potions(c: char) -> u32 {
    match c {
        'B' => return 1,
        'C' => return 3,
        'D' => return 5,
        _ => return 0,
    }
}

fn my_partition<T>(mut iter: impl Iterator<Item = T>, n: usize, all: bool) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = Vec::new();

    let mut tmp: Vec<T> = Vec::new();
    while let Some(v) = iter.next() {
        tmp.push(v);
        if tmp.len() >= n {
            res.push(tmp);
            tmp = Vec::new();
        }
    }

    if all && !tmp.is_empty() {
        res.push(tmp)
    }
    res
}

fn determine_potions_group(enemy_group: Vec<char>) -> u32 {
    let sum: u32 = enemy_group.iter().map(|c| determine_potions(*c)).sum();
    let enemies: u32 = enemy_group
        .iter()
        .filter(|c| **c != 'x')
        .count()
        .try_into()
        .unwrap();

    if enemies == 0 {
        return 0;
    }

    sum + ((enemies - 1) * enemies)
}

fn group_enemies_and_sum_potions(contents: String, group_size: usize) -> u32 {
    my_partition(contents.chars(), group_size, true)
        .iter()
        .map(|group| determine_potions_group(group.to_vec()))
        .sum()
}

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE_A).expect("Should have been able to read the file");

    let sum: u32 = group_enemies_and_sum_potions(contents, 1);
    println!("Part a: {}", sum);

    let contents_b =
        fs::read_to_string(INPUT_FILE_B).expect("Should have been able to read the file");
    let sum_b: u32 = group_enemies_and_sum_potions(contents_b, 2);

    println!("Part b: {}", sum_b);

    let contents_c =
        fs::read_to_string(INPUT_FILE_C).expect("Should have been able to read the file");
    let sum_c: u32 = group_enemies_and_sum_potions(contents_c, 3);

    println!("Part c: {}", sum_c);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_partition() {
        let v = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(
            my_partition(v.iter(), 2, true),
            vec![vec![&1, &2], vec![&3, &4], vec![&5, &6]]
        );
    }

    #[test]
    fn test_my_partition2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(
            my_partition(v.iter(), 2, true),
            vec![vec![&1, &2], vec![&3, &4], vec![&5, &6], vec![&7]]
        );
        assert_eq!(
            my_partition(v.iter(), 2, false),
            vec![vec![&1, &2], vec![&3, &4], vec![&5, &6]]
        );
    }

    #[test]
    fn test_my_partition3() {
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(
            my_partition(v.iter(), 3, true),
            vec![vec![&1, &2, &3], vec![&4, &5, &6], vec![&7]]
        );
        assert_eq!(
            my_partition(v.iter(), 3, false),
            vec![vec![&1, &2, &3], vec![&4, &5, &6]]
        );
    }

    #[test]
    fn test_determine_potions_group() {
        assert_eq!(determine_potions_group(vec!['A']), 0);
        assert_eq!(determine_potions_group(vec!['B']), 1);
        assert_eq!(determine_potions_group(vec!['C']), 3);
        assert_eq!(determine_potions_group(vec!['A', 'x']), 0);
        assert_eq!(determine_potions_group(vec!['B', 'C']), 6);
        assert_eq!(determine_potions_group(vec!['D', 'D']), 12);
        assert_eq!(determine_potions_group(vec!['C', 'A']), 5);
        assert_eq!(determine_potions_group(vec!['x', 'D']), 5);
        assert_eq!(determine_potions_group(vec!['x', 'B', 'x']), 1);
        assert_eq!(determine_potions_group(vec!['A', 'A', 'A']), 6);
        assert_eq!(determine_potions_group(vec!['B', 'C', 'D']), 15);
        assert_eq!(determine_potions_group(vec!['x', 'C', 'C']), 8);
    }
}
