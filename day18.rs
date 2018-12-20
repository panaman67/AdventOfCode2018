use std::error::Error;
use std::fs;

type MyResult = Result<(), Box<Error>>;

fn main() -> MyResult
{
	Ok(())
}