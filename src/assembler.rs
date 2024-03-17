use multimap::MultiMap;
use chumsky::prelude::*;
use std::{collections::HashMap, env, fmt, fs};
use std::fmt::Write;
use std::io::Read;
use chumsky::container::Container;
use crate::isa::Instructions;
use crate::registry::Registers;

#[derive(Clone, Debug, PartialEq)]
enum Token<'src> {
    Num(i32),
    Str(&'src str),
    Ident(&'src str),
    DefLabel(&'src str),
    Label(&'src str),
    Call,
    Jmp,
    Jze,
    Jzne,
    Ret,
    Add,
    AddNum,
    Sub,
    SubNum,
    Mov,
    MovNum,
    Print,
    End,
}

pub type Span = SimpleSpan<usize>;
impl<'src> fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Num(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "{}", s),
            Token::Ident(s) => write!(f, "{}", s),
            Token::DefLabel(s) => write!(f, "{}", s),
            Token::Label(s) => write!(f, "{}", s),
            Token::Call => write!(f, "CALL"),
            Token::Jmp => write!(f, "JMP"),
            Token::Jze => write!(f, "JZE"),
            Token::Jzne => write!(f, "JZNE"),
            Token::Ret => write!(f, "RET"),
            Token::End => write!(f, "END"),
            Token::Add => write!(f, "ADD"),
            Token::Sub => write!(f, "SUB"),
            Token::SubNum => write!(f, "SUBN"),
            Token::Mov => write!(f, "MOV"),
            Token::MovNum => write!(f, "MOVN"),
            Token::AddNum => write!(f, "ADDN"),
            Token::Print => write!(f, "PRINT"),
        }
    }
}


pub struct Assembly {
}


fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<(Token<'a>, Span)>> {
    // A parser for numbers
    let int = text::int(10).map(|s: &str| Token::Num(s.parse().unwrap()));

    // A parser for strings
    let str_ = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .to_slice()
        .map(Token::Str);

    // Label parser
    let def_label = just("deflabel_")
        .ignore_then(none_of(':').repeated())
        .then_ignore(just(':'))
        .to_slice()
        .map(Token::DefLabel);

    // Label parser
    let label = just("label_")
        .ignore_then(none_of(';').repeated())
        .then_ignore(just(';'))
        .to_slice()
        .map(Token::Label);

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: &str| match ident {
        "MOV" => Token::Mov,
        "MOVN" => Token::MovNum,
        "PRINT" => Token::Print,
        "CALL" => Token::Call,
        "RET" => Token::Ret,
        "JMP" => Token::Jmp,
        "JZE" => Token::Jze,
        "JZNE" => Token::Jzne,
        "END" => Token::End,
        "ADD" => Token::Add,
        "ADDN" => Token::AddNum,
        "SUB" => Token::Sub,
        "SUBN" => Token::SubNum,
        _ => Token::Ident(ident),
    });

    // A single token can be one of the above
    let token = int.or(str_).or(def_label).or(label).or(ident);

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

type ParserInput<'tokens, 'src> =
    chumsky::input::SpannedInput<Token<'src>, Span, &'tokens [(Token<'src>, Span)]>;


fn write_int(res: &mut Vec<u8>, val: usize, start: usize) {
    res[start + 3] = (val & 0xFF) as u8;
    res[start + 2] = (val & 0xFF00) as u8;
    res[start + 1] = (val & 0xFF0000) as u8;
    res[start + 0] = (val & 0xFF000000) as u8;
}

impl Assembly {
    pub fn new() -> Assembly {
        Self {  }
    }

    pub fn parse(&self, input_file: String, output_file: String) {
        let data = fs::read_to_string(&input_file).expect("Failed to read file");
        let mut res = Vec::<u8>::new();
        res.append(&mut Vec::from(0f32.to_be_bytes()));
        let (tokens, mut errs) = lexer().parse(data.as_str()).into_output_errors();
        let mut strings = HashMap::new();
        let mut def_labels = HashMap::new();
        let bindings = tokens.unwrap();
        for (tok, dat) in &bindings {
            match tok {
                Token::Str(str) => {
                    strings.push((str, res.len() as u32));
                    for value in str.bytes().skip(1) {
                        res.push(value);
                    }
                    res.pop();
                },
                _ => {}
            }
        };
        let start = res.len();
        write_int(&mut res, start, 0);

        let mut labels_usage = MultiMap::new();
        for (tok, dat) in &bindings {
            match tok {
                Token::Num(x) => res.append(&mut Vec::from(x.to_be_bytes())),
                Token::Str(str) => res.append(&mut Vec::from(strings[str].to_be_bytes())),
                Token::Ident(val) => res.push(Registers::from_str(val)),
                Token::Call => res.push(Instructions::CALL as u8),
                Token::DefLabel(str) => def_labels.push((&str[3..str.len()-1], res.len())),
                Token::Label(str) => {
                    labels_usage.insert(&str[0..str.len()-1], res.len());
                    res.append(&mut Vec::from(0i32.to_be_bytes()));
                },
                Token::Jmp => res.push(Instructions::JMP as u8),
                Token::Jze => res.push(Instructions::JZE as u8),
                Token::Jzne => res.push(Instructions::JZNE as u8),
                Token::Ret => res.push(Instructions::RET as u8),
                Token::Add => res.push(Instructions::AddReg as u8),
                Token::AddNum => res.push(Instructions::AddNum as u8),
                Token::Sub => res.push(Instructions::SubReg as u8),
                Token::SubNum => res.push(Instructions::SubNum as u8),
                Token::Mov => res.push(Instructions::MovReg as u8),
                Token::End => res.push(Instructions::END as u8),
                Token::MovNum => res.push(Instructions::MovNum as u8),
                Token::Print => res.push(Instructions::Print as u8),
            }
        }
        for (k, v) in labels_usage {
            for start in v {
                write_int(&mut res, def_labels[k], start);
            }
        }
        fs::write(output_file, res).unwrap();
    }

}