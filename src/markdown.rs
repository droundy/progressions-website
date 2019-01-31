use display_as::{HTML, UTF8, DisplayAs, with_template, format_as};
use std::fmt::{Formatter, Error};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug,Clone,Eq,PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Markdown(String);

impl Markdown {
    pub fn new(md: String) -> Markdown {
        Markdown(md)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn from_html(html: &str) -> Markdown {
        Markdown(html2md::parse_html(html).trim().to_string())
    }
    pub fn edit_me(&self, id: impl crate::data::ID, name: &str) -> EditMarkdown {
        EditMarkdown { md: self.clone(), id: format_as!(HTML, id), name: name.to_string() }
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
impl DisplayAs<UTF8> for Markdown {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.0)
    }
}

#[derive(Debug,Clone,Eq,PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EditMarkdown {
    md: Markdown,
    id: String,
    name: String,
}
#[with_template("[%" "%]" "edit-markdown.html")]
impl DisplayAs<HTML> for EditMarkdown {}
