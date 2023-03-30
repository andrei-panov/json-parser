use std::process::Output;

fn main() {
    let start_medved = start_with("medved");
    let start_preved = start_with("preved");

    let and = vec![start_medved, start_preved];

    for func in and {
        println!("{:?}", func("medved preved"))
    }


    let f1: ParserFunc = Box::new(|input:&str| Ok(("res1", "res2")));

    let x = f1.parse("privet");
    println!("{:?}", x)
}

impl Parser for Box<dyn Fn(&str) -> IResult<&str, &str>> {
    type Input = &'static str;
    type Output = &'static str;

    fn parse(&self, input: &str) -> IResult<Self::Input, Self::Output> {
        println!("вызов метода Парс у парсера!");
        Ok(("","из парсера"))
    }
}

trait Parser {
    type Input;
    type Output;

    fn parse(&self, input: &str) -> IResult<Self::Input,Self::Output>;
}


type ParserFunc = Box<dyn Fn(&str) -> IResult<&str, &str>>;

fn start_with<'t, 'i>(with: &'t str) -> impl Fn(&'i str) -> IResult<&'i str, &'i str> + 't {
    return move |input: &str| {
        return if input.starts_with(&with) {
            Ok((&input[with.len()..], &input[..with.len()]))
        } else {
            Err("не шмогла".to_string())
        };
    };
}

type IResult<IN, OUT> = Result<(IN, OUT), String>;