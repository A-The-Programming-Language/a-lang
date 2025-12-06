//! # A-lang Lexer
//!
//! Tokenization for A-lang with support for:
//! - Standard scripting constructs
//! - Reactive variable declarations
//! - Time-travel debugging commands
//! - Syntax extension definitions
//! - Parallel execution hints

use logos::Logos;
use std::fmt;

/// Token types for A-lang
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
pub enum Token {
    // Keywords
    #[token("let")]
    Let,

    #[token("const")]
    Const,

    #[token("reactive")]
    Reactive,

    #[token("var")]
    Var,

    #[token("fn")]
    Fn,

    #[token("return")]
    Return,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("elif")]
    Elif,

    #[token("for")]
    For,

    #[token("while")]
    While,

    #[token("in")]
    In,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("match")]
    Match,

    #[token("struct")]
    Struct,

    #[token("enum")]
    Enum,

    #[token("class")]
    Class,

    #[token("new")]
    New,

    #[token("this")]
    This,

    #[token("super")]
    Super,

    #[token("extends")]
    Extends,

    #[token("import")]
    Import,

    #[token("export")]
    Export,

    #[token("try")]
    Try,

    #[token("catch")]
    Catch,

    #[token("finally")]
    Finally,

    #[token("throw")]
    Throw,

    #[token("async")]
    Async,

    #[token("await")]
    Await,

    // WOW Factor 1: Time-Travel Debugging
    #[token("snapshot")]
    Snapshot,

    #[token("rewind")]
    Rewind,

    #[token("replay")]
    Replay,

    #[token("checkpoint")]
    Checkpoint,

    // WOW Factor 2: Reactive System
    #[token("watch")]
    Watch,

    #[token("effect")]
    Effect,

    #[token("computed")]
    Computed,

    #[token("signal")]
    Signal,

    // WOW Factor 3: Syntax Extensions
    #[token("syntax")]
    Syntax,

    #[token("macro")]
    Macro,

    #[token("quote")]
    Quote,

    #[token("unquote")]
    Unquote,

    // WOW Factor 4: Parallelization
    #[token("parallel")]
    Parallel,

    #[token("concurrent")]
    Concurrent,

    #[token("atomic")]
    Atomic,

    #[token("sync")]
    Sync,

    // WOW Factor 5: Context-Aware Types
    #[token("context")]
    Context,

    #[token("infer")]
    Infer,

    #[token("adapt")]
    Adapt,

    #[token("type")]
    Type,

    #[token("interface")]
    Interface,

    // Literals
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),

    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        let inner = &s[1..s.len()-1];
        // Process escape sequences
        let mut result = String::new();
        let mut chars = inner.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some(other) => {
                        result.push('\\');
                        result.push(other);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(c);
            }
        }
        result
    })]
    String(String),

    // Template strings with backticks
    #[regex(r"`([^`\\]|\\.)*`", |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    TemplateString(String),

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("nil")]
    Nil,

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("**")]
    Power,

    #[token("=")]
    Assign,

    // Compound assignment operators
    #[token("+=")]
    PlusAssign,

    #[token("-=")]
    MinusAssign,

    #[token("*=")]
    StarAssign,

    #[token("/=")]
    SlashAssign,

    #[token("%=")]
    PercentAssign,

    // Increment/Decrement operators
    #[token("++")]
    Increment,

    #[token("--")]
    Decrement,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token("<")]
    Less,

    #[token("<=")]
    LessEqual,

    #[token(">")]
    Greater,

    #[token(">=")]
    GreaterEqual,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    #[token("&")]
    BitwiseAnd,

    #[token("|")]
    BitwiseOr,

    #[token("^")]
    BitwiseXor,

    #[token("<<")]
    LeftShift,

    #[token(">>")]
    RightShift,

    // Reactive operators (removed <- in favor of =)
    #[token("->")]
    Arrow,

    #[token("=>")]
    FatArrow,

    #[token("~>")]
    StreamPipe,

    #[token("|>")]
    Pipe,

    // Delimiters
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("..")]
    DotDot,

    #[token("...")]
    DotDotDot,

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    #[token(";")]
    Semicolon,

    #[token("?")]
    Question,

    #[token("@")]
    At,

    #[token("#")]
    Hash,

    #[token("$")]
    Dollar,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Let => write!(f, "let"),
            Token::Const => write!(f, "const"),
            Token::Reactive => write!(f, "reactive"),
            Token::Fn => write!(f, "fn"),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Nil => write!(f, "nil"),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Tokenize source code into a vector of tokens with positions
pub fn tokenize(source: &str) -> Result<Vec<(Token, std::ops::Range<usize>)>, LexerError> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(source);

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => {
                tokens.push((token, lexer.span()));
            }
            Err(_) => {
                return Err(LexerError {
                    message: format!("Unexpected token at position {}", lexer.span().start),
                    span: lexer.span(),
                });
            }
        }
    }

    Ok(tokens)
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub span: std::ops::Range<usize>,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {:?}", self.message, self.span)
    }
}

impl std::error::Error for LexerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "let x = 42;";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].0, Token::Let);
        assert_eq!(tokens[1].0, Token::Identifier("x".to_string()));
        assert_eq!(tokens[2].0, Token::Assign);
        assert_eq!(tokens[3].0, Token::Integer(42));
        assert_eq!(tokens[4].0, Token::Semicolon);
    }

    #[test]
    fn test_reactive_tokens() {
        let source = "reactive counter = 0;";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens[0].0, Token::Reactive);
        assert_eq!(tokens[2].0, Token::Assign);
    }

    #[test]
    fn test_time_travel_tokens() {
        let source = "snapshot state; rewind 5;";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens[0].0, Token::Snapshot);
        assert_eq!(tokens[1].0, Token::Identifier("state".to_string()));
        assert_eq!(tokens[2].0, Token::Semicolon);
        assert_eq!(tokens[3].0, Token::Rewind);
        assert_eq!(tokens[4].0, Token::Integer(5));
    }

    #[test]
    fn test_comments() {
        let source = "// This is a comment\nlet x = 1; /* block comment */";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn test_strings() {
        let source = r#"let msg = "Hello, A-lang!";"#;
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens[3].0, Token::String("Hello, A-lang!".to_string()));
    }

    #[test]
    fn test_operators() {
        let source = "a + b * c -> d |> e";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens[1].0, Token::Plus);
        assert_eq!(tokens[3].0, Token::Star);
        assert_eq!(tokens[5].0, Token::Arrow);
        assert_eq!(tokens[7].0, Token::Pipe);
    }
}
