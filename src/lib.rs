use pp_rs::{
    pp::Preprocessor,
    token::{Punct, TokenValue},
};

pub fn preprocess(input: &str) -> Result<String, ()> {
    let preprocessor = Preprocessor::new(input);
    let mut output = String::new();

    let mut prev_token_needs_space = false;

    for token in preprocessor {
        let token = token.unwrap();

        // Add space between consecutive idents, integers, or floats
        match token.value {
            TokenValue::Ident(_) | TokenValue::Integer(_) | TokenValue::Float(_) => {
                if prev_token_needs_space {
                    output.push_str(" ")
                }
                prev_token_needs_space = true;
            }
            _ => prev_token_needs_space = false,
        }

        match token.value {
            TokenValue::Ident(ident) => {
                output.push_str(&ident);
            }
            TokenValue::Integer(integer) => {
                output.push_str(&format!("{}", integer.value));
            }
            TokenValue::Float(float) => {
                output.push_str(&format!("{}", float.value));
                if float.value.fract() == 0.0 {
                    output.push_str(".0");
                }
            }
            TokenValue::Punct(punct) => match punct {
                Punct::AddAssign => {
                    output.push_str(" += ");
                }
                Punct::SubAssign => output.push_str(" -= "),
                Punct::MulAssign => output.push_str(" *= "),
                Punct::DivAssign => output.push_str(" /= "),
                Punct::ModAssign => output.push_str(" %= "),
                Punct::LeftShiftAssign => output.push_str(" <<= "),
                Punct::RightShiftAssign => output.push_str(" >>= "),
                Punct::AndAssign => output.push_str(" &= "),
                Punct::XorAssign => output.push_str(" ^= "),
                Punct::OrAssign => output.push_str(" |= "),
                Punct::Increment => output.push_str(" ++ "),
                Punct::Decrement => output.push_str(" -- "),
                Punct::LogicalAnd => output.push_str(" && "),
                Punct::LogicalOr => output.push_str(" || "),
                Punct::LogicalXor => output.push_str(" ^^ "),
                Punct::LessEqual => output.push_str(" <= "),
                Punct::GreaterEqual => output.push_str(" >= "),
                Punct::EqualEqual => output.push_str(" == "),
                Punct::NotEqual => output.push_str(" != "),
                Punct::LeftShift => output.push_str(" << "),
                Punct::RightShift => output.push_str(" >> "),
                Punct::LeftBrace => output.push_str("{"),
                Punct::RightBrace => output.push_str("}"),
                Punct::LeftParen => output.push_str("("),
                Punct::RightParen => output.push_str(")"),
                Punct::LeftBracket => output.push_str("["),
                Punct::RightBracket => output.push_str("]"),
                Punct::LeftAngle => output.push_str(" < "),
                Punct::RightAngle => output.push_str(" > "),
                Punct::Semicolon => output.push_str(";"),
                Punct::Comma => output.push_str(", "),
                Punct::Colon => output.push_str(": "),
                Punct::Dot => output.push_str("."),
                Punct::Equal => output.push_str(" = "),
                Punct::Bang => output.push_str(" !"),
                Punct::Minus => output.push_str(" - "),
                Punct::Tilde => output.push_str(" ~"),
                Punct::Plus => output.push_str(" + "),
                Punct::Star => output.push_str(" * "),
                Punct::Slash => output.push_str(" / "),
                Punct::Percent => output.push_str(" % "),
                Punct::Pipe => output.push_str(" | "),
                Punct::Caret => output.push_str(" ^ "),
                Punct::Ampersand => output.push_str(" & "),
                Punct::Question => output.push_str(" ? "),
            },
            TokenValue::Version(version) => {
                output.push_str("#version ");
                for token in version.tokens {
                    match token.value {
                        TokenValue::Integer(integer) => {
                            output.push_str(&format!("{}", integer.value))
                        }
                        TokenValue::Ident(ident) => {
                            if ident == "es" {
                                output.push_str(" es")
                            } else {
                                panic!()
                            }
                        }
                        _ => panic!(),
                    }
                }
                output.push_str("\n");
            }
            TokenValue::Extension(extension) => {
                output.push_str("#extension ");
                match &extension.tokens[0].value {
                    TokenValue::Ident(name) => output.push_str(name),
                    _ => panic!(),
                }
                match &extension.tokens[1].value {
                    TokenValue::Punct(Punct::Colon) => output.push_str(" : "),
                    _ => panic!(),
                }
                match &extension.tokens[2].value {
                    TokenValue::Ident(behaviour) => output.push_str(&behaviour),
                    _ => panic!(),
                }
                output.push_str("\n");
            }
            TokenValue::Pragma(_) => unimplemented!(),
        }
    }

    Ok(output)
}
