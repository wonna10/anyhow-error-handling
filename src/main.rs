use std::error::Error;
use std::fmt::{Display, format, Formatter};
use anyhow::Context;

#[derive(Debug)]
enum OutOfBoundError {
    Overflow,
    Underflow,
}

impl Display for OutOfBoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for OutOfBoundError {}

impl From<OutOfBoundError> for String {
    fn from(v: OutOfBoundError) -> Self {
        format!("{:?}", v)
    }
}

struct Adder<'a>(&'a i8);

impl<'a> Adder<'a> {
    fn add_one(&self) -> Result<i8, OutOfBoundError> {
        self.0.checked_add(1).ok_or(OutOfBoundError::Overflow)
    }

    fn sub_one(&self) -> Result<i8, OutOfBoundError> {
        self.0.checked_sub(1).ok_or(OutOfBoundError::Underflow)
    }
}

fn always_err() -> anyhow::Result<()> {
    // anyhow::bail!("Hello, Failure")
    Err(anyhow::anyhow!("Hello too"))
}

fn do_it() -> anyhow::Result<()> {
    let v = Adder(&126).add_one()?;
    always_err().context("Additional information")?;
    println!("{}", v);
    Adder(&-128).sub_one()?;
    Ok(())
}


fn main() -> anyhow::Result<()> {
    do_it()?;
    Ok(())
}
