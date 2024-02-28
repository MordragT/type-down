use chumsky::{
    prelude::*,
    text::{ascii, newline},
};
use unscanny::Scanner;

use crate::Span;

use self::{
    error::{LexError, LexResult},
    util::is_newline,
};

pub mod error;
pub mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Markup,
    Code,
    Math,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SyntaxKind {
    // Markup
    Markup,
    Escape,
    Raw,
    Link,
    Citation,
    HeadingMarker,
    ListMarker,
    EnumMarker,
    BlockQuoteMarker,
    Text,
    ParBreak,

    // Shared
    Space,
    Dollar,
    Hash,
    Tilde,
    DoubleQuote,
    SingleQuote,
    Star,
    Slash,
    Caret,
    Underscore,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comment,
    Eof,
}

pub type Token = (SyntaxKind, Span);

type Extra<'src> = extra::Err<Rich<'src, char, Span>>;

pub fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<SyntaxKind>, Extra<'src>> {}

pub fn comment<'src>() -> impl Parser<'src, &'src str, SyntaxKind, Extra<'src>> {
    just("%")
        .then(any().and_is(newline().not()).repeated().to_slice())
        .to(SyntaxKind::Comment)
}
pub struct Lexer<'src> {
    mode: Mode,
    s: Scanner<'src>,
    newline: bool,
}

/// Public
impl<'src> Lexer<'src> {
    pub fn new(src: &'src str, mode: Mode) -> Self {
        Self {
            mode,
            s: Scanner::new(src),
            newline: false,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn switch(&mut self, mode: Mode) {
        self.mode = mode;
    }

    // create parser over syntaxkind and span
    // and then in a next step use the generated syntaxnodes
    // to create a ast
    pub fn next(&mut self) -> LexResult<Token> {
        self.newline = false;
        let start = self.s.cursor();

        let token = match self.s.eat() {
            Some(c) if util::is_space(c, self.mode) => self.whitespace(start),
            Some('%') => self.comment(start),
            Some(c) => match self.mode {
                Mode::Markup => self.markup(c, start)?,
                Mode::Code => self.code(c, start)?,
                Mode::Math => self.math(c, start)?,
            },
            None => self.eof(start),
        };

        Ok(token)
    }
}

/// Shared
impl<'src> Lexer<'src> {
    fn token(&self, kind: SyntaxKind, start: usize) -> Token {
        let end = self.s.cursor();
        let span = (start..end).into();
        (kind, span)
    }

    fn error(&self, msg: impl Into<String>, start: usize) -> LexError {
        let end = self.s.cursor();
        let span = (start..end).into();

        LexError {
            span,
            msg: msg.into(),
        }
    }

    fn whitespace(&mut self, start: usize) -> Token {
        let is_space = |c| util::is_space(c, self.mode);
        self.s.eat_while(is_space);
        let end = self.s.cursor();
        let spacing = self.s.get(start..end);

        let mut count = 0;
        let mut s = Scanner::new(spacing);
        while let Some(c) = s.eat() {
            if util::is_newline(c) {
                if c == '\r' {
                    s.eat_if('\n');
                }
                count += 1;
            }
        }

        self.newline = count > 0;
        if count >= 2 && self.mode == Mode::Markup {
            self.token(SyntaxKind::ParBreak, start)
        } else {
            self.token(SyntaxKind::Space, start)
        }
    }

    fn comment(&mut self, start: usize) -> Token {
        self.s.eat_until(util::is_newline);
        self.token(SyntaxKind::Comment, start)
    }

    fn eof(&self, start: usize) -> Token {
        self.token(SyntaxKind::Eof, start)
    }
}

/// Markup
impl<'src> Lexer<'src> {
    fn markup(&mut self, c: char, start: usize) -> LexResult<Token> {
        let token = match c {
            '\\' => self.escape(start),
            '`' => self.raw(start)?,
            '<' => self.link(start)?,
            '@' => self.citation(start)?,
            '*' if !self.in_word() => self.token(SyntaxKind::Star, start),
            '/' if !self.in_word() => self.token(SyntaxKind::Slash, start),
            '\'' if !self.in_word() => self.token(SyntaxKind::SingleQuote, start),
            '"' => self.token(SyntaxKind::DoubleQuote, start),
            '~' => self.token(SyntaxKind::Tilde, start),
            '^' => self.token(SyntaxKind::Caret, start),
            '_' => self.token(SyntaxKind::Underscore, start),
            '[' => self.token(SyntaxKind::LeftBracket, start),
            ']' => self.token(SyntaxKind::RightBracket, start),
            '{' => self.token(SyntaxKind::LeftBrace, start),
            '}' => self.token(SyntaxKind::RightBrace, start),
            '#' => self.token(SyntaxKind::Hash, start),
            '$' => self.token(SyntaxKind::Dollar, start),
            '=' => {
                self.s.eat_while('=');
                if self.ws_or_end() {
                    self.token(SyntaxKind::HeadingMarker, start)
                } else {
                    self.text(start)
                }
            }
            '-' if self.ws_or_end() => self.token(SyntaxKind::ListMarker, start),
            '+' if self.ws_or_end() => self.token(SyntaxKind::EnumMarker, start),
            '>' if self.ws_or_end() => self.token(SyntaxKind::BlockQuoteMarker, start),
            _ => self.text(start),
        };

        Ok(token)
    }

    fn escape(&mut self, start: usize) -> Token {
        self.s.eat();
        self.token(SyntaxKind::Escape, start)
    }

    fn raw(&mut self, start: usize) -> LexResult<Token> {
        let mut count = 1;
        while self.s.eat_if('`') {
            count += 1;
        }

        if count == 2 {
            return Ok(self.token(SyntaxKind::Raw, start));
        }

        let mut found = 0;
        while found < count {
            match self.s.eat() {
                Some('`') => found += 1,
                Some(_) => found = 0,
                None => break,
            }
        }

        if count != found {
            return Err(self.error("unclosed raw text", start));
        }

        Ok(self.token(SyntaxKind::Raw, start))
    }

    fn link(&mut self, start: usize) -> LexResult<Token> {
        self.s.eat_until(|c| is_newline(c) || c == '>');
        if self.s.eat_if('>') {
            Ok(self.token(SyntaxKind::Link, start))
        } else {
            Err(self.error("unclosed link", start))
        }
    }

    fn citation(&mut self, start: usize) -> LexResult<Token> {
        self.s.eat_while(util::is_identy);

        let end = self.s.cursor();

        if start == end {
            Err(self.error("empty citing", start))
        } else {
            Ok(self.token(SyntaxKind::Citation, start))
        }
    }

    fn text(&mut self, start: usize) -> Token {
        self.s.eat_until(util::is_special);
        self.token(SyntaxKind::Text, start)
    }

    fn in_word(&self) -> bool {
        let prev = self.s.scout(-2);
        let next = self.s.peek();

        util::is_wordy(prev) && util::is_wordy(next)
    }

    fn ws_or_end(&self) -> bool {
        self.s.done() || self.s.at(char::is_whitespace)
    }
}

/// Code
impl<'src> Lexer<'src> {
    fn code(&mut self, c: char, start: usize) -> LexResult<Token> {
        todo!()
    }
}

/// Math
impl<'src> Lexer<'src> {
    fn math(&mut self, c: char, start: usize) -> LexResult<Token> {
        todo!()
    }
}
