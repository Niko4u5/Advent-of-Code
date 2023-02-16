use std::{fs};

fn main() {
    let input = "input";

    let input_text = fs::read_to_string(input)
        .expect("Should have been able to read the file");
    let elfs = input_text.split("\n\n").map(
        |elf| {elf.split("\n").map(
            |cal|{cal.parse::<i32>().unwrap_or(0)}).sum::<i32>()});
    let mut list: Vec<i32> = elfs.collect();
    list.sort();
    println!("{}", list.last().unwrap());

    let mut sum = 0;
    if list.len() > 3{
        sum = list.split_at(list.len()-3).1.iter().sum();
    }

    println!("{}", sum)
}
