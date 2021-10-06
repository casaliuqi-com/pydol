use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::ffi::OsStr;
use std::convert::TryFrom;

use regex;

#[cfg(test)]
mod tests;

// Silakan kontributor baru coba: 
// 1. Tambahkan 2 konten: BottomRightWidthHeight 
//    dan LeftBottomWidthHeight
// 2. Urutan leksikografis(A ke Z)
enum PositionType {
    TopLeftBottomRight,
    TopLeftWidthHeight,
    TopRightWidthHeight,
    TopLeftBottomWidth,
    TopBottomRightWidth,
    TopLeftRightHeight,
    LeftBottomRightHeight,
}

impl PositionType {
    #[inline]
    pub fn to_rect(&self, width: u32, height: u32, param1: u32, 
    param2: u32, param3: u32, param4: u32) -> (u32, u32, u32, u32) {
        match self {
            PositionType::TopLeftBottomRight => (param2, param1, width - param4, height - param3),
            PositionType::TopLeftWidthHeight => (param2, param1, param2 + param3, param1 + param4),
            // Silakan kontributor baru coba: 
            // Returnan ialah tupel(atau tuple dalam Inggris) yang dibuat oleh x1, y1, x2 dan y2
            // x1 dan y1 adalah koordinat ujung kiri atas, 
            // x2 dan y2 adalah koordinat ujung kanan bawah
            // hapus #[ignore] untuk fungsi yang bernama 
            // full_test_for_type_positiontype di mod tests
            _ => (0, 0, 0, 0)
        }
    }
}

enum ComponentType {
    Block,
    Custom,
    RoundedBlock(u32),
    // Yang tipenya boolean adalah apakah teks ditulis 
    // dari kanan ke kiri, seperti bahasa Arab
    Text(String, bool),
}

#[derive(Hash, Eq, PartialEq)]
enum Event {
    Click,
    Enter,
    Out,
    Move,
    Focus,
    Blur,
}

struct Component {
    name: String,
    component_type: ComponentType,
    position: (PositionType, u32, u32, u32, u32),
    events: HashMap<Event, String>,
    children: Vec<Component>
}

impl Component {
    #[cfg(debug_assertions)]
    fn load_core(path: &Path) -> Option<String> {
        if let Ok(r) = fs::read_to_string(path) {
            Some(r)
        } else {
            None
        }
    }
    
    #[cfg(not(debug_assertions))]
    fn load_core(path: &Path) -> Option<String> {
        if let Some(p) = path.to_str() {
            Some(include_str!(p).to_string())
        } else {
            None
        }
    }
    
    pub fn load(path: impl AsRef<Path>, name: impl Into<Option<&'static str>>) -> Option<Self> {
        let path = path.as_ref();
        let (name, component) = (
            name.into().unwrap_or(
                path.file_name()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("")
            ),
            Self::load_core(path)
        );
        if let Some(c) = component {
            if let Ok(r) = Self::try_from(c) {
                return Some(r);
            }
        }
        None
    }
}

enum Token {
    At, // @
    Apostrophe, // '
    Quotes, // "
    Colon, // :
    Backslash, // \
    EqualSign,   // =
    LeftBrackets, // <
    RightBrackets, // >
    Text(String),
}

fn string2token_list(stream: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut last = "".to_string();
    stream.to_str().chars().for_each(|c| {
        match c {
            '@'|'\''|'"'|
            ':'|'\\'|'='|
            '<'|'>' => {
                result.push(Token::Text(last));
                last = "".to_string();
            }
            _ => last = format!("{}{}", last, c),
        }
        match c {
            '@' => result.push(Token::At),
            '\'' => result.push(Token::Apostrophe),
            '"' => result.push(Token::Quotes),
            ':' => result.push(Token::Colon),
            '\\' => result.push(Token::Backslash),
            '=' => result.push(Token::EqualSign),
            '<' => result.push(Token::LeftBrackets),
            '>' => result.push(Token::RightBrackets),
            _ => {}
        }
    });
    result.push(Token::Text(last));
    result
}

impl TryFrom<String> for Component {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        string2token_list(value);
        unimplemented!();
    }
}
