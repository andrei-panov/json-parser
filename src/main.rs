
fn main() {
    let start_medved = start_with("medved");
    let start_preved = start_with("preved");

    let both: Vec<ParserFunc> = vec![start_medved, start_preved];

    let anyParser = any(both);
    let res = anyParser("preved medved");

    println!("{:?}", &res);
}

type IResult<IN, OUT> = Result<(IN, OUT), String>;
type ParserFunc<'i> = Box<dyn Fn(&'i str) -> IResult<&'i str, &'i str>>;

fn any<'i>(functions: Vec<ParserFunc<'i>>) -> ParserFunc<'i> {
    Box::new(move |input: &'i str| {
        for function in &functions {
            let res: IResult<&str, &str> = function(input);
            if res.is_ok() {
                return res;
            }
        }
        return Err("все плохо!".to_string());
    })
}

fn start_with<'t, 'i>(with: &'t str) -> ParserFunc<'i> {
    Box::new(move |input: &'i str| {
        return if input.starts_with(&with) {
            Ok((&input[with.len()..], &input[..with.len()]))
        } else {
            Err("".to_string())
        };
    })
}

