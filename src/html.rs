//! Missing functionality:
//! comments
//! doctypes and processing instructions
//! self-closing tags
//! non-well-formed markup
//! character entities

use parser::{ Parser, HtmlParser };
use dom;

/// Parse a whole html file and return the root node
pub fn parse (source: String) -> dom::Node {
    let mut nodes = Parser::new(source).parse_nodes();

    // If there is a root element return it, otherwise create one
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), dom::AttrMap::new(), nodes)
    }
}
