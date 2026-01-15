use std::{fs, iter::repeat_n};

const INPUT_FILE_A: &str = "input/everybody_codes_e2024_q02_p1.txt";
const INPUT_FILE_B: &str = "input/everybody_codes_e2024_q02_p2.txt";
const INPUT_FILE_C: &str = "input/everybody_codes_e2024_q02_p3.txt";

fn is_prefix(s : &str, candidate : &str) -> bool {
    s.len() >= candidate.len() && s[0..candidate.len()].eq(candidate)
}

fn find_runic_words(inscription : &Vec<&str>, runic_words : &Vec<&str>) -> i32 {
    let mut count = 0;
    for word in inscription {
        let mut w : &str = word;
        while !w.is_empty() {
            for rword in runic_words.iter() {
                if is_prefix(w, rword) {
                    count += 1;
                }
            }
            w = &(w[1..]);
        }
    }
    count
}

fn find_runic_symbols(inscription : &str, runic_words : &Vec<&str>) -> usize {
    let mut v : Vec<bool> = vec![false; inscription.len()];
    let mut w : &str = inscription;

    let mut idx = 0;
    while !w.is_empty() {
        for rword in runic_words {
            if is_prefix(w, rword) {
                v[idx..idx+rword.len()].copy_from_slice(vec![true; rword.len()].as_slice())
            }
            let revword = rword.chars().rev().collect::<String>();
            if is_prefix(w, &revword) {
                v[idx..idx+rword.len()].copy_from_slice(vec![true; rword.len()].as_slice())
            }
        }
        w = &(w[1..]);
        idx += 1;
    }
    v.iter().filter(|x| **x == true).count()
    
}

fn vector_repeat<T : Clone>(v : Vec<T>, n : usize) -> Vec<T> {
    v.iter().cloned().cycle().take(n * v.len()).collect::<Vec<T>>()
}

fn inscription_to_sphere_view(inscription : &Vec<String>) -> Vec<Vec<char>> {
    let m : Vec<Vec<char>> = inscription.into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let sphere_view_horizontal  = m.into_iter().map(|v| vector_repeat(v, 3)).collect::<Vec<Vec<char>>>();
    //let sphere_view = vector_repeat(sphere_view_horizontal, 3);
    sphere_view_horizontal
}

fn find_words(inscription : &Vec<Vec<char>>, flags : &mut Vec<Vec<bool>>, runic_words : &Vec<&str>) -> () {
    // this is a mess I do not want to talk about it
    let revwords1 = runic_words.clone().iter().map(|w| String::from_iter(w.chars().rev())).collect::<Vec<String>>();
    let revwords = &revwords1.iter().map(|w| w.as_str()).collect::<Vec<&str>>();

    let mut allwords = runic_words.clone();
    allwords.extend(revwords);

    let rowlen = inscription[0].len();
    let collen = inscription.len();
    for y in 0..inscription.len() {
        let row = inscription[y].clone();
        for x in 0.. rowlen {
            // all positions
            for w in &allwords {
                // all words, if can fit
                if x + w.len() < rowlen {
                    let snippet = String::from_iter(row[x..x+w.len()].iter());
                    let wchars = String::from_iter(w.chars().into_iter());
                    if snippet == wchars {
                        flags[y][x..x+w.len()].copy_from_slice(vec![true; w.len()].as_slice())
                    }
                }
                if y + w.len() <= collen {
                    let mut res = true;
                    for (idx, c) in w.chars().into_iter().enumerate() {
                        if inscription[y+idx][x] == c {
                            continue
                        } else {
                            res = false;
                            break;
                        }
                    }
                    if res {
                        for i in 0..w.len() {
                            flags[y+i][x] = true
                        }
                    }
                }
            }
        }
    }
}

