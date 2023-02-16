use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    
    for i in 14..(input.len()){
        if is_begin(&input[(i-14)..i]) {
            println!("{i}");
            return;
        }
    }
}

fn is_begin(input: &str) -> bool{
    let mut begin: Vec<char>= input[0..14].chars().collect();
    begin.sort();
    begin.dedup();
    if begin.len() < 14 {
        return false;
    }
    true
}