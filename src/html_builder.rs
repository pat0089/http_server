use std::fmt;
use crate::server::util::mime_types::MimeType;

pub struct Page {
    html_head: HtmlElement,
}

impl Page {
    pub fn new() -> Self {
        let mut html_head = HtmlElement::new("html", None);
        html_head.add_child(HtmlElement::new("head", None));
        html_head.add_child(HtmlElement::new("body", None));
        Page { html_head }
    }

    pub fn add_title(&mut self, title: &str) {
        let head = &mut self.html_head.children[0];
        if let Some(title_element) = head.children.iter_mut().find(|element| element.tag_name == "title") {
            title_element.tag_name = title.to_string();
        } else {
            head.add_child(HtmlElement::new("title", Some(title)));
        }
    }

    fn add_body_element(&mut self, element: HtmlElement) {
        let body = &mut self.html_head.children[1];
        body.add_child(element);
    }

    fn add_head_element(&mut self, element: HtmlElement) {
        let head = &mut self.html_head.children[0];
        head.add_child(element);
    }

    pub fn add_heading(&mut self, level: usize, content: &str) {
        match level {
            1 => self.add_body_element(HtmlElement::new("h1", Some(content))),
            2 => self.add_body_element(HtmlElement::new("h2", Some(content))),
            3 => self.add_body_element(HtmlElement::new("h3", Some(content))),
            4 => self.add_body_element(HtmlElement::new("h4", Some(content))),
            5 => self.add_body_element(HtmlElement::new("h5", Some(content))),
            6 => self.add_body_element(HtmlElement::new("h6", Some(content))),
            _ => self.add_paragraph(content)
            
        }
    }
    
    pub fn add_paragraph(&mut self, content: &str) {
        self.add_body_element(HtmlElement::new("p", Some(content)))
    }

    pub fn add_break(&mut self) {
        let mut element = HtmlElement::new("br", None);
        element.open_closed = false;
        self.add_body_element(element);
    }
    
    pub fn add_script(&mut self, script_type: MimeType, path: Option<&str>, content: Option<&str>) {
        let mut element = HtmlElement::new("script", content);
        element.add_attribute("type", &script_type.to_string());
        if let Some(path) = path {
            element.add_attribute("src", path);
        }
        self.add_head_element(element);
    }

    pub fn add_style(&mut self, path: Option<&str>, content: Option<&str>) {
        let mut element = HtmlElement::new("style", content);
        if let Some(path) = path {
            element.add_attribute("src", path);
        }
        self.add_head_element(element);
    }

    pub fn add_canvas(&mut self, width: u32, height: u32, class: Option<&str>, setup_webgl: bool) {
        let mut element = HtmlElement::new("canvas", None);
        
        element.add_attribute("width", &width.to_string());
        
        element.add_attribute("height", &height.to_string());

        if let Some(class) = class {
            element.add_attribute("class", class);
        }
        
        if setup_webgl {
            self.add_script(MimeType::JavaScript, Some("gl-matrix-min.js"), None);
            self.add_script(MimeType::JavaScript, Some("webgl_test.js"), None);

            element.add_attribute("class", "webgl_canvas");
        }

        self.add_body_element(element);

    }

    pub fn add_hyperlink(&mut self, content: &str, addr: &str) {
        let mut element = HtmlElement::new("a", Some(content));
        element.add_attribute("href", addr);
        self.add_body_element(element);
    }
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.html_head.print_node_tree(0))
    }
}

#[derive(Debug)]
struct HtmlElement {
    tag_name: String,
    children: Vec<HtmlElement>,
    attributes: Vec<HtmlAttribute>,
    content: Option<String>,
    open_closed: bool,
}

impl HtmlElement {
    fn new(tag_name: &str, content: Option<&str>) -> Self {
        HtmlElement {
            tag_name: tag_name.to_string(),
            children: Vec::new(),
            attributes: Vec::new(),
            content: content.and_then(|inner| Some(inner.to_string())),
            open_closed: true,
        }
    }

    fn add_child(&mut self, child: HtmlElement) {
        if self.open_closed && !child.tag_name.is_empty() {
            self.children.push(child);
        }
    }

    fn add_attribute(&mut self, name: &str, value: &str) {
        if name.is_empty() || value.is_empty() {
            return;
        }

        if let Some(attr) = self.attributes.iter_mut().find(|attr| attr.name == name) {
            attr.value = value.to_string();
            return;
        }
        self.attributes.push(HtmlAttribute::new(name.to_string(), value.to_string()));
    }

    /// Print the node tree in a pretty way
    /// Note: assumes that the calling node is the root, and will not care about traversing backwards
    /// # Arguments
    /// * `indent` - The number of tabs to indent by
    /// # Returns
    /// * `String` - The pretty printed node tree string
    pub fn print_node_tree(&self, indent: usize) -> String {
        let mut result = String::new();
        for _ in 0..indent {
            result.push_str("\t");
        }
        result.push_str(&format!("<{}", self.tag_name));
        for attribute in &self.attributes {
            result.push_str(&format!(" {}=\'{}\'", attribute.name, attribute.value));
        }
        if !self.open_closed {
            result.push_str("/>\n");
            return result;
        }
        result.push_str(">");
        if !self.children.is_empty() || self.content.is_some() {
            result.push_str("\n");
        }
        if let Some(content) = &self.content {
            for line in content.lines() {
                for _ in 0..indent + 1 {
                    result.push_str("\t");
                }
                result.push_str(line);    
            }
        }
        for child in &self.children {
            result.push_str(&child.print_node_tree(indent + 1));
        }
        if !self.children.is_empty() || self.content.is_some() {
            result.push_str("\n");
            for _ in 0..indent {
                result.push_str("\t");
            }
        }
        result.push_str(&format!("</{}>\n", self.tag_name));
        result
    }
}

#[derive(Debug)]
struct HtmlAttribute {
    name: String,
    value: String
}

impl HtmlAttribute {
    pub fn new(name: String, value: String) -> Self {
        HtmlAttribute {
            name,
            value
        }
    }
}

impl fmt::Display for HtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.open_closed {
            true => {
                write!(f, "<{}", self.tag_name)?;
                for attribute in &self.attributes {
                    write!(f, " {}=\'{}\'", attribute.name, attribute.value)?;
                }
                write!(f, ">")?;
                write!(f, "</{}>", self.tag_name)
        }            
            false => {
                write!(f, "<{}", self.tag_name)?;
                for attribute in &self.attributes {
                    write!(f, " {}=\'{}\'", attribute.name, attribute.value)?;
                }
                write!(f, "/>")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_node() {
        println!("{}", Page::new())
    }
}