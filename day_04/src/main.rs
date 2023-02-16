use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part1(&input);
}

fn part1(input: &str) {
    let result: u32= input.lines().map(|pair|{
        let mut areas = pair.split(',');
        let mut area1 = areas.next().unwrap().split('-');
        let mut area2 = areas.next().unwrap().split('-');

        let area1start: u32 = area1.next().unwrap().parse().unwrap();
        let area1end: u32 = area1.next().unwrap().parse().unwrap();
        let area2start: u32 = area2.next().unwrap().parse().unwrap();
        let area2end: u32 = area2.next().unwrap().parse().unwrap();
        
        let mut relult = 0;
        if (area1start >= area2start && area1start <= area2end) || // check if 1 begins in 2
        (area1end >= area2start && area1end <= area2end) || // check if 1 ends in 2
        (area2start >= area1start && area2start <= area1end) || // check if 2 begins in 1
        (area2end >= area1start && area2end <= area1end) { // check if 2 ends in 1
            relult = 1;
        } 
        relult
    }).sum();

    println!("{}", result)
}
