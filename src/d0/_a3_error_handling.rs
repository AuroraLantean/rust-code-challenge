use core::fmt;
use std::fs;
//use std::error::Error;
use thiserror::Error;
/* Anyhow: The downside to using anyhow or Box<dyn Error>> is that we lose the types of errors that are returned by functions and these are often helpful for writing branching logic based on what kind of error occurred.
 */
use anyhow::{Context, Result};
struct Solution;

#[derive(PartialEq, Debug)]
struct Output {
    balance: u32,
    nft_vector: Vec<u32>,
}
impl Output {
    fn new() -> Self {
        Self {
            balance: 0,
            nft_vector: vec![0, 5, 9],
        }
    }
}
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "balance: {}, nft_vector: {:?}",
            self.balance, self.nft_vector
        )
    }
}

/*Thiserror makes it easier to map error types onto your custom enum error types.
thiserror implements std::error::Error
https://nrempel.com/idiomatic-error-handling-in-rust/
*/
#[derive(Error, Debug)]
pub enum OutError {
    #[error("default error")]
    DefaultError,
    #[error("invalid input (expected {expected:?}, found {found:?})")]
    InvalidInput { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
    #[error("file error: {0}")]
    File(#[from] std::io::Error),
    //#[error("download error: {0}")]
    //Download(#[from] reqwest::Error),
} //bail!("Missing attribute: {}", attribute1);

fn str_error(successful: bool) -> Result<Output, &'static str> {
    if successful {
        return Ok(Output::new());
    }
    Err("err_101")
}
fn enum_error(successful: bool) -> Result<Output> {
    if successful {
        return Ok(Output::new());
    }
    Err(OutError::DefaultError.into())
}

fn get_str_error(successful: bool) -> Result<Output, &'static str> {
    println!("--------== a3 err_handling successful= {:?}", successful);
    let value = str_error(successful)?;
    Ok(value)
    /*let value = match result {
    Ok(value) => value,
    Err(err) => {
        return Err(err);
    }  };*/
}
//https://docs.rs/anyhow/latest/anyhow/?ref=nrempel.com
fn get_enum_error(x: i32) -> Result<Output> {
    /*error type is the type you get in the code below
    if there are potential multiple error types, use Box<dyn std::error::Error>
    OR use anyhow::Error as the error type, assuming the error types exposed by libraries have implemented std::error::Error so we can easily convert several different error types to the generic trait object

    use anyhow::Result;//use std::error::Error;
    Use Result<T, anyhow::Error>, or equivalently anyhow::Result<T> */
    let mut output: Output = enum_error(true)?;
    if x == 1 {
        let _file = fs::read_to_string("file.txt")?; //Result<String, std::io::Error> ... to avoid using .map_err(|_| Error::File)?;
    } else if x == 2 {
        output = enum_error(false)?;
    }
    Ok(output)
}
fn get_many_error(_x: i32) -> Result<()> {
    /*
    func1()
    .and_then(func2)
    .and_then(func3)
    .map_err(|e| handle_all_errors(e))?*/
    Ok(())
}

#[test]
fn testa3() {
    let successful = true;
    let out = get_str_error(successful);
    assert_eq!(out.unwrap(), Output::new());

    let successful = false;
    let out = get_str_error(successful);
    println!("error: {:?}", out);
    assert_eq!(out.unwrap_err(), "err_101");

    let x = 0;
    let out = get_enum_error(x);
    assert_eq!(out.unwrap(), Output::new());

    let x = 1;
    let out = get_enum_error(x);
    println!("error: {:?}", out);
    assert_eq!(
        format!("{}", out.unwrap_err().root_cause()),
        "No such file or directory (os error 2)"
    );

    let x = 2;
    let out = get_enum_error(x);
    println!("error: {:?}", out);
    assert_eq!(
        format!("{}", out.unwrap_err().root_cause()),
        "default error"
    ); //https://stackoverflow.com/questions/73841031/how-to-test-the-error-message-from-anyhowerror
}
