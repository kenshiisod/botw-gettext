use msbt::{Msbt, Encoding};
use potty::PotMessage;
use msbt::section::txt2::Token;
use std::io::{Read, Seek, BufRead, Write};
use regex::Regex;

macro_rules! tag_code_maps {
    ($(($a:expr, $b:expr) => $result:path),*) => {
        pub fn tag_codes_to_name(group: u16, tag: u16) -> String {
            match (group, tag) {
                $(($a, $b) => $result.to_string()),*,
                _ => format!("{:02X}:{:02X}", group, tag)
            }
        }
        pub fn tag_name_to_codes(name: &str) -> Option<(u16, u16)> {
            match name {
                $($result => Some(($a, $b))),*,
                _ => None
            }.or_else(|| {
                let mut group_tag = name.split(":");
                let group = hex::decode(group_tag.next()?).ok()?[0];
                let tag = hex::decode(group_tag.next()?).ok()?[0];
                Some((group as u16, tag as u16))
            })
        }
    }
}

pub mod tag_name {
    pub const ANIMATION: &str = "Animation";
    pub const CHOICE1: &str = "Choice1";
    pub const CHOICE2: &str = "Choice2";
    pub const CHOICE3: &str = "Choice3";
    pub const CHOICE4: &str = "Choice4";
    pub const CHOICE4FLAGS: &str = "Choice4Flags";
    pub const CHOICE4UNKNOWN: &str = "Choice4Unknown";
    pub const COLOR: &str = "Color";
    pub const FONT_FACE: &str = "Font";
    pub const FONT_SIZE: &str = "Size";
    pub const GENDER: &str = "Gender";
    pub const HORSE_ACTIVE: &str = "ActiveHorse";
    pub const HORSE_STABLE: &str = "StableHorse";
    pub const ICON: &str = "Icon";
    pub const PAGE_BREAK: &str = "PageBreak";
    pub const PAUSE: &str = "Pause";
    pub const PAUSE_AUTO: &str = "PauseAuto";
    pub const PAUSE_LONG: &str = "PauseLong";
    pub const PAUSE_MID: &str = "PauseMid";
    pub const PAUSE_SHORT: &str = "PauseShort";
    pub const RUBY: &str = "Ruby";
    pub const SOUND1: &str = "Sound1";
    pub const SOUND2: &str = "Sound2";
    pub const SPSWITCH: &str = "SPSwitch";
}

tag_code_maps! {
    (0x00, 0x00) => tag_name::RUBY,
    (0x00, 0x01) => tag_name::FONT_FACE,
    (0x00, 0x02) => tag_name::FONT_SIZE,
    (0x00, 0x03) => tag_name::COLOR,
    (0x00, 0x04) => tag_name::PAGE_BREAK,
    (0x01, 0x00) => tag_name::PAUSE,
    (0x01, 0x03) => tag_name::PAUSE_AUTO,
    (0x01, 0x04) => tag_name::CHOICE2,
    (0x01, 0x05) => tag_name::CHOICE3,
    (0x01, 0x06) => tag_name::CHOICE4,
    (0x01, 0x07) => tag_name::ICON,
    (0x01, 0x08) => tag_name::CHOICE4FLAGS,
    (0x01, 0x09) => tag_name::CHOICE4UNKNOWN,
    (0x01, 0x0A) => tag_name::CHOICE1,
    (0x02, 0x03) => tag_name::HORSE_ACTIVE,
    (0x02, 0x04) => tag_name::HORSE_STABLE,
    (0x03, 0x01) => tag_name::SOUND1,
    (0x04, 0x01) => tag_name::SOUND2,
    (0x04, 0x02) => tag_name::ANIMATION,
    (0x05, 0x00) => tag_name::PAUSE_SHORT,
    (0x05, 0x01) => tag_name::PAUSE_MID,
    (0x05, 0x02) => tag_name::PAUSE_LONG,
    (0xC9, 0x05) => tag_name::GENDER,
    (0xC9, 0x06) => tag_name::SPSWITCH
}

