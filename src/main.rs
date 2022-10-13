use std::fmt::format;

#[derive(Debug)]
enum OutOfBoundError {
    Overflow,
    Underflow
}

impl From<OutOfBoundError> for String {
    fn from(v : OutOfBoundError) -> Self {
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

fn always_err() -> Result<(), String> {
    Err("Hello, Failure".into())
}

fn do_it() -> Result<(), String>{
    let v = Adder(&126).add_one()?;
    always_err()?;
    println!("{}", v);
    Adder(&-128).sub_one()?;
    Ok(())
}


fn main() -> Result<(), String> {
    do_it()
}
