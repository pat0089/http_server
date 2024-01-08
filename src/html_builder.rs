fn write_html_tag(tag_name: &str, open_closed: bool, content: Option<&str>, attributes: Option<&str>) -> String {
    match open_closed {
        true => format!("<{}{}>{}</{}>", tag_name, attributes.unwrap_or(""), content.unwrap_or(""), tag_name),
        false => format!("<{} {}/>", tag_name, attributes.unwrap_or("")),
    }
}

pub fn write_attribute(name: &str, value: &str) -> String {
    format!(" {}=\'{}\'", name, value)
}

pub fn write_html(content: Option<&str>) -> String {
    write_html_tag("html", true, content, None)
}

pub fn write_head(content: Option<&str>) -> String {
    write_html_tag("head", true, content, None)
}

pub fn write_body(content: Option<&str>) -> String {
    write_html_tag("body", true, content, None)
}

pub fn write_title(content: Option<&str>) -> String {
    write_html_tag("title", true, content, None)
}

pub fn write_script(content: Option<&str>, additional_attributes: Option<&str>) -> String {
    write_html_tag("script", true, content, Some(&format!("{}{}", write_attribute("type", "text/javascript"), additional_attributes.unwrap_or(""))))
}

pub fn write_header(level: u8, content: Option<&str>) -> String {
    match level {
        1 => write_html_tag("h1", true, content, None),
        2 => write_html_tag("h2", true, content, None),
        3 => write_html_tag("h3", true, content, None),
        4 => write_html_tag("h4", true, content, None),
        5 => write_html_tag("h5", true, content, None),
        6 => write_html_tag("h6", true, content, None),
        _ => write_paragraph(content),
    }
}

pub fn write_paragraph(content: Option<&str>) -> String {
    write_html_tag("p", true, content, None)
}

pub fn write_style(content: Option<&str>) -> String {
    write_html_tag("style", true, content, Some(&write_attribute("type", "text/css")))
}
