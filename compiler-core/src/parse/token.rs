use num_bigint::BigInt;
use std::fmt;

use ecow::EcoString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Name { name: EcoString },
    HtmlTagAttrName { name: EcoString },
    UpName { name: EcoString },
    DiscardName { name: EcoString },
    Int { value: EcoString, int_value: BigInt },
    Float { value: EcoString },
    String { value: EcoString },
    CommentDoc { content: EcoString },
    HtmlText { value: EcoString },
    HtmlStartTag,
    // Groupings
    LeftParen,   // (
    RightParen,  // )
    LeftSquare,  // [
    RightSquare, // ]
    LeftBrace,   // {
    RightBrace,  // }
    // Int Operators
    Plus,
    Minus,
    Star,
    Slash,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Percent,
    // Float Operators
    PlusDot,         // '+.'
    MinusDot,        // '-.'
    StarDot,         // '*.'
    SlashDot,        // '/.'
    LessDot,         // '<.'
    GreaterDot,      // '>.'
    LessEqualDot,    // '<=.'
    GreaterEqualDot, // '>=.'
    EqualDot,        // '=.'
    LtSt,            // '</'
    LtStGt,          // '</>'
    StGt,            // '/>'
    LtGt,            // '<>'
    // String Operators
    PlusPlus, // '++'
    // Other Punctuation
    Colon,
    Comma,
    Hash, // '#'
    Bang, // '!'
    Equal,
    EqualEqual, // '=='
    NotEqual,   // '!='
    Vbar,       // '|'
    VbarVbar,   // '||'
    AmperAmper, // '&&'
    LtLt,       // '<<'
    GtGt,       // '>>'
    Pipe,       // '|>'
    Dot,        // '.'
    RArrow,     // '->'
    LArrow,     // '<-'
    DotDot,     // '..'
    At,         // '@'
    EndOfFile,
    // Extra
    CommentNormal,
    CommentModule,
    NewLine,
    // Keywords (alphabetically):
    As,
    Assert,
    Auto,
    Case,
    Const,
    Delegate,
    Derive,
    Echo,
    Else,
    Fn,
    If,
    Implement,
    Import,
    Let,
    Macro,
    Opaque,
    Panic,
    Pub,
    Test,
    Todo,
    Type,
    Record,
    Use,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum TokenIteratorMode {
    HtmlContent,
    HtmlTagAttr,
    Code,
}

pub trait TokenIterator {
    type Item;
    fn change_mode(&mut self, mode: TokenIteratorMode);
    fn collect_vec(self) -> Vec<Self::Item>;
    fn next(&mut self) -> Option<Self::Item>;
}
impl Token {
    pub fn guard_precedence(&self) -> Option<u8> {
        match self {
            Self::VbarVbar => Some(1),

            Self::AmperAmper => Some(2),

            Self::EqualEqual | Self::NotEqual => Some(3),

            Self::Less
            | Self::LessEqual
            | Self::LessDot
            | Self::LessEqualDot
            | Self::GreaterEqual
            | Self::Greater
            | Self::GreaterEqualDot
            | Self::GreaterDot => Some(4),

            Self::Plus | Self::PlusDot | Self::Minus | Self::MinusDot => Some(5),

            Self::Star | Self::StarDot | Self::Slash | Self::SlashDot | Self::Percent => Some(6),

            _ => None,
        }
    }