fn find_runic_symbols_on_sphere(inscription : &Vec<String>, runic_words : &Vec<&str>) -> usize {
    let sphere_view = inscription_to_sphere_view(inscription);
    let v = repeat_n(false, sphere_view[0].len()).collect::<Vec<bool>>();
    let mut flags = repeat_n(v.clone(), sphere_view.len()).collect::<Vec<Vec<bool>>>();
    
    find_words(&sphere_view, &mut flags, runic_words);

    // debugging code
    /*for (yidx, row) in flags.iter().enumerate() {
        for (xidx, c) in row.iter().enumerate() {
            if *c {
                print!("{}", sphere_view[yidx][xidx])
            } else {
                print!(" ");
            }
            
        }
        println!()
    }*/
    //println!("{:?}", flags);
    flags.iter()
         .map(|v| v[inscription[0].len()..inscription[0].len()*2].iter()
                 .filter(|x|  **x == true)
                 .count())
         .sum()
}

fn split_words(s : &str, c : char) -> Vec<&str> {
    s.split(c).collect()
}

fn process_input(input : &str) -> (Vec<String>, String) {
    let contents =
        fs::read_to_string(input).expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    // Process first line, such as: "Words: foo,bar,baz"
    let split = lines[0].split(':');
    let collect : Vec<&str>= split.collect();
    let s = (collect)[1];
    (lines[2..].iter().map(|x| x.to_string()).collect(), s.to_string())
}

fn main() {
    let (inscription, runic_words) = process_input(INPUT_FILE_A);
    let result = find_runic_words(&split_words(&inscription[0], ' '),
                                       &split_words(&runic_words, ','));
    
    println!("{result}");

    let (inscription2, runic_words2) = process_input(INPUT_FILE_B);
    let actual_runic_words2 = &split_words(&runic_words2, ',');
    let result_b : usize = inscription2.iter().map(|inscr| find_runic_symbols(inscr, actual_runic_words2)).sum();

    println!("{result_b}");

    let (inscription3, runic_words3) = process_input(INPUT_FILE_C);
    let actual_runic_words3 = &split_words(&runic_words3, ',');
    let result_c : usize = find_runic_symbols_on_sphere(&inscription3, actual_runic_words3);
    println!("{result_c}");
}   


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_runic_words() {
        let inscription = vec!["AWAKEN", "THE", "POWER", "ADORNED", "WITH", "THE", "FLAMES", "BRIGHT", "IRE"];
        let runic_words = vec!["THE", "OWE", "MES", "ROD", "HER"];
        assert_eq!(find_runic_words(&inscription, &runic_words), 4);
        let inscription2 = vec!["THE", "FLAME", "SHIELDED", "THE", "HEART", "OF", "THE", "KINGS"];
        assert_eq!(find_runic_words(&inscription2, &runic_words), 3);
        let inscription3 = vec!["POWE", "PO", "WER", "P", "OWE", "R"];
        assert_eq!(find_runic_words(&inscription3, &runic_words), 2);
        let inscription4 = vec!["THERE", "IS", "THE", "END"];
        assert_eq!(find_runic_words(&inscription4, &runic_words), 3);
    }

    #[test]
    fn test_find_runic_symbols() {
        let runic_words = vec!["THE", "OWE", "MES", "ROD", "HER", "QAQ"];
        let inscription = "AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE";
        assert_eq!(find_runic_symbols(inscription, &runic_words), 15);
        let inscription2 = "THE FLAME SHIELDED THE HEART OF THE KINGS";
        assert_eq!(find_runic_symbols(inscription2, &runic_words), 9);
        let inscription3 = "POWE PO WER P OWE R";
        assert_eq!(find_runic_symbols(inscription3, &runic_words), 6);
        let inscription4 = "THERE IS THE END";
        assert_eq!(find_runic_symbols(inscription4, &runic_words), 7);
        let inscription5 = "QAQAQ";
        assert_eq!(find_runic_symbols(inscription5, &runic_words), 5);
    }


    #[test]
    fn test_find_runic_symbols_on_sphere() {
        let runic_words = vec!["THE", "OWE", "MES", "ROD", "RODEO"];
        let inscription = vec![String::from("HELWORLT"), String::from("ENIGWDXL"), String::from("TRODEOAL")];

        assert_eq!(find_runic_symbols_on_sphere(&inscription, &runic_words), 10);

    }
}