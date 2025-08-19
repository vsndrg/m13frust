use std::collections::{HashMap};

enum Expr {
    Var(String),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

fn eval(expr: &Expr, var_tab: &HashMap<String, Option<bool>>) -> bool {
    match expr {
        Expr::Var(name) => {
            var_tab.get(name)
                .expect(&format!("Variable with name {} was not found", name))
                .expect(&format!("Uninitialized variable {}", name))
        }
        Expr::And(a, b) => eval(a, var_tab) && eval(b, var_tab),
        Expr::Or(a, b) => eval(a, var_tab) || eval(b, var_tab),
        Expr::Not(a) => !eval(a, var_tab),
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Ident(String),
    Or,
    And,
    Impl,
    Not,
    LPar,
    RPar,
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut i = 0;
    let mut start;
    let mut tok_list: Vec<Token> = Vec::new();

    while let Some(ch) = expr.chars().nth(i) {
        match ch {
            c if c.is_ascii_alphabetic() => {
                start = i;
                while let Some(next) = expr.chars().nth(i + 1) && next.is_ascii_alphabetic() {
                    i += 1;
                }
                tok_list.push(Token::Ident(expr[start..=i].trim().to_string()));
            }
            '&' | '|' | '-' => {
                i += 1;
                assert_eq!(expr.chars().nth(i).unwrap(), if ch == '-' { '>' } else { ch });
                tok_list.push(if ch == '&' { Token::And } else if ch == '|' { Token::Or } else { Token::Impl });
            }
            '!' => tok_list.push(Token::Not),
            '(' => tok_list.push(Token::LPar),
            ')' => tok_list.push(Token::RPar),
            c if c.is_whitespace() => {}
            _ => panic!("Unexpected symbol: {}", ch)
        }
        i += 1;
    }

    tok_list
}

fn parse_expr(token_list: &mut Vec<Token>) -> Expr {
    parse_impl(token_list)
}

fn parse_impl(token_list: &mut Vec<Token>) -> Expr {
    let mut node = parse_or(token_list);

    while let Some(Token::Impl) = token_list.first() {
        token_list.remove(0);
        let rhs = parse_impl(token_list);
        node = Expr::Or(Box::new(Expr::Not(Box::new(node))), Box::new(rhs));
    }
    node
}

fn parse_or(token_list: &mut Vec<Token>) -> Expr {
    let mut node = parse_and(token_list);

    while let Some(Token::Or) = token_list.first() {
        token_list.remove(0);
        let rhs = parse_and(token_list);
        node = Expr::Or(Box::new(node), Box::new(rhs));
    }
    node
}

fn parse_and(token_list: &mut Vec<Token>) -> Expr {
    let mut node = parse_not(token_list);

    while let Some(Token::And) = token_list.first() {
        token_list.remove(0);
        let rhs = parse_not(token_list);
        node = Expr::And(Box::new(node), Box::new(rhs));
    }
    node
}

fn parse_not(token_list: &mut Vec<Token>) -> Expr {
    if let Some(Token::Not) = token_list.first() {
        token_list.remove(0);
        let inner = parse_not(token_list);
        Expr::Not(Box::new(inner))
    }
    else {
        parse_primary(token_list)
    }
}

fn parse_primary(token_list: &mut Vec<Token>) -> Expr {
    match token_list.remove(0) {
        Token::Ident(name) => Expr::Var(name),
        Token::LPar => {
            let node = parse_impl(token_list);
            assert_eq!(token_list.remove(0), Token::RPar);
            node
        }
        _ => panic!("Unexpected token"),
    }
}

fn collect_vars(expr: &Expr, var_tab: &mut HashMap<String, Option<bool>>) {
    match expr {
        Expr::Var(name) => {
            var_tab.insert(name.clone(), None);
        }
        Expr::And(a, b) | Expr::Or(a, b) => {
            collect_vars(a, var_tab);
            collect_vars(b, var_tab);
        }
        Expr::Not(a) => {
            collect_vars(a, var_tab);
        }
    }
}

pub fn run() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut token_list = tokenize(&input);
    let expr = parse_expr(&mut token_list);

    let mut var_table: HashMap<String, Option<bool>> = HashMap::new();
    collect_vars(&expr, &mut var_table);

    let n = var_table.len();
    let mut counts = [0; 2];
    for bits in 0..(1 << n) {
        for (i, (_, v)) in var_table.iter_mut().enumerate() {
            *v = Some((bits >> (n - 1 - i)) & 1 == 1);
        }
        counts[eval(&expr, &var_table) as usize] += 1;
    }

    match counts {
        [_, 0] => println!("Unsatisfiable"),
        [0, _] => println!("Valid"),
        [f, t] => println!("Satisfiable and falsifiable, {} true and {} false cases", t, f),
    }
}

