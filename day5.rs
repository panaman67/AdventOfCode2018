use std::error::Error;
use std::fs;

type MyResult<T> = Result<T, Box<Error>>;

fn main() -> MyResult<()>
{
	let input = fs::read_to_string("test.txt")?;
	let polymer : Vec<char> = input.chars().collect();

	part1(polymer)?;

	Ok(())
}

fn part1(input : Vec<char>) -> MyResult<()>
{
	// let test : Vec<&[char]> = input.chunks(2)
	// 								.filter(|x| !x.is_empty())
	// 								.collect();

	let test : Vec<&[char]> = input.chunks(2)
									.filter(|x| x.is_empty())
									.collect();

	Ok(())
}