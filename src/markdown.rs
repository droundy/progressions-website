use display_as::{HTML, DisplayAs};
use std::fmt::{Formatter, Error};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug,Clone,Eq,PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Markdown(String);

impl Markdown {
    pub fn new(md: &str) -> Markdown {
        Markdown(md.to_string())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn from_html(html: &str) -> Markdown {
        // println!("\nstarting with:\n*************\n{}\n********\n", html);
        let x = html2md::parse_html(html);
        // preserve "\\" in math mode, which disallows escaping a
        // backslash, but why would we want a backslash?
        let x = x.replace(r"\\", r"\\\\");
        // mode disallow escaping square brackets [] since we want
        // math mode to *not* require extra escapes.
        let x = x.replace(r"\[", r"\\[");
        let x = x.replace(r"\]", r"\\]");
        // println!("\nending with:\n*************\n{}\n********\n", x.trim());
        Markdown(x.trim().to_string())
    }
}

impl From<&str> for Markdown {
    fn from(x: &str) -> Self {
        Markdown::new(x)
    }
}

impl Default for Markdown {
    fn default() -> Self {
        Markdown("".to_string())
    }
}

impl DisplayAs<HTML> for Markdown {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let parser = pulldown_cmark::Parser::new(&self.0);

        let mut html_buf = String::new();
        pulldown_cmark::html::push_html(&mut html_buf, parser);
        f.write_str(&html_buf)
    }
}
