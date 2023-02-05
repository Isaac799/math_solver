use std::{io, vec};

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Right,
    Left,
    None,
}

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Float,     // all numbers are treated as floats for simplicity
    Function,  // Only pre-defined functions are allowed
    Separator, // Things like parenthesis or logic
    Operator,  // Things like + - * /
}

#[derive(Debug, Clone)]
struct Token {
    keyword: String,
    token_type: TokenType,
    parameters: usize,
    associativity: Associativity,
    precedence: usize,
}

#[derive(Clone)]
struct KnownToken<'a> {
    keyword: &'a str,
    token_type: TokenType,
    parameters: usize,
    associativity: Associativity,
    precedence: usize,
}

const ACCEPTABLE_SPECIAL: [char; 1] = ['.'];

const KNOWN_TOKENS: [KnownToken; 8] = [
    KnownToken {
        keyword: "+",
        token_type: TokenType::Operator,
        associativity: Associativity::Left,
        parameters: 2,
        precedence: 2,
    },
    KnownToken {
        keyword: "-",
        token_type: TokenType::Operator,
        associativity: Associativity::Left,
        parameters: 2,
        precedence: 2,
    },
    KnownToken {
        keyword: "*",
        token_type: TokenType::Operator,
        associativity: Associativity::Left,
        parameters: 2,
        precedence: 3,
    },
    KnownToken {
        keyword: "/",
        token_type: TokenType::Operator,
        associativity: Associativity::Left,
        parameters: 2,
        precedence: 3,
    },
    KnownToken {
        keyword: "^",
        token_type: TokenType::Operator,
        associativity: Associativity::Right,
        parameters: 2,
        precedence: 4,
    },
    KnownToken {
        keyword: "(",
        token_type: TokenType::Separator,
        associativity: Associativity::None,
        parameters: 0,
        precedence: 1,
    },
    KnownToken {
        keyword: ")",
        token_type: TokenType::Separator,
        associativity: Associativity::None,
        parameters: 0,
        precedence: 1,
    },
    KnownToken {
        keyword: "SUM",
        token_type: TokenType::Function,
        associativity: Associativity::Left,
        parameters: 1,
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
    let mut tokens = create_tokens(&input);
    print_pretty_tokens(&tokens);
    // Create postfix
    let mut postfix_tokens = create_postfix(&mut tokens);
    print_pretty_tokens(&postfix_tokens);

    // Solve postfix
    // let answer: f64 = solve_postfix(&mut postfix_tokens);
    // println!("Answer: {}", answer)
}

fn print_pretty_tokens(tokens: &Vec<Token>) {
    let mut pretty_output: String = String::from("");
    let mut temp: Vec<String> = vec![];
    for x in tokens {
        temp.push(x.keyword.clone().to_string());
    }
    pretty_output = temp.join(" ");
    println!("{:?}", &pretty_output);
}

fn create_tokens(input: &String) -> Vec<Token> {
    println!("\nCreating Tokens...");
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
    let open_p_count: usize = input_chars.iter().filter(|&y| y == &'(').count();
    let close_p_count: usize = input_chars.iter().filter(|&y| y == &')').count();
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

    // Returns the top character from a vector
    fn peek(vec: &Vec<char>) -> &char {
        match vec.last() {
            Some(x) => &x,
            None => {
                panic!("Failed peek!")
            }
        }
    }

    // Converts the cache vector into a token to save and clears the cache
    fn save_and_clear_cache(cache: &mut Vec<char>, save: &mut Vec<Token>) {
        let str_cache = String::from_iter(cache.iter());
        // println!("save_and_clear_cache: {:?}", str_cache);
        cache.clear();

        match KNOWN_TOKENS.iter().find(|e| e.keyword == str_cache) {
            // Know tokens are saved into the answer
            Some(x) => {
                // println!("Provided token is know: {}", str_cache);
                let x = x.clone();
                let y: Token = Token {
                    keyword: String::from(x.keyword),
                    token_type: x.token_type,
                    parameters: x.parameters, // Change to be function accurate?
                    associativity: x.associativity,
                    precedence: x.precedence,
                };
                save.push(y)
            }
            // Unknown tokens are saved if they can be parsed into a float
            None => {
                if str_cache.parse::<f64>().is_ok() {
                    // println!("Provided token assigned TokenType::Float: {}", str_cache);
                    save.push(Token {
                        keyword: str_cache,
                        token_type: TokenType::Float,
                        parameters: 0,
                        associativity: Associativity::None,
                        precedence: 1,
                    });
                } else {
                    panic!("Provided token was unidentifiable!")
                }
            }
        };
    }

    // Ensures a character is precisely a-z or A-Z
    fn is_alphabetic_char(c: &char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z')
    }

    // Ensures a character is precisely 0-9
    fn is_numeric_char(c: &char) -> bool {
        matches!(c, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
    }

    // Ensures a character matches a pre-defined, know token's character
    fn is_known_token(c: &char) -> bool {
        for o in KNOWN_TOKENS {
            if o.keyword == c.to_string() {
                return true;
            }
        }
        false
    }

    // Ensures a character is an acceptable special character, such as decimal
    fn is_acceptable_special_char(c: &char) -> bool {
        for o in ACCEPTABLE_SPECIAL {
            if &o == c {
                return true;
            }
        }
        false
    }

    // Loop through the characters and save tokens for them.
    while input_chars.len() > 0 {
        // println!("");
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

fn create_postfix(tokens: &mut Vec<Token>) -> Vec<Token> {
    println!("\nCreating Postfix...");
    let mut operator_stack: Vec<Token> = vec![];
    let mut output_queue: Vec<Token> = vec![];

    fn pop_and_push(from: &mut Vec<Token>, to: &mut Vec<Token>) {
        let a = match from.pop() {
            Some(b) => b,
            None => panic!("Unable to pop 'from'"),
        };
        to.push(a);
    }

    // Returns a copy of the top token from a vector
    // Makes a copy because I had clashing mut ref to output
    fn peek(vec: &Vec<Token>) -> Token {
        match vec.last() {
            Some(x) => x.clone(),
            None => {
                panic!("Failed peek!")
            }
        }
    }

    // reverse the input tokens so we can pop tokens off as we cycle through them
    tokens.reverse();

    while tokens.len() > 0 {
        // println!("\n\t\toperator_stack: {:?}", operator_stack);

        let x: Token = match tokens.pop() {
            Some(x) => x,
            None => {
                println!("No token found!");
                continue;
            }
        };
        // println!("looking at token: {:?}", x);

        match x.token_type {
            TokenType::Float => {
                // println!("Pushed to output");
                output_queue.push(x)
            }
            TokenType::Function => {
                // println!("Pushed to output");
                output_queue.push(x)
            }
            TokenType::Operator => {
                while (operator_stack.len() > 0)
                    && peek(&operator_stack).keyword != ")"
                    && ((peek(&operator_stack).precedence >= x.precedence)
                        || ((peek(&operator_stack).precedence == x.precedence)
                            && (x.associativity == Associativity::Left)))
                {
                    // println!("Popping and pushing to output -  Operator");
                    pop_and_push(&mut operator_stack, &mut output_queue)
                }
                // println!("Pushing to operator stack -  Operator");
                operator_stack.push(x);
            }
            TokenType::Separator => {
                // println!("X: {:?}", x);
                if x.keyword == "(" {
                    // println!("Pushed to stack - Separator");
                    operator_stack.push(x);
                } else if x.keyword == ")" {
                    while (operator_stack.len() > 0) && (peek(&operator_stack).keyword != "(") {
                        assert!(&operator_stack.len() > &0);
                        // println!("Popping and pushing to output -  Separator");
                        pop_and_push(&mut operator_stack, &mut output_queue)
                    }

                    assert!((operator_stack.len() > 0) && (peek(&operator_stack).keyword == "("));
                    // println!("Popping Separator (");
                    operator_stack.pop();

                    if (operator_stack.len() > 0)
                        && peek(&operator_stack).token_type == TokenType::Function
                    {
                        // println!("Popping and pushing to output - Function");
                        pop_and_push(&mut operator_stack, &mut output_queue)
                    }
                }
            }
        }
    }
    while operator_stack.len() > 0 {
        // println!("remaining operator onto the output queue");
        assert!((operator_stack.len() > 0) && peek(&operator_stack).keyword != "(");
        pop_and_push(&mut operator_stack, &mut output_queue)
    }
    output_queue
}

// fn solve_postfix(tokens: &mut Vec<Token>) -> f64 {
// }