pub const FONT_FACES: [(&str, &str); 2] = [
    ("0", "hylian"), ("65535", "unset")
];

pub const COLOR_NAMES: [(&str, &str); 7] = [
    ("0", "red"), ("1", "green"), ("2", "blue"), ("3", "gray"),
    ("4", "white"), ("5", "orange"), ("65535", "unset")
];

fn new_params(group: u16, tag: u16) -> Vec<Param> {
    let name = tag_codes_to_name(group, tag);
    let mut params: Vec<Param> = Vec::new();
    match name.as_str() {
        tag_name::RUBY => params.extend([
            Param::new("width", Value::U16(0)),
            Param::new("rt", Value::String("".to_string()))
        ].to_vec()),
        tag_name::FONT_FACE => params.extend([
            Param::new_mapped("face", Value::U16(0), &FONT_FACES)
        ].to_vec()),
        tag_name::FONT_SIZE => params.extend([
            Param::new("percent", Value::U16(0))
        ].to_vec()),
        tag_name::COLOR => params.extend([
            Param::new_mapped("name", Value::U16(0), &COLOR_NAMES)
        ].to_vec()),
        tag_name::PAUSE | tag_name::PAUSE_AUTO => params.extend([
            Param::new("frames", Value::U16(0)),
            Param::new_stubbed(Value::U16(0))
        ].to_vec()),
        tag_name::CHOICE2 => params.extend([
            Param::new("label1", Value::U16(0)),
            Param::new("label2", Value::U16(0)),
            Param::new("select_idx", Value::U8(0)),
            Param::new("cancel_idx", Value::U8(0))
        ].to_vec()),
        tag_name::CHOICE3 => params.extend([
            Param::new("label1", Value::U16(0)),
            Param::new("label2", Value::U16(0)),
            Param::new("label3", Value::U16(0)),
            Param::new("select_idx", Value::U8(0)),
            Param::new("cancel_idx", Value::U8(0))
        ].to_vec()),
        tag_name::CHOICE4 => params.extend([
            Param::new("label1", Value::U16(0)),
            Param::new("label2", Value::U16(0)),
            Param::new("label3", Value::U16(0)),
            Param::new("label4", Value::U16(0)),
            Param::new("select_idx", Value::U8(0)),
            Param::new("cancel_idx", Value::U8(0))
        ].to_vec()),
        tag_name::ICON | tag_name::SOUND2 => params.extend([
            Param::new("id", Value::U8(0)),
            Param::new_stubbed(Value::Bytes(1, vec![0xCD])),
        ].to_vec()),
        tag_name::CHOICE4FLAGS => params.extend([
            Param::new("label1", Value::U16(0)),
            Param::new("flag1", Value::String("".to_string())),
            Param::new("label2", Value::U16(0)),
            Param::new("flag2", Value::String("".to_string())),
            Param::new("label3", Value::U16(0)),
            Param::new("flag3", Value::String("".to_string())),
            Param::new("label4", Value::U16(0)),
            Param::new("flag4", Value::String("".to_string())),
            Param::new("select_idx", Value::U8(0)),
            Param::new("cancel_idx", Value::U8(0))
        ].to_vec()),
        tag_name::CHOICE4UNKNOWN => params.extend([
            Param::new("label1", Value::U16(0)),
            Param::new("flag1", Value::String("".to_string())),
            Param::new("label2", Value::U16(0)),
            Param::new("flag2", Value::String("".to_string())),
            Param::new("label3", Value::U16(0)),
            Param::new("flag3", Value::String("".to_string())),
            Param::new("label4", Value::U16(0)),
            Param::new("flag4", Value::String("".to_string())),
            Param::new("unk5", Value::U16(0)),
            Param::new("name5", Value::String("".to_string()))
        ].to_vec()),
        tag_name::CHOICE1 => params.extend([
            Param::new("label", Value::U16(0)),
            Param::new_stubbed(Value::Bytes(2, vec![0x01, 0xCD]))
        ].to_vec()),
        tag_name::ANIMATION => params.extend([
            Param::new("name", Value::String("".to_string()))
        ].to_vec()),
        tag_name::GENDER => params.extend([
            Param::new("masculine", Value::String("".to_string())),
            Param::new("feminine", Value::String("".to_string())),
            Param::new("unk", Value::String("".to_string()))
        ].to_vec()),
        tag_name::SPSWITCH => params.extend([
            Param::new("singular", Value::String("".to_string())),
            Param::new("plural", Value::String("".to_string())),
            Param::new("plural2", Value::String("".to_string()))
        ].to_vec()),
        _ => ()
    };

    if group == 0x02 {
        match tag {
            0x01 | 0x02 | 0x09 | 0x0B | 0x0C | 0x0E |
            0x0F | 0x10 | 0x11 | 0x12 | 0x13 => params.extend([
                Param::new("name", Value::String("".to_string())),
                Param::new_stubbed(Value::U16(0))
            ].to_vec()),
            _ => ()
        };
    };

    params
}

