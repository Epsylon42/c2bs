use ast::*;

use std::cmp::max;

macro_rules! push_many {
    ($dst:expr, $($e:expr)+) => {
        $(
            $dst.push_str($e.as_ref());
        )+
    }
}

pub trait Gen {
    fn gen(&self, dst: &mut String, name: &str, prev: &str) -> (String, String, u8);
}

impl Gen for Node {
    fn gen(&self, dst: &mut String, name: &str, prev: &str) -> (String, String, u8) {
        match *self {
            Node::Block(ref x) => x.gen(dst, name, prev),
            Node::If(ref x) => x.gen(dst, name, prev),
            Node::While(ref x) => x.gen(dst, name, prev),
            Node::For(ref x) => x.gen(dst, name, prev),
            Node::Nodes(ref x) => x.gen(dst, name, prev),
        }
    }
}

impl Gen for Block {
    fn gen(&self, dst: &mut String, name: &str, prev: &str) -> (String, String, u8) {
        push_many!(dst,
                   "\\node[block, below of=" prev "] "

                   "(" name ") "

                   "{" self.text "};\n"
        );

        (name.to_owned(), name.to_owned(), max(1, (self.text.chars().count() / 5) as u8))
    }
}

impl Gen for Vec<Node> {
    fn gen(&self, dst: &mut String, name: &str, prev: &str) -> (String, String, u8) {
        push_many!(dst,
                   "\\coordinate (" name ") at (" prev ".south);\n"
        );

        let (start, mut out_name, mut width) = self.first().unwrap().gen(
            dst,
            format!("{}C0", name).as_str(),
            name
        );

        for (i, node) in self.into_iter().skip(1).enumerate() {
            let (start_new_name, end_new_name, new_width) = node.gen(
                dst,
                format!("{}C{}", name, i+1).as_str(),
                out_name.as_str()
            );

            push_many!(dst,
                       "\\draw[l] (" out_name ") -- (" start_new_name ");\n"
            );

            out_name = end_new_name;
            if new_width > width {
                width = new_width;
            }
        }

        (start, out_name, width)
    }
}

impl Gen for While {
    fn gen(&self, dst: &mut String, name: &str, prev: &str) -> (String, String, u8) {
        push_many!(dst,
                   "\\node[decision, below of=" prev ", node distance=2cm] "

                   "(" name ") "

                   "{" self.cond "};\n"
        );


        let (body_start, body_end, width) = self.body.gen(
            dst,
            format!("{}L", name).as_str(),
            name
        );

        let loop_end = format!("{}LE", name);

        let width_str = format!("{}", width/2);

        push_many!(dst,
                   "\\draw[l] (" name ") -- (" body_start ");\n"

                   "\\draw[l] (" body_end ") -- ++(0,-1) -| ($ (" name ") + (" width_str "cm+1cm,0 ) $) -- (" name ");\n"

                   "\\coordinate[below=1.5cm of " body_end "] (" loop_end ");\n"

                   "\\draw[lne] (" name ") -- node[midway, above] {$-$} ++(-" width_str "cm-1cm,0 ) |- ($ (" loop_end ") + (0,0.5) $) -- (" loop_end ");\n"
        );

        (String::from(name), loop_end, width + 2)
    }
}

impl Gen for For {
    fn gen(&self, dst: &mut String, name: &str, prev: &str) -> (String, String, u8) {
        push_many!(dst,
                   "\\node[block, below of=" prev "] "
                   "(" name ") "
                   "{$" self.var " := " self.from "$};\n"

                   "\\node[decision, below of=" name ", node distance=1.5cm] "
                   "(" name "B) "
                   "{$" self.var " < " self.to "$};\n"

                   "\\draw[l] (" name ") -- (" name "B);\n"
        );

        let (body_start, body_end, width) = self.body.gen(
            dst,
            format!("{}L", name).as_str(),
            format!("{}B", name).as_str()
        );

        push_many!(dst,
                   "\\node[block, below of=" body_end "] "
                   "(" name "FE) "
                   "{$ " self.var " = " self.var " + 1 $};\n"
        );

        let loop_end = format!("{}LE", name);
        let width_str = format!("{}", width/2);

        push_many!(dst,
                   "\\draw[l] (" name "B) -- (" body_start ");\n"
                   "\\draw[l] (" body_end ") -- (" name "FE);\n"

                   "\\draw[l] (" name "FE) -- ++(0,-1) -| ($ (" name "B) + (" width_str "cm+1cm,0 ) $) -- (" name "B);\n"

                   "\\coordinate[below=1.5cm of " name "FE] (" loop_end ");\n"

                   "\\draw[lne] (" name "B) -- node[midway, above] {$-$} ++(-" width_str "cm-1cm,0 ) |- ($ (" loop_end ") + (0,0.5) $) -- (" loop_end ");\n"
        );

        (String::from(name), loop_end, width+2)
    }
}

