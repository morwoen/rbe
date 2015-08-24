use std::ascii::AsciiExt;
use std::borrow::Borrow;
use dom::{ self, Node, AttrMap };
use css::{ Value, Unit, Color, Declaration, SimpleSelector, Selector, Rule };

#[derive(Debug)]
pub struct Parser {
    pos: usize,
    input: String
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            pos: 0,
            input: source
        }
    }

    /// Returns the next character from the input
    fn next_char (&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Returns true if the input from the current position starts with the given String
    ///         false otherwise
    fn starts_with (&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// Returns true if the end of the input has been reached
    fn eof (&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Consumes and returns the next character of the input
    fn consume_char (&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// Consumes and returns as strings characters while they pass the test function
    fn consume_while (&mut self, check: &Fn (char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && check(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Consume all whitespaces from the current position
    fn consume_whitespace (&mut self) {
        self.consume_while(&char::is_whitespace);
    }

    /// Consume a tag or attribute name
    fn parse_tag_name (&mut self) -> String {
        self.consume_while(&|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })
    }
}

pub trait HtmlParser {
    fn parse_nodes (&mut self) -> Vec<Node>;
    fn parse_node (&mut self) -> Node;
    fn parse_text (&mut self) -> Node;
    fn parse_element (&mut self) -> Node;
    fn parse_attr (&mut self) -> (String, String);
    fn parse_attr_value (&mut self) -> String;
    fn parse_attributes (&mut self) -> AttrMap;
}

impl HtmlParser for Parser {
    /// Parse and return in a vec all nodes
    fn parse_nodes (&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// Parse and return a node
    fn parse_node (&mut self) -> Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    /// Parse and return a text node
    fn parse_text (&mut self) -> Node {
        dom::text(self.consume_while(&|c| c != '<'))
    }

    /// Parse and return an element node
    fn parse_element (&mut self) -> Node {
        // Opening tag
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Children
        let children = self.parse_nodes();

        // Closing tag
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        dom::elem(tag_name, attrs, children)
    }

    /// Parse and return attribute name and value
    fn parse_attr (&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    /// Parse and return attribute value
    fn parse_attr_value (&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(&|c| c!= open_quote);
        assert!(self.consume_char() == open_quote);
        value
    }

    /// Parse and return all attributes
    fn parse_attributes (&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }
}

trait CssParser {
    fn parse_rules (&mut self) -> Vec<Rule>;
    fn parse_rule (&mut self) -> Rule;
    fn parse_selectors (&mut self) -> Vec<Selector>;
    fn parse_simple_selector (&mut self) -> SimpleSelector;
    fn parse_declarations (&mut self) -> Vec<Declaration>;
    fn parse_declaration (&mut self) -> Declaration;
    fn parse_value (&mut self) -> Value;
    fn parse_length (&mut self) -> Value;
    fn parse_float (&mut self) -> f32;
    fn parse_unit (&mut self) -> Unit;
    fn parse_color (&mut self) -> Value;
    fn parse_hex_pair (&mut self) -> u8;
    fn parse_identifier (&mut self) -> String;
    fn valid_indentifier_char (c: char) -> bool;
}

impl CssParser for Parser {
    fn parse_rules (&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }
        rules
    }

    fn parse_rule (&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations()
        }
    }

    fn parse_selectors (&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                },
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c)
            }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    fn parse_simple_selector (&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new()
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                },
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                },
                '*' => {
                    self.consume_char();
                },
                c if Parser::valid_indentifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                },
                _ => break
            }
        }
        selector
    }

    fn parse_declarations (&mut self) -> Vec<Declaration> {
        assert!(self.consume_char() == '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    fn parse_declaration (&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        assert!(self.consume_char() == ':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert!(self.consume_char() == ';');

        Declaration {
            name: property_name,
            value: value
        }
    }

    fn parse_value (&mut self) -> Value {
        match self.next_char() {
            '0'...'9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier())
        }
    }

    fn parse_length (&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float (&mut self) -> f32 {
        let s = self.consume_while(&|c| match c {
            '0'...'9' | '.' => true,
            _ => false
        });
        s.parse().unwrap()
    }

    fn parse_unit (&mut self) -> Unit {
        match self.parse_identifier().to_ascii_lowercase().borrow() {
            "px" => Unit::Px,
            _ => panic!("Unrecognized unit")
        }
    }

    fn parse_color (&mut self) -> Value {
        assert!(self.consume_char() == '#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255
        })
    }

    fn parse_hex_pair (&mut self) -> u8 {
        let s = &self.input[self.pos .. self.pos + 2];
        self.pos = self.pos + 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    fn parse_identifier (&mut self) -> String {
        self.consume_while(&Parser::valid_indentifier_char)
    }

    fn valid_indentifier_char (c: char) -> bool {
        match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' => true,
            _ => false
        }
    }
}
