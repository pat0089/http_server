use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Debug, Eq, Copy, Clone)]
pub enum MimeType {
    Html,
    Json,
    PlainText,
    Javascript,
    Css,
    Jpeg,
    Png,
    Gif,
    Bitmap,
    CommaSeparatedValues,

    Unknown,
}

impl FromStr for MimeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text/html" => Ok(MimeType::Html),
            "application/json" => Ok(MimeType::Json),
            "text/plain" => Ok(MimeType::PlainText),
            "text/javascript" => Ok(MimeType::Javascript),
            "text/css" => Ok(MimeType::Css),
            "image/jpeg" => Ok(MimeType::Jpeg),
            "image/png" => Ok(MimeType::Png),
            "image/gif" => Ok(MimeType::Gif),
            "image/bmp" => Ok(MimeType::Bitmap),
            "text/csv" => Ok(MimeType::CommaSeparatedValues),

            _ => Ok(MimeType::Unknown),
        }
    }
}

impl Display for MimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}", match self {
                MimeType::Html => "text/html",
                MimeType::Json => "application/json",
                MimeType::PlainText => "text/plain",
                MimeType::Javascript => "text/javascript",
                MimeType::Css => "text/css",
                MimeType::Jpeg => "image/jpeg",
                MimeType::Png => "image/png",
                MimeType::Gif => "image/gif",
                MimeType::Bitmap => "image/bmp",
                MimeType::CommaSeparatedValues => "text/csv",
                
                _ => "Unknown", 
            }
        )
    }
}

pub fn from_file_extension(extension: &str) -> MimeType {
    match extension {
        "html" => MimeType::Html,
        "json" => MimeType::Json,
        "txt" => MimeType::PlainText,
        "js" => MimeType::Javascript,
        "css" => MimeType::Css,
        "jpg" => MimeType::Jpeg,
        "png" => MimeType::Png,
        "gif" => MimeType::Gif,
        "bmp" => MimeType::Bitmap,
        "csv" => MimeType::CommaSeparatedValues,

        _ => MimeType::Unknown,
    }
}