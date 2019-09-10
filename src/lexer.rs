    use super::enums::{Token, Operator};
    use std::iter::Peekable;


    /// Converts a shell input into a vector of tokens.
    pub fn tokenize(input: &String, mut tokens: Vec<Token>) {
        let mut input_iter = input.chars().peekable();
        for c in input_iter {
            if c == ';' {
                tokens.push(Token::Operator(Operator::Semicolon));
            } else if c == '<' {
                tokens.push(Token::Operator(Operator::RedirectLeft));
            } else if c == '>' {
                tokens.push(Token::Operator(Operator::RedirectRight));
            } else if c == '&' {
                if input_iter.peek().unwrap_or(&' ') == &'&' {
                    tokens.push(Token::Operator(Operator::And));
                    input_iter.next();
                } else {
                    tokens.push(Token::Operator(Operator::Background));
                }
            } else if c == '|' {
                if *input_iter.peek().unwrap_or(&' ') == '|' {
                    tokens.push(Token::Operator(Operator::Or));
                    input_iter.next();
                } else {
                    tokens.push(Token::Operator(Operator::Pipe));
                }
            } else if c == '\'' {
                let mut s = String::from("\'");
                keep_while(&mut s, |x| x != '\'', &mut input_iter);
                tokens.push(Token::CommandOrArgument(s));
            } else if c == '\"' {
                let mut s = String::from("\"");
                keep_while(&mut s, |x| x != '\"', &mut input_iter);
                tokens.push(Token::CommandOrArgument(s));
            } else if c == '(' {
                let mut s = String::from("(");
                read_until_close_paren(&mut s, &mut input_iter);
                tokens.push(Token::CommandOrArgument(s));
            } else if !c.is_whitespace() {
                let mut s = String::new();
                let closure = |x: char| -> bool {
                    x != '<'
                        || x != '>'
                        || x != '|'
                        || x != '&'
                        || x != ';'
                        || x.is_whitespace()
                        || x != '\''
                        || x != '\"'
                };
                keep_while(&mut s, closure, &mut input_iter);
            }
        }
    }

    fn read_until_close_paren<T>(s: &mut String, iter: &mut Peekable<T>) 
    where T: Iterator<Item = char>
    {
        let mut starts = 1;
        let mut ends = 0;
        let mut sqmode = false;
        let mut dqmode = false;
        for c in iter {
            if sqmode && c == '\'' {
                sqmode = false;
            } else if dqmode && c == '\"' {
                dqmode = false;
            } else if c == ')' {
                ends += 1;
            } else if c == '(' {
                starts += 1;
            } else if c == '\'' {
                sqmode = true;
            } else if c == '\"' {
                dqmode = true;
            }
            s.push(c);
            if starts == ends {
                break;
            }
        }
    }

    /// Return `s` from start until `predicate` is no longer true.
    /// Is there a better error type to use?
    fn keep_while<F, T>(s: &mut String, predicate: F, iter: &mut Peekable<T>)
    where
        F: Fn(char) -> bool,
        T: Iterator<Item = char>,
    {
        for c in iter {
            s.push(c);
            if !predicate(c) {
                break;
            }
        }
    }

    // fn skip_whitespace(s: &str) -> &str {
    //     match s.chars().position(|c| !c.is_whitespace()) {
    //         None => return s,
    //         Some(index) => return &s[index..],
    //     }
    // }


#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn test_tokenize() {}
}