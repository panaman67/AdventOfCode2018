use std::error::Error;
use std::fs;

type MyResult<T> = Result<T, Box<Error>>;

fn main() -> MyResult<()>
{
	let input = fs::read_to_string("day8.txt")?;
	let tree : Vec<u8> = input.split_whitespace()
							  .map(|x| x.parse::<u8>().unwrap())
							  .collect();

	part1(tree)?;
	Ok(())
}

fn part1(input : Vec<u8>) -> MyResult<()>
{
	let mut ans : u32 = 0;
	let mut remaining = input.clone();

	while !remaining.is_empty()
	{
		let childs = remaining.remove(0);
		let num_meta = remaining.remove(0);

		if childs == 0
		{
			for _ in 0..num_meta
			{
				ans += remaining.remove(0) as u32;
			}
		}
		else
		{
			for _ in 0..num_meta
			{
				if let Some(ele) = remaining.pop()
				{
					ans += ele as u32;
				}
				else
				{
					println!("Error in popping");
				}
			}
		}
	}
	println!("{}", ans);
	Ok(())
}