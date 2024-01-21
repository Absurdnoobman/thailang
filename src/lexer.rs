pub enum TokenKind {
    LParen,
    RParen,
    Integer(isize)
}

pub struct Token{
    kind: TokenKind,
}

pub struct TextSpan{
    start: usize,
    end: usize,
    Literal: String
}

impl TextSpan {
    pub fn new(content: String , start: usize , end: usize) -> Self {
        TextSpan{start, end, Literal: content}
    }
    pub fn lenght(&self) -> usize {
        self.end - self.start
    }
}