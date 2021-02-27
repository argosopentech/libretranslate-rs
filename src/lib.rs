//! A LibreTranslate API for Rust.
//! ```
//! libretranslate = "0.1.6"
//! ```
//!
//! libretranslate-rs allows you to use open source machine translation in your projects through an easy to use API that connects to the official webpage.
//!
//! Using it is fairly simple:
//!
//! ```rust
//! use libretranslate::{Translator, Language};
//!
//! fn main() {
//!     let input = "Olá Mundo!";
//!     let source = Language::Portuguese;
//!     let target = Language::English;
//!
//!     match Translator::translate(source, target, input) {
//!         Ok(data) => println!("{}: {}\n{}: {}", data.source.pretty(), data.input, data.target.pretty(), data.output),
//!         Err(error) => panic!("{}", error),
//!     };
//! }
//! ```
//!
//! Output:
//! ```
//! Portuguese: Olá Mundo!
//! English: Hello world!
//! ```
//!
//! Written with love, in Rust by Grant Handy.

use serde_json::Value;

/// Languages that can used for input and output of the ['translate'] function.
#[derive(Debug, Clone, Copy)]
pub enum Language {
    English,
    Arabic,
    Chinese,
    French,
    German,
    Italian,
    Portuguese,
    Russain,
    Spanish,
}

impl Language {
    /// Return the language with the language code name. (ex. "ar", "de")
    pub fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Arabic => "ar",
            Language::Chinese => "zh",
            Language::French => "fr",
            Language::German => "de",
            Language::Italian => "it",
            Language::Portuguese => "pt",
            Language::Russain => "rs",
            Language::Spanish => "es",
        }
    }

    /// Return the language with the full English name. (ex. "Arabic", "German")
    pub fn pretty(&self) -> &str {
        match self {
            Language::English => "English",
            Language::Arabic => "Arabic",
            Language::Chinese => "Chinese",
            Language::French => "French",
            Language::German => "German",
            Language::Italian => "Italian",
            Language::Portuguese => "Portuguese",
            Language::Russain => "Russain",
            Language::Spanish => "Spanish",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "en"),
            Language::Arabic => write!(f, "ar"),
            Language::Chinese => write!(f, "zh"),
            Language::French => write!(f, "fr"),
            Language::German => write!(f, "de"),
            Language::Italian => write!(f, "it"),
            Language::Portuguese => write!(f, "pt"),
            Language::Russain => write!(f, "rs"),
            Language::Spanish => write!(f, "es"),
        }
    }
}

/// Errors that could be outputed by the translator.
#[derive(Debug, Clone)]
pub enum TranslateError {
    HttpError(String),
    ParseError(String),
}

impl std::error::Error for TranslateError {}

impl std::fmt::Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TranslateError::HttpError(error) => {
                write!(f, "HTTP Request Error: {}", error.to_string())
            }
            TranslateError::ParseError(error) => {
                write!(f, "JSON Parsing Error: {}", error.to_string())
            }
        }
    }
}

pub struct Translator {
    pub source: Language,
    pub target: Language,
    pub input: String,
    pub output: String,
}

impl Translator {
    /// Translate text between two languages.
    pub fn translate(source: Language, target: Language, input: &str) -> Result<Self, TranslateError> {
        match ureq::post("https://libretranslate.com/translate").send_json(ureq::json!({
            "q": input,
            "source": source.code(),
            "target": target.code(),
        })) {
            Ok(data) => {
                let string: String = match data.into_string() {
                    Ok(data) => data,
                    Err(error) => {
                        return Err(TranslateError::ParseError(error.to_string()));
                    }
                };

                let parsed_json: Value = match serde_json::from_str(&string) {
                    Ok(parsed_json) => parsed_json,
                    Err(error) => {
                        return Err(TranslateError::ParseError(error.to_string()));
                    }
                };

                let output = match &parsed_json["translatedText"] {
                    Value::String(output) => output,
                    _ => {
                        return Err(TranslateError::ParseError(String::from(
                            "Unable to find translatedText in parsed JSON",
                        )))
                    }
                };

                let input = input.to_string();
                let output = output.to_string();

                let myself = Self {
                    source,
                    target,
                    input,
                    output,
                };

                return Ok(myself);
            }
            Err(error) => return Err(TranslateError::HttpError(error.to_string())),
        };
    }
}
