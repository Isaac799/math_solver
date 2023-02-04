use std::io;

#[derive(Debug, Clone)]
pub enum Associativity {
    Right,
    Left,
    None,
}

#[derive(Debug)]
struct Token(String, Associativity, usize);

#[derive(Clone)]
struct KnownToken<'a> {
    value: &'a str,
    associativity: Associativity,
    precedence: usize,
}

const KNOWN_TOKENS: [KnownToken; 9] = [
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
    KnownToken {
        value: "[",
        associativity: Associativity::None,
        precedence: 1,
    },
    KnownToken {
        value: "]",
        associativity: Associativity::None,
        precedence: 1,
    },
];

fn main() {
    let mut input = String::from("");
    println!("My name is Math Bot!");
    println!("Enter a math problem for me to solve:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    // Create tokens
    let tokens = create_tokens(&input);

    println!("{:?}", tokens);
    // Create postfix

    // Solve postfix
}

fn peek(vec: &Vec<char>) -> char {
    match vec.last() {
        Some(x) => x.clone(),
        None => {
            panic!("Failed peek!")
        }
    }
}

fn save_and_clear_cache(cache: &mut Vec<char>, save: &mut Vec<Token>) {
    let str_cache = String::from_iter(cache.iter());
    println!("str_cache: {:?}", str_cache);
    cache.clear();

    match KNOWN_TOKENS.iter().find(|e| e.value == str_cache) {
        Some(x) => {
            let x = x.clone();
            let y: Token = Token(String::from(x.value), x.associativity, x.precedence);
            save.push(y)
        }
        None => save.push(Token(str_cache, Associativity::Left, 1)),
    };
}

fn is_alphabet(c: &char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z')
}
fn is_numeric(c: &char) -> bool {
    matches!(c, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
}

fn create_tokens(input: &String) -> Vec<Token> {
    let mut answer: Vec<Token> = vec![];
    let mut cache: Vec<char> = vec![];

    let input = input.clone();
    let input = input.trim().replace(" ", "");
    let mut input_chars: Vec<char> = input.chars().rev().collect();

    // println!("{:?}", input_chars);

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
        // println!("cache.len: {:?}", cache.len());

        if cache.len() == 0 {
            cache.push(x);
        } else {
            // Number -> Number || Alpha -> Alpha
            if (is_numeric(&x) && is_numeric(&peek(&cache)))
                || (is_alphabet(&x) && is_alphabet(&peek(&cache)))
            {
                println!("Number -> Number || Alpha -> Alpha");
                cache.push(x)
            }
            // Number -> Alpha || Alpha -> Number
            else if (is_numeric(&x) && is_alphabet(&peek(&cache)))
                || (is_alphabet(&x) && is_numeric(&peek(&cache)))
            {
                println!("Number -> Alpha || Alpha -> Number");
                println!("{:?}", cache);
                save_and_clear_cache(&mut cache, &mut answer);
                cache.push(x)
            }
            // Decimal -> Number || Number -> Decimal
            else if (is_numeric(&x) && (peek(&cache) == '.'))
                || (is_numeric(&peek(&cache)) && x == '.')
            {
                println!("Decimal -> Number || Number -> Decimal");
                cache.push(x)
            }
            // Non-Number
            else {
                println!("Non-Number: {:?}", x);
                save_and_clear_cache(&mut cache, &mut answer);
                for o in KNOWN_TOKENS {
                    if o.value.to_string() == x.to_string() {
                        cache.push(x)
                    }
                }
                save_and_clear_cache(&mut cache, &mut answer);
            }
        }
    }
    println!("Pushing remaining cache to answer.");
    println!("cache: {:?}", cache);
    save_and_clear_cache(&mut cache, &mut answer);
    answer
}
// fn create_postfix(tokens: Vec<token>) -> Vec<token> {}
// fn solve_postfix(tokens: Vec<token>) -> f64 {}
