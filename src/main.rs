#[macro_use]
extern crate nom;

#[macro_use]
extern crate error_chain;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        Nom(::nom::Err<u32>);
    }

    errors {
        NoFile {
            description("No argument with file name provided")
        }
    }
}

mod parser;
mod ast;
mod gen;

use std::env;
use std::fs;
use std::io::{Read, Write, stderr};

fn doit() -> Result<()> {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => bail!(ErrorKind::NoFile)
    };

    let mut buf = String::new();
    {
        let mut file = fs::File::open(path)?;
        file.read_to_string(&mut buf)?;
    }

    let fc = parser::flowchart(buf.as_str()).to_result()?;

    // println!("{:#?}", fc);

    println!("{}", gen::gen(&fc));

    Ok(())
}

fn main() {
    let mut stde = stderr();

    if let Err(e) = doit() {
        match *e.kind() {
            ErrorKind::NoFile => writeln!(stde, "{}", e.description()),
            ErrorKind::Nom(_) => writeln!(stde, "Parsing error: {}", e.description()),
            ErrorKind::Io(_) => writeln!(stde, "I/O error: {}", e.description()),
            ErrorKind::Msg(ref msg) => writeln!(stde, "{}", msg),
            _ => writeln!(stde, "Unknown error"),
        }.unwrap();
    }
}
