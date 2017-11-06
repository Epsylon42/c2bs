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

pub mod parser;
pub mod ast;
pub mod gen;
