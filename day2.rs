use std::collections::HashMap;
use std::fs;

fn main()
{
	let input = fs::read_to_string("day2.txt").expect("day2.txt Not Exist");

	part1(&input);
	part2(&input);
}

fn part1(input : &str)
{
	let mut seen : HashMap<char, u16> = HashMap::new();
	let mut two : u32 = 0;
	let mut three : u32 = 0;

	for line in input.lines()
	{
		for x in line.chars()
		{
			if !seen.contains_key(&x)
			{
				seen.insert(x, 1);
			}
			else
			{
				if let Some(x) = seen.get_mut(&x)
				{
					*x += 1;
				}
			}
		}

		let mut foundtwo = false;
		let mut foundthree = false;
		for (_, val) in seen.iter()
		{
			if !foundtwo && *val == 2
			{
				two += 1;
				foundtwo = true;
				continue;
			}
			else if !foundthree && *val == 3
			{
				three += 1;
				foundthree = true;
				continue;
			}
		}
		seen.clear();
	}
	println!("{}", two * three);
}

fn part2(input : &str)
{
}

fn common_letters(first : &str, second : &str) -> Option<String>
{
	if first.len() != second.len()
	{
		return None;
	}

	let mut diff_seen = false;
	for (c1, c2) in first.chars().zip(second.chars())
	{
		if !diff_seen && c1 != c2
		{
			diff_seen = true;
		}
	}

	Some("".to_string())
}