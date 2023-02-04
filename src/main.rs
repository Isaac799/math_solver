use std::io;

#[derive(Debug, Clone)]
pub enum Associativity {
    Right,
    Left,
    None,
}

#[derive(Debug)]
struct Token {
    value: String,
    associativity: Associativity,
    precedence: usize,
}

#[derive(Clone)]
struct KnownToken<'a> {
    value: &'a str,
    associativity: Associativity,
    precedence: usize,
}

const ACCEPTABLE_SPECIAL: [char; 1] = ['.'];

const KNOWN_TOKENS: [KnownToken; 7] = [
    KnownToken {
        value: "+",
        associativity: Associativity::Left,
        precedence: 2,
    },
    KnownToken {
        value: "-",
        associativity: Associativity::Left,
        precedence: 2,
    },
    KnownToken {
        value: "*",
        associativity: Associativity::Left,
        precedence: 3,
    },
    KnownToken {
        value: "/",
        associativity: Associativity::Left,
        precedence: 3,
    },
    KnownToken {
        value: "^",
        associativity: Associativity::Right,
        precedence: 4,
    },
    KnownToken {
        value: "(",
        associativity: Associativity::None,
        precedence: 1,
    },
    KnownToken {
        value: ")",
        associativity: Associativity::None,
        precedence: 1,
    },
];

fn main() {
    let mut input = String::from("");
    println!("My name is Math Bot!");
    println!("Enter a math problem for me to solve: ");
    match io::stdin().read_line(&mut input) {
        Ok(a) => {}
        Err(_) => {}
    }

    // Create tokens
    let tokens = create_tokens(&input);

    println!("{:?}", tokens);
    // Create postfix

    // Solve postfix
}

fn peek(vec: &Vec<char>) -> &char {
    match vec.last() {
        Some(x) => &x,
        None => {
            panic!("Failed peek!")
        }
    }
}

fn save_and_clear_cache(cache: &mut Vec<char>, save: &mut Vec<Token>) {
    let str_cache = String::from_iter(cache.iter());
    println!("save_and_clear_cache: {:?}", str_cache);
    cache.clear();

    match KNOWN_TOKENS.iter().find(|e| e.value == str_cache) {
        Some(x) => {
            let x = x.clone();
            let y: Token = Token {
                value: String::from(x.value),
                associativity: x.associativity,
                precedence: x.precedence,
            };
            save.push(y)
        }
        None => save.push(Token {
            value: str_cache,
            associativity: Associativity::Left,
            precedence: 1,
        }),
    };
}

fn is_alphabetic_char(c: &char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z')
}
fn is_numeric_char(c: &char) -> bool {
    matches!(c, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
}

fn is_known_token(c: &char) -> bool {
    for o in KNOWN_TOKENS {
        if o.value == c.to_string() {
            return true;
        }
    }
    false
}

fn is_acceptable_special_char(c: &char) -> bool {
    for o in ACCEPTABLE_SPECIAL {
        if &o == c {
            return true;
        }
    }
    false
}

fn create_tokens(input: &String) -> Vec<Token> {
    let mut answer: Vec<Token> = vec![];
    let mut cache: Vec<char> = vec![];
    let input = input
        .trim()
        .replace(" ", "")
        .replace("\n", "")
        .to_ascii_uppercase();

    // reverse the input characters so we can pop chars off as we cycle through them
    let mut input_chars: Vec<char> = input.chars().rev().collect();
    // println!("{:?}", input_chars);

    // Make sure the open and close parenthesis count are the same
    let open_p_count: usize = input_chars.iter().filter(|&y| y.clone() == '(').count();
    let close_p_count: usize = input_chars.iter().filter(|&y| y.clone() == ')').count();
    assert_eq!(open_p_count, close_p_count);

    // Check for valid characters before starting.
    for x in &input_chars {
        if !is_alphabetic_char(&x)
            && !is_numeric_char(&x)
            && !is_known_token(&x)
            && !is_acceptable_special_char(&x)
        {
            panic!("Input can only contain a-z | A-Z | 0-9 | () ")
        }
    }

    while input_chars.len() > 0 {
        println!("");
        let x: char = match input_chars.pop() {
            Some(x) => x,
            None => {
                println!("No character found!");
                break;
            }
        };

        // println!("input_chars.len: {:?}", input_chars.len());

        if cache.len() == 0 {
            cache.push(x);
            continue;
        }
        // Number -> Number || Alpha -> Alpha
        if (is_numeric_char(&x) && is_numeric_char(peek(&cache)))
            || (is_alphabetic_char(&x) && is_alphabetic_char(peek(&cache)))
        {
            cache.push(x)
        }
        // Number -> Alpha || Alpha -> Number
        else if (is_numeric_char(&x) && is_alphabetic_char(peek(&cache)))
            || (is_alphabetic_char(&x) && is_numeric_char(peek(&cache)))
        {
            println!("{:?}", cache);
            save_and_clear_cache(&mut cache, &mut answer);
            cache.push(x)
        }
        // Decimal -> Number || Number -> Decimal
        else if ((peek(&cache) == &ACCEPTABLE_SPECIAL[0]) && is_numeric_char(&x))
            || (is_numeric_char(peek(&cache)) && x == ACCEPTABLE_SPECIAL[0])
        {
            cache.push(x)
        }
        // KnownToken
        else if is_known_token(&x) {
            save_and_clear_cache(&mut cache, &mut answer);
            cache.push(x);
            save_and_clear_cache(&mut cache, &mut answer);
        }
        // KnownToken -> Number || Alpha
        else if is_known_token(peek(&cache)) && (is_numeric_char(&x) || is_alphabetic_char(&x)) {
            save_and_clear_cache(&mut cache, &mut answer);
            cache.push(x);
        }
    }
    if cache.len() > 0 {
        println!("Pushing remaining cache to answer.");
        println!("cache: {:?}", cache);
        save_and_clear_cache(&mut cache, &mut answer);
    }
    answer
}
// fn create_postfix(tokens: Vec<token>) -> Vec<token> {}
// fn solve_postfix(tokens: Vec<token>) -> f64 {}
