extern crate plist;

use plist::{dictionary::Dictionary, Value};
use std::{fmt, path::Path};

// ---- Color schemes ----

#[derive(Debug)]
pub struct ColorScheme {
    pub ansi: Vec<Color>,
    pub special: SpecialColors,
}
impl ColorScheme {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ColorScheme, Error> {
        let value = Value::from_file(path)?;
        let dictionary = value.as_dictionary_r()?;
        Self::from_dictionary(dictionary)
    }
    pub fn from_dictionary(d: &Dictionary) -> Result<ColorScheme, Error> {
        let ansi = Self::ansi_from_dictionary(d)?;
        let special = SpecialColors::from_dictionary(d)?;
        Ok(ColorScheme { ansi, special })
    }
    fn ansi_from_dictionary(d: &Dictionary) -> Result<Vec<Color>, Error> {
        let mut ansi: Vec<Color> = Vec::new();
        for i in 0..16 {
            let ansi_name = format!("Ansi {} Color", i);
            let color = Color::named_color(&ansi_name, d)?;
            ansi.push(color);
        }
        Ok(ansi)
    }
    pub fn to_kitty(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("foreground {}\n", self.special.foreground.to_hex()));
        s.push_str(&format!("background {}\n", self.special.background.to_hex()));
        s.push_str(&format!("selection_foreground {}\n", self.special.selected_text.to_hex()));
        s.push_str(&format!("selection_background {}\n", self.special.selection.to_hex()));
        for i in 0..16 {
            let name = format!("color{:<2}", i);
            s.push_str(&format!("{} {}\n", name, self.ansi[i].to_hex()));
        }
        s
    }
}

#[derive(Debug)]
pub struct SpecialColors {
    pub background: Color,
    pub bold: Color,
    pub cursor: Color,
    pub cursor_text: Color,
    pub foreground: Color,
    pub selected_text: Color,
    pub selection: Color,
}
impl SpecialColors {
    fn from_dictionary(d: &Dictionary) -> Result<SpecialColors, Error> {
        let background = Color::named_color("Background Color", d)?;
        let bold = Color::named_color("Bold Color", d)?;
        let cursor = Color::named_color("Cursor Color", d)?;
        let cursor_text = Color::named_color("Cursor Text Color", d)?;
        let foreground = Color::named_color("Foreground Color", d)?;
        let selected_text = Color::named_color("Selected Text Color", d)?;
        let selection = Color::named_color("Selection Color", d)?;
        Ok(SpecialColors {
            background,
            bold,
            cursor,
            cursor_text,
            foreground,
            selected_text,
            selection,
        })
    }
}

#[derive(Debug)]
pub enum Color {
    SRGB(SRGB),
}
impl Color {
    fn named_color(name: &str, d: &Dictionary) -> Result<Color, Error> {
        let color_dict = d.get_r(name)?.as_dictionary_r()?;
        Self::from_dictionary(color_dict)
    }
    fn from_dictionary(d: &Dictionary) -> Result<Color, Error> {
        let color_space = d.get_r("Color Space")?.as_string_r()?;
        match color_space {
            "sRGB" => Ok(Color::SRGB(SRGB::from_dictionary(d)?)),
            _ => Err(Error::UnknownColorSpace(color_space.to_string())),
        }
    }
    fn to_hex(&self) -> HexColor {
        self.into()
    }
}

#[derive(Debug)]
pub struct SRGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl SRGB {
    fn new(r: f64, g: f64, b: f64) -> SRGB {
        SRGB { r, g, b }
    }
    fn from_dictionary(d: &Dictionary) -> Result<SRGB, Error> {
        let color_space = d.get_r("Color Space")?.as_string_r()?;
        if color_space != "sRGB" {
            return Err(Error::InvalidColorSpace {
                expected: "sRGB".to_string(),
                actual: color_space.to_string(),
            });
        }

        let r = d.get_r("Red Component")?.as_real_r()?;
        let g = d.get_r("Green Component")?.as_real_r()?;
        let b = d.get_r("Blue Component")?.as_real_r()?;

        Ok(Self::new(r, g, b))
    }
}

#[derive(Debug)]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.as_string())
    }
}
impl HexColor {
    fn new(r: u8, g: u8, b: u8) -> HexColor {
        HexColor { r, g, b }
    }
    fn as_string(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
impl From<&SRGB> for HexColor {
    fn from(srgb: &SRGB) -> HexColor {
        let r = float_color_component_to_u8(srgb.r);
        let g = float_color_component_to_u8(srgb.g);
        let b = float_color_component_to_u8(srgb.b);
        HexColor::new(r, g, b)
    }
}
impl From<&Color> for HexColor {
    fn from(color: &Color) -> HexColor {
        match color {
            Color::SRGB(srgb) => srgb.into()
        }
    }
}

fn float_color_component_to_u8(comp: f64) -> u8 {
    clip(comp * 255.0, 0.0, 255.0).round() as u8
}

fn clip(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

trait DictionaryExtensions {
    fn get_r<'a>(&'a self, key: &str) -> Result<&'a Value, Error>;
}

impl DictionaryExtensions for Dictionary {
    fn get_r<'a>(&'a self, key: &str) -> Result<&'a Value, Error> {
        self.get(key).ok_or(Error::MissingKey(key.to_string()))
    }
}

trait ValueExtensions {
    fn as_string_r(&self) -> Result<&str, Error>;
    fn as_real_r(&self) -> Result<f64, Error>;
    fn as_dictionary_r(&self) -> Result<&Dictionary, Error>;
}

impl ValueExtensions for Value {
    fn as_string_r(&self) -> Result<&str, Error> {
        self.as_string().ok_or(Error::invalid_type("String", self))
    }
    fn as_real_r(&self) -> Result<f64, Error> {
        self.as_real().ok_or(Error::invalid_type("Real", self))
    }
    fn as_dictionary_r(&self) -> Result<&Dictionary, Error> {
        self.as_dictionary()
            .ok_or(Error::invalid_type("Dictionary", self))
    }
}

// ---- Errors ----

#[derive(Debug)]
pub enum Error {
    PListError(plist::Error),
    MissingKey(String),
    InvalidType { expected: String, actual: String },
    InvalidColorSpace { expected: String, actual: String },
    UnknownColorSpace(String),
}
impl Error {
    fn invalid_type(expected: &str, actual: &Value) -> Error {
        let actual_str = match actual {
            Value::Array(_) => "Array",
            Value::Dictionary(_) => "Dictionary",
            Value::Boolean(_) => "Boolean",
            Value::Data(_) => "Data",
            Value::Date(_) => "Date",
            Value::Integer(_) => "Integer",
            Value::Real(_) => "Real",
            Value::String(_) => "String",
            Value::Uid(_) => "Uid",
            _ => "(unknown)",
        };
        Error::InvalidType {
            expected: expected.to_string(),
            actual: actual_str.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PListError(err) => write!(f, "PList parsing error: {}", err),
            Error::MissingKey(name) => write!(f, "Dictionary did not contain key: {}", name),
            Error::InvalidType { expected, actual } => write!(
                f,
                "Expected value of type {}, but found value of type {}",
                expected, actual
            ),
            Error::InvalidColorSpace { expected, actual } => {
                write!(f, "Expected color space {}, but found {}", expected, actual)
            }
            Error::UnknownColorSpace(name) => {
                write!(f, "Unknown color space {}", name)
            }
        }
    }
}

impl From<plist::Error> for Error {
    fn from(err: plist::Error) -> Error {
        Error::PListError(err)
    }
}