    pub fn is_reserved_word(&self) -> bool {
        match self {
            Token::As
            | Token::Assert
            | Token::Case
            | Token::Const
            | Token::Fn
            | Token::If
            | Token::Import
            | Token::Let
            | Token::Opaque
            | Token::Pub
            | Token::Todo
            | Token::Type
            | Token::Record
            | Token::Use
            | Token::Auto
            | Token::Delegate
            | Token::Derive
            | Token::Echo
            | Token::Else
            | Token::Implement
            | Token::Macro
            | Token::Panic
            | Token::Test => true,

            Token::Name { .. }
            | Token::HtmlTagAttrName { .. }
            | Token::UpName { .. }
            | Token::DiscardName { .. }
            | Token::Int { .. }
            | Token::Float { .. }
            | Token::String { .. }
            | Token::HtmlText { .. }
            | Token::CommentDoc { .. }
            | Token::LeftParen
            | Token::RightParen
            | Token::LeftSquare
            | Token::RightSquare
            | Token::LeftBrace
            | Token::RightBrace
            | Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Less
            | Token::HtmlStartTag
            | Token::Greater
            | Token::LessEqual
            | Token::GreaterEqual
            | Token::Percent
            | Token::PlusPlus
            | Token::PlusDot
            | Token::MinusDot
            | Token::StarDot
            | Token::SlashDot
            | Token::LessDot
            | Token::GreaterDot
            | Token::LessEqualDot
            | Token::GreaterEqualDot
            | Token::LtGt
            | Token::LtSt
            | Token::StGt
            | Token::LtStGt
            | Token::Colon
            | Token::Comma
            | Token::Hash
            | Token::Bang
            | Token::Equal
            | Token::EqualEqual
            | Token::NotEqual
            | Token::EqualDot
            | Token::Vbar
            | Token::VbarVbar
            | Token::AmperAmper
            | Token::LtLt
            | Token::GtGt
            | Token::Pipe
            | Token::Dot
            | Token::RArrow
            | Token::LArrow
            | Token::DotDot
            | Token::At
            | Token::EndOfFile
            | Token::CommentNormal
            | Token::CommentModule
            | Token::NewLine => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Token::Name { name }
            | Token::HtmlTagAttrName { name }
            | Token::UpName { name }
            | Token::DiscardName { name } => name.as_str(),
            Token::Int {
                value,
                int_value: _,
            }
            | Token::HtmlText { value }
            | Token::Float { value }
            | Token::String { value } => value.as_str(),
            Token::AmperAmper => "&&",
            Token::As => "as",
            Token::Assert => "assert",
            Token::At => "@",
            Token::Auto => "auto",
            Token::Bang => "!",
            Token::Case => "case",
            Token::Colon => ":",
            Token::Comma => ",",
            Token::CommentDoc { .. } => "///",
            Token::CommentModule => "////",
            Token::CommentNormal => "//",
            Token::Const => "const",
            Token::Delegate => "delegate",
            Token::Derive => "derive",
            Token::Dot => ".",
            Token::DotDot => "..",
            Token::Echo => "echo",
            Token::Else => "else",
            Token::NewLine => "NEWLINE",
            Token::EndOfFile => "EOF",
            Token::Equal => "=",
            Token::EqualEqual => "==",
            Token::Fn => "fn",
            Token::Greater => ">",
            Token::GreaterDot => ">.",
            Token::GreaterEqual => ">=",
            Token::GreaterEqualDot => ">=.",
            Token::GtGt => ">>",
            Token::Hash => "#",
            Token::If => "if",
            Token::Implement => "implement",
            Token::Import => "import",
            Token::LArrow => "<-",
            Token::LeftBrace => "{",
            Token::LeftParen => "(",
            Token::LeftSquare => "[",
            Token::Less => "<",
            Token::LessDot => "<.",
            Token::LessEqual => "<=",
            Token::LessEqualDot => "<=.",
            Token::EqualDot => "=.",
            Token::Let => "let",
            Token::LtGt => "<>",
            Token::LtSt => "</",
            Token::LtStGt => "</>",
            Token::StGt => "/>",
            Token::LtLt => "<<",
            Token::Macro => "macro",
            Token::Minus => "-",
            Token::MinusDot => "-.",
            Token::NotEqual => "!=",
            Token::HtmlStartTag => "<",
            Token::Opaque => "opaque",
            Token::Panic => "panic",
            Token::Percent => "%",
            Token::Pipe => "|>",
            Token::Plus => "+",
            Token::PlusPlus => "++",
            Token::PlusDot => "+.",
            Token::Pub => "pub",
            Token::RArrow => "->",
            Token::RightBrace => "}",
            Token::RightParen => ")",
            Token::RightSquare => "]",
            Token::Slash => "/",
            Token::SlashDot => "/.",
            Token::Star => "*",
            Token::StarDot => "*.",
            Token::Test => "test",
            Token::Todo => "todo",
            Token::Type => "type",
            Token::Record => "record",
            Token::Use => "use",
            Token::Vbar => "|",
            Token::VbarVbar => "||",
        };
        write!(f, "`{s}`")
    }
}
