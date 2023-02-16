use std::{fs};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, multispace1, newline, space1, u32,
    },
    multi::{many1, separated_list0},
    sequence::{delimited, preceded},
    *,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part1(&input);
    println!();
    part2(&input);
}


fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(
            complete::char('['),
            alpha1,
            complete::char(']'),
        ),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

fn lines(input: &str) -> IResult<&str, Vec<Option<&str>>>{
    Ok(separated_list0(complete::char(' '), parse_crate)(input)?)
}

fn stacks_lines(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>>{
    Ok(separated_list0(complete::newline,lines)(input)?)
}

fn part1(input: &str){
    let (input, stacks_wrong) = stacks_lines(input).unwrap();
    
    let mut lines = stacks_wrong.iter();
    let mut stacks: Vec<Vec<&str>> = lines.next().unwrap().iter().map(|str| {
        match str {
            Some(str) => vec![*str],
            None => vec![],
        }
    }).collect();

    let mut stacks = lines.fold(stacks, |stacks, str|{
        let mut stacks = stacks.clone();
        stacks.iter_mut().zip(str).for_each(|(stack, str)|{
            if(str.is_some()){
                stack.insert(0, str.unwrap());
            };
        });
        stacks
    });    
    
    let mut moves = input.lines();
    moves.next();
    moves.next();
    let moves = moves.map(|str| parse_move(str).unwrap().1);
    moves.for_each(|(count, from, to)|{
        for _ in 0..count {
            let f = stacks[usize::try_from(from-1).unwrap()].pop().unwrap();
            stacks[usize::try_from(to-1).unwrap()].push(f);
        }
    });
    stacks.iter().for_each(|stack| print!("{}", stack.last().unwrap()))

}

fn part2(input: &str){
    let (input, stacks_wrong) = stacks_lines(input).unwrap();
    
    let mut lines = stacks_wrong.iter();
    let mut stacks: Vec<Vec<&str>> = lines.next().unwrap().iter().map(|str| {
        match str {
            Some(str) => vec![*str],
            None => vec![],
        }
    }).collect();

    let mut stacks = lines.fold(stacks, |stacks, str|{
        let mut stacks = stacks.clone();
        stacks.iter_mut().zip(str).for_each(|(stack, str)|{
            if(str.is_some()){
                stack.insert(0, str.unwrap());
            };
        });
        stacks
    });    
    
    let mut moves = input.lines();
    moves.next();
    moves.next();
    let moves = moves.map(|str| parse_move(str).unwrap().1);
    moves.for_each(|(count, from, to)|{
        let i = stacks[usize::try_from(from-1).unwrap()].len()-(usize::try_from(count).unwrap());
        for _ in 0..count {
            let f = stacks[usize::try_from(from-1).unwrap()].remove(i);
            stacks[usize::try_from(to-1).unwrap()].push(f);
        }
    });
    stacks.iter().for_each(|stack| print!("{}", stack.last().unwrap()))

}

fn parse_move(input: &str) -> IResult<&str, (u32,u32,u32)>{
    let (input, _) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, count) = u32(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, from) = u32(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, to) = u32(input)?;
    Ok((input,(count,from,to)))
}