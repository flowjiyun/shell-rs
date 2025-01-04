// Tokenizer
// state management for tokenizing a string
enum QuoteState {
    None,
    Single,
    Double,
}
pub struct Tokenizer {} 

impl Tokenizer { 
    pub fn tokenize(input: &str) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut cur_token = String::new();
        let mut quote_state = QuoteState::None;

        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            match quote_state {
                QuoteState::Single => {
                    if c == '\'' {
                        quote_state = QuoteState::None;
                    } else {
                        cur_token.push(c);
                    }
                },
                QuoteState::Double => {
                    if c == '\"' {
                        quote_state = QuoteState::None;
                    } else if c == '\\' {
                        match chars.peek() {
                            Some(&next_char) => {
                                if next_char == '\\'
                                    || next_char == '$'
                                    || next_char == '\"'
                                    || next_char == '\n'
                                {
                                    cur_token.push(next_char);
                                    chars.next();
                                } else {
                                    cur_token.push(c);
                                    cur_token.push(next_char);
                                    chars.next();
                                }
                            },
                            None => {
                                cur_token.push(c);
                            }
                        }
                    } else {
                        cur_token.push(c);
                    }
                },
                QuoteState::None => {
                    match c {
                        '\'' => {
                            quote_state = QuoteState::Single;
                        },
                        '\"' => {
                            quote_state = QuoteState::Double;
                        },
                        ' ' => {
                            if !cur_token.is_empty() {
                                tokens.push(cur_token.clone());
                                cur_token.clear();
                            }
                        },
                        '\\' => {
                            match chars.peek() {
                                Some(&next_char) => {
                                    cur_token.push(next_char);
                                    chars.next();
                                },
                                None => {
                                    cur_token.push(c);
                                }
                            }
                        },
                        _ => {
                            cur_token.push(c);
                        }
                    }
                }
            }
        }

        if !cur_token.is_empty() {
            tokens.push(cur_token);
        }

        tokens
    }
}