impl Gen for If {
    fn gen(&self, dst: &mut String, name: &str, prev: &str) -> (String, String, u8) {
        push_many!(dst,
                   "\\node[decision, below of=" prev ", node distance=2cm] "
                   "(" name ") "
                   "{" self.cond "};\n"
        );

        let mut t_str = String::new();
        let mut f_str = String::new();
        let mut width = 0;

        let t_end = {
            let (t_start, t_end, t_width) = self.t.gen(
                &mut t_str,
                format!("{}T", name).as_str(),
                format!("{}TS", name).as_str()
            );

            let t_width_str = format!("{}", t_width/2);

            push_many!(dst,
                       "\\coordinate (" name "TS) at ($ (" name ".east) + (" t_width_str ",-0.5) $);\n"

                       t_str

                       "\\draw[l] (" name ") -| node[midway, above] {+} (" t_start ");\n"
            );

            width += t_width;
            t_end
        };

        let f_end = if let Some(ref f) = self.f {
            let (f_start, f_end, f_width) = f.gen(
                &mut f_str,
                format!("{}F", name).as_str(),
                format!("{}FS", name).as_str()
            );

            let f_width_str = format!("{}", f_width/2);

            push_many!(dst,
                       "\\coordinate (" name "FS) at ($ (" name ".west) + (-" f_width_str ",-0.5) $);\n"

                       f_str

                       "\\draw[l] (" name ") -| node[midway, above] {$-$} (" f_start ");\n"
            );

            width += f_width;
            f_end
        } else {
            push_many!(dst,
                       "\\coordinate (" name "FS) at ($ (" name ".west) + (-0.5,-0.5) $);\n"

                       "\\draw[lne] (" name ") -| node[midway, above] {$-$} (" name "FS);\n"
            );

            width += 2;
            format!("{}FS", name)
        };

        push_many!(dst,
                   "\\path let \\p0 = (" name "), \\p1 = (" t_end "), \\p2 = (" f_end ") in "
                   "coordinate (" name "IE) at (\\x0, {min(\\y1, \\y2)-1cm});\n"

                   "\\draw[lne] (" t_end ") |- (" name "IE);\n"
                   "\\draw[lne] (" f_end ") |- (" name "IE);\n"
        );

        (String::from(name), format!("{}IE", name), width+2+2)
    }
}

pub fn gen_flowchart(chart: &Flowchart, dst: &mut String) {
        push_many!(dst,
                   "\\begin{tikzpicture}[node distance = 1cm]\n"
                   "\\coordinate (START) at (0,0);\n"

                   "\\node[below of=START] (LBL) "
                   "{ \\Large " chart.name "};\n"
        );

        let mut start = "LBL";

        if let Some(ref inp) = chart.input {
            push_many!(dst,
                       "\\node[below of=" start ", node distance=0.5cm] (IN) "
                       "{ \\Large input: " inp "};\n"
            );
            start = "IN";
        }

        if let Some(ref out) = chart.output {
            push_many!(dst,
                       "\\node[below of=" start ", node distance=0.5cm] (OUT) "
                       "{ \\Large output: " out "};\n"
            );
            start = "OUT";
        }

        push_many!(dst,
                   "\\node[be, below of=" start "] (BEGIN) "
                   "{ \\large BEGIN };\n"
        );

        let (body_start, body_end, _) = chart.body.gen(
            dst,
            "A",
            "BEGIN"
        );

        push_many!(dst,
                   "\\draw[l] (BEGIN) -- (" body_start ");\n"

                   "\\node[be, below of=" body_end "] (END) "
                   "{ \\large END };\n"

                   "\\draw[l] (" body_end ") -- (END);\n"

                   "\\end{tikzpicture}\n"
        );
}

//TODO: use something more structured than str for styles
pub fn gen_document(doc: &Document, style: &str, dst: &mut String) {
    push_many!(dst,
               "\\documentclass[tikz]{standalone}\n"

               "\\begin{document}\n"
               "\\usetikzlibrary{shapes, calc, positioning}\n"

               style

               "\n"
    );

    for chart in &doc.0 {
        gen_flowchart(chart, dst);
        dst.push('\n');
    }

    dst.push_str("\\end{document}\n");
}