#[derive(Clone)]
pub struct Param {
    pub name: String,
    pub value: Value,
    pub stubbed: bool,
    pub map: Vec<(&'static str, &'static str)>
}

impl Param {
    pub fn new(name: &str, value: Value) -> Self {
        Self{
            name: name.to_string(), value: value,
            stubbed: false,
            map: Vec::new()
        }
    }
    pub fn new_mapped(name: &str, value: Value, map: &[(&'static str, &'static str)]) -> Self {
        Self{
            map: map.to_vec(), ..Self::new(name, value)
        }
    }
    pub fn new_stubbed(value: Value) -> Self {
        Self {
            stubbed: true, ..Self::new("stubbed", value)
        }
    }
    pub fn apply_bytes<R>(&mut self, rdr: &mut R)
    where R: Read + Seek {
        let mut reader = byteordered::ByteOrdered::le(rdr);
        match self.value {
            Value::U8(ref mut n) => *n = reader.read_u8().expect("param: expected u8"),
            Value::U16(ref mut n) => *n = reader.read_u16().expect("param: expected u16"),
            Value::String(ref mut s) => {
                let len = reader.read_u16().expect("param: expected string len byte");
                let mut val = vec![0; len as usize];
                reader.read_exact(&mut val).expect("param: expected string of specified len");
                let val_u16: Vec<u16> = val.chunks_exact(2)
                    .map(|x| u16::from_le_bytes([x[0], x[1]])).collect();
                *s = String::from_utf16_lossy(&val_u16);
            },
            Value::Bytes(len, ref mut v) => {
                let mut val = vec![0; len as usize];
                reader.read_exact(&mut val).expect("param: expected bytes of specified len");
                *v = val;
            }
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut writer = byteordered::ByteOrdered::le(&mut result);
        let sval = self.value.to_string();
        let sv = self.map.iter()
            .find(|m| m.1 == sval)
            .map(|m| m.0);
        match self.value {
            Value::U8(n) => {
                let n = sv.and_then(|qq| qq.parse().ok()).unwrap_or(n);
                writer.write_u8(n).unwrap();
            },
            Value::U16(n) => {
                let n = sv.and_then(|qq| qq.parse().ok()).unwrap_or(n);
                writer.write_u16(n).unwrap();
            },
            Value::String(ref s) => {
                let bytes_u16: Vec<u16> = s.encode_utf16().collect();
                let bytes_u8: Vec<u8> = bytes_u16.iter().flat_map(|u| Vec::from(u.to_le_bytes())).collect();
                writer.write_u16(bytes_u8.len() as u16).unwrap();
                writer.write(&bytes_u8).unwrap();
            },
            Value::Bytes(_len, ref v) => {
                let v = sv.and_then(|qq| hex::decode(qq).ok()).unwrap_or(v.to_vec());
                writer.write_all(&v).unwrap()
            }
        }
        result
    }
    pub fn to_string(&self) -> String {
        if self.stubbed {
            return "".to_string()
        }
        let sval = self.value.to_string();
        let value = self.map.iter()
            .find(|m| m.0 == sval)
            .map(|m| m.1).unwrap_or(sval.as_str());
        if value.is_empty() {
            return format!("");
        }
        format!("{}=\"{}\" ", self.name, value)
    }
}

#[derive(Clone)]
pub enum Value {
    U8(u8),
    U16(u16),
    String(String),
    Bytes(u16, Vec<u8>)
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Self::U8(n) => n.to_string(),
            Self::U16(n) => n.to_string(),
            Self::String(s) => s.to_string(),
            Self::Bytes(_len, v) => hex::encode_upper(&v)
        }
    }
}

