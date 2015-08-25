#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector)
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Value
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
    pub a: u8
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity (&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let id = simple.id.iter().count();
        let class = simple.class.len();
        let tagname = simple.tag_name.iter().count();
        (id, class, tagname)
    }
}

impl Value {
    pub fn to_px (&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0f32
        }
    }
}
