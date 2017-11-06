extern crate c2bs;

#[macro_use]
extern crate error_chain;

use c2bs::{parser, gen, ErrorKind, Result};

use std::env;
use std::fs;
use std::io::{Read, Write, stderr};

const DEFAULT_STYLE: &str = r#"
\tikzstyle{{base}}=[draw=blue, ultra thick, fill=blue!20, text badly centered]

\tikzstyle{{mh}}=[minimum height=2em]

\tikzstyle{{decision}} = [base, diamond, aspect=2.5, inner sep=0.6]
\tikzstyle{{be}} = [base, rectangle, rounded corners, mh,  text width=4.5em]
\tikzstyle{{block}}=[base, rectangle, mh]
\tikzstyle{{io}}=[base, trapezium, mh, trapezium left angle=60, trapezium right angle=120]

\tikzstyle{{lne}}=[ultra thick]
\tikzstyle{{l}}=[lne, ->]
"#;

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

    let doc = parser::document(buf.as_str()).to_result()?;

    // println!("{:#?}", fc);

    let mut result = String::new();

    gen::gen_document(&doc, DEFAULT_STYLE, &mut result);

    println!("{}", result);

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