pub fn tokens_to_string(msbt: &Msbt, message: &mut PotMessage, value: &[Token]) -> String {
    let mut name = "".to_string();
    let result = value.iter().map(|t| {
        match t {
            Token::TagStart(group, tag, _params) => {
                name = tag_codes_to_name(*group, *tag);
                let mut rdr = std::io::Cursor::new(_params);
                let mut params = new_params(*group, *tag);
                if _params.len() > 0 && params.len() == 0 {
                    params.push(Param::new("bytes", Value::Bytes(_params.len() as u16, Vec::new())))
                }
                for p in &mut params {
                    p.apply_bytes(&mut rdr);
                }
                format!("[{} {}]", name, params.iter().map(|p| p.to_string()).collect::<String>())
            },
            Token::Text(b) => {
                match msbt.header().encoding() {
                    Encoding::Utf16 => String::from_utf16_lossy(
                        &b.chunks(2)
                            .map(|bs| u16::from_le_bytes([bs[0], bs[1]]))
                            .collect::<Vec<u16>>()
                    ),
                    _ => String::from_utf8_lossy(b).to_string()
                }
            },
            Token::TagEnd => format!("[/{} ]", name),
            _ => "".to_string()
        }
    }).collect::<String>();
    result
}

pub fn string_to_tokens<R: std::io::BufRead + std::io::Read + std::io::Seek>(reader: &mut R) -> Vec<Token> {
    let mut rdr = byteordered::ByteOrdered::le(reader);
    let mut tokens: Vec<Token> = Vec::new();
    let mut prev_bytes = Vec::new();

    let params_re = Regex::new(r#"([^=\s]+)="([^"\\]*(?:\\.[^"\\]*)*)"#).unwrap();

    while let Ok(byte) = rdr.read_u8() {
        if byte != b'[' {
            prev_bytes.push(byte);
            continue;
        }

        let tag_start = rdr.stream_position().unwrap();
        let is_closing = rdr.read_u8().unwrap() == b'/';
        if !is_closing {
            rdr.seek(std::io::SeekFrom::Current(-1)).unwrap();
        }

        if !prev_bytes.is_empty() {
            tokens.push(Token::Text(prev_bytes));
            prev_bytes = Vec::new();
        }

        let mut opening_u8 = Vec::new();
        rdr.read_until(b']', &mut opening_u8).unwrap();

        let cnts = String::from_utf8_lossy(&opening_u8[..opening_u8.len()-1]);
        let mut parts = cnts.splitn(2, " ");
        let tag_name = parts.next().unwrap();

        if is_closing {
            tokens.push(Token::TagEnd);
        } else {
            let codes = match tag_name_to_codes(&tag_name) {
                Some(t) => t,
                _ => {
                    prev_bytes.push(b'[');
                    rdr.seek(std::io::SeekFrom::Start(tag_start)).unwrap();
                    continue
                }
            };

            let mut params = new_params(codes.0, codes.1);
            let params_str = parts.next().unwrap();
            for cap in params_re.captures_iter(params_str) {
                let pp = params.iter_mut().find(|p| p.name == &cap[1]);
                if let Some(p) = pp {
                    if let Value::String(ref mut s) = p.value {
                        *s = cap[2].replace("\\\"", "\"")
                            .replace("\\r", "\r").replace("\\t", "\t").to_string();
                    }
                }
            }
            tokens.push(Token::TagStart(codes.0, codes.1, params.iter().flat_map(|p| p.to_bytes()).collect()));
        }
    }

    tokens.push(Token::Text(prev_bytes));
    tokens
}
