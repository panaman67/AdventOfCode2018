use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

type GenResult = Result<(), Box<std::error::Error>>;

fn main() -> GenResult
{
    let mut input : String = String::new();
    File::open("day1.txt")?.read_to_string(&mut input)?;

    part1(input.as_str())?;
    part2(input.as_str())?;

    Ok(())
}

fn part1(input : &str) -> GenResult
{
    let mut num : i64 = 0;

    for line in input.lines()
    {
        num += line.parse::<i64>()?;
    }
    println!("{}", num);

    Ok(())
}

fn part2(input : &str) -> GenResult
{
    let mut seen : HashSet<i64> = HashSet::new();
    seen.insert(0);
    let mut num = 0i64;

    'hello : loop
    {
        for line in input.lines()
        {
            num += line.parse::<i64>()?;
            if !seen.contains(&num)
            {
                seen.insert(num);
            }
            else
            {
                break 'hello;
            }
        }
    }
    println!("{}", num);

    Ok(())
}