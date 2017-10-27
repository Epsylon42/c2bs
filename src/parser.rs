use nom;

use ast::*;

use std::iter;

fn anychar(input: &str) -> nom::IResult<&str, char> {
    if input.len() == 0 {
        nom::IResult::Incomplete(nom::Needed::Size(1))
    } else {
        let c = input.chars().next().unwrap();
        if input.chars().count() == 1 {
            nom::IResult::Done("", c)
        } else {
            nom::IResult::Done(input.split_at(input.char_indices().nth(1).unwrap().0).1, c)
        }
    }
}

named!(escaped_char<&str, char>,
       preceded!(char!('\\'), call!(anychar))
);

named!(pub flowchart<&str, Flowchart>,
       ws!(do_parse!(tag!("flowchart") >>
                     char!('{') >>
                     tag!("name:") >>
                     name: call!(nom::alphanumeric) >>
                     opt!(complete!(char!(';'))) >>
                     input: opt!(complete!(
                         delimited!(
                             tag!("in:"),
                             separated_list_complete!(
                                 call!(nom::space),
                                 is_not_s!("\n")
                             ),
                             opt!(complete!(char!(';')))
                         )
                     )) >>
                     output: opt!(complete!(
                         delimited!(
                             tag!("out:"),
                             separated_list_complete!(
                                 call!(nom::space),
                                 is_not_s!("\n")
                             ),
                             opt!(complete!(char!(';')))
                         )
                     )) >>
                     body: call!(nodes) >>
                     char!('}') >>

                     (Flowchart {
                         name: String::from(name),
                         input:   input.map(|inp| inp.into_iter().flat_map(|i| iter::once(i).chain(iter::once(" "))).collect()),
                         output: output.map(|out| out.into_iter().flat_map(|o| iter::once(o).chain(iter::once(" "))).collect() ),
                         body,
                     })
       ))
);

named!(pub node<&str, Node>,
       ws!(alt_complete!(
           _if => {Node::If} |
           _while => {Node::While} |
           block => {Node::Block}
       ))
);

named!(pub block<&str, Block>,
       map!(
           many1!(
               alt!(
                   none_of!("\\;{}()\n") |
                   escaped_char
               )
           ),
           |s: Vec<char>| Block {text: iter::once('$').chain(s.into_iter()).chain(iter::once('$')).collect()}
       )
);

named!(pub nodes_raw<&str, Vec<Node>>,
       many1!(terminated!(node, opt!(complete!(char!(';')))))
);

named!(pub nodes<&str, Node>,
       map!(
           call!(nodes_raw),
           Node::Nodes
       )
);

named!(_else<&str, Node>,
       ws!(
           do_parse!(tag!("else") >>
                     nds: delimited!(
                         char!('{'),
                         nodes,
                         char!('}')
                     ) >>
                     (nds)
           )
       )
);

named!(pub _if<&str, If>,
       ws!(do_parse!(tag!("if") >>
                     cond: delimited!(
                         char!('('),
                         block,
                         char!(')')
                     ) >>
                     nds: delimited!(
                         char!('{'),
                         nodes,
                         char!('}')
                     ) >>
                     els: opt!(complete!(_else)) >>
                     (If {cond: cond.text, t: Box::new(nds), f: els.map(Box::new)})
       ))
);

named!(pub _while<&str, While>,
       ws!(do_parse!(tag!("while") >>
                     cond: delimited!(
                         char!('('),
                         block,
                         char!(')')
                     ) >>
                     nds: delimited!(
                         char!('{'),
                         nodes,
                         char!('}')
                     ) >>
                     (While {cond: cond.text, body: Box::new(nds)})
       ))
);
