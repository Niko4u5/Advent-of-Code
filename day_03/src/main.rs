use std::fs;

fn main() {
    part2(fs::read_to_string("input").unwrap())
}

fn part1(input: String){
    let result: u32 = input.lines().map(|rucksack| {
        let (a,b) = rucksack.split_at(rucksack.len()/2);
        let mut duplicate: char = 'a';
        a.chars().for_each(|char| {
            if b.find(char).is_some(){
                duplicate = char;
            }
        });
        get_priority(duplicate)
   }).sum();

    println!("{}", result);
}

fn part2(input: String){
    let result: u32 = input.lines()
    .collect::<Vec<_>>()
    .chunks(3)
    .map(|rucksacks|{
        let a: &str = rucksacks[0];
        let b: &str = rucksacks[1];
        let c = rucksacks[2];
        let mut char = ' ';
        a.chars().for_each(|d|{
            if b.find(d).is_some() && c.find(d).is_some() {
                char = d;
            }
        });
        get_priority(char)
    }).sum();
    print!("{}", result)
}

fn get_priority(char: char) -> u32{
    if char.is_lowercase(){
        char.to_digit(36).unwrap() - 9
    } else {
        char.to_digit(36).unwrap() - 9 + 26
    }
}