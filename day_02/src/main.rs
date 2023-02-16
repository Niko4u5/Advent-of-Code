use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("file exists");
    let result: u32 = input.lines().map(|round| {
        let shape: u32 = match round.split(' ').last().unwrap() {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!(),
        };
        let game: u32 = match round {
            "A X" => 3,
            "A Y" => 6,
            "A Z" => 0,
            "B X" => 0,
            "B Y" => 3,
            "B Z" => 6,
            "C X" => 6,
            "C Y" => 0,
            "C Z" => 3,
            _ => panic!(),
        };
        shape + game
    }).sum();
    
    println!("{}", result);

    main2();

}

fn main2() {
    let input = fs::read_to_string("input").expect("file exists");
    let result: u32 = input.lines().map(|round| {
        let shape: u32 = match round.split(' ').last().unwrap() {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!(),
        };
        let game: u32 = match round {
            "A X" => 3,
            "A Y" => 1,
            "A Z" => 2,
            "B X" => 1,
            "B Y" => 2,
            "B Z" => 3,
            "C X" => 2,
            "C Y" => 3,
            "C Z" => 1,
            _ => panic!(),
        };
        shape + game
    }).sum();
    
    println!("{}", result)

}
