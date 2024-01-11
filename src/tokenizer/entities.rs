#[derive(Debug, Clone)]
pub enum Element<'a> {
    PlainText(PlainText<'a>),
    Instruction(Instruction<'a>),
    Assignment(Assignment<'a>),
    Block(Block<'a>),
}


#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub start_index: usize,
    pub end_index: usize,
    pub row: usize,
    pub col: usize,
    pub text: &'a str,
}

#[derive(Debug, Clone)]
pub struct Instruction<'a> {
    pub at_rule: Token<'a>,
    pub name: Token<'a>,
    pub expression: Vec<Element<'a>>
}


#[derive(Debug, Clone)]
pub struct PlainText<'a> {
    pub token: Token<'a>
}

#[derive(Debug, Clone)]
pub struct Assignment<'a> {
    pub var_name: Token<'a>,
    pub eq: Token<'a>,
    pub expression: Token<'a>
}

#[derive(Debug, Clone)]
pub struct Block<'a> {
    pub start: Token<'a>,
    pub content: Vec<Element<'a>>,
    pub end: Token<'a>,
}
