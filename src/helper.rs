use msbt::{Msbt, Encoding};
use potty::PotMessage;
use regex::Regex;
use msbt::section::txt2::Token;
use std::io::{Read, Seek, BufRead, Write};

#[macro_export]
macro_rules! tag_code_maps {
    ($(($group_code:expr, $code:expr, $var:ident, $tag_name:expr)),*) => {
        pub mod tag_name {
            $(pub const $var: &str = $tag_name;)*
        }
        pub fn tag_codes_to_name(group_code: u16, code: u16) -> String {
            match (group_code, code) {
                $(($group_code, $code) => $tag_name.to_string()),*,
                _ => format!("{:02X}:{:02X}", group_code, code)
            }
        }
        pub fn tag_name_to_codes(name: &str) -> Option<(u16, u16)> {
            match name {
                $($tag_name => Some(($group_code, $code))),*,
                _ => None
            }.or_else(|| {
                let mut group_tag = name.split(":");
                let group_code = hex::decode(group_tag.next()?).ok()?[0];
                let code = hex::decode(group_tag.next()?).ok()?[0];
                Some((group_code as u16, code as u16))
            })
        }
    }
}

#[derive(Clone)]
pub struct Param {
    pub name: String,
    pub value: Value,
    pub map: Vec<(&'static str, &'static str)>
}

impl Param {
    pub fn new(name: &str, value: Value) -> Self {
        Self{
            name: name.to_string(), value: value,
            map: Vec::new()
        }
    }
    pub fn new_mapped(name: &str, value: Value, map: &[(&'static str, &'static str)]) -> Self {
        Self{
            map: map.to_vec(), ..Self::new(name, value)
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
        if self.name == "stub" {
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

#[macro_export]
macro_rules! param_u8 {
    ($name:expr) => { Param::new($name, Value::U8(0)) };
    ($name:expr, $value:expr) => { Param::new($name, Value::U8($value)) };
}

#[macro_export]
macro_rules! param_u16 {
    ($name:expr) => { Param::new($name, Value::U16(0)) };
    ($name:expr, $value:expr) => { Param::new($name, Value::U16($value)) };
    ($name:expr, $value:expr, $map:expr) => { Param::new_mapped($name, Value::U16($value), $map) };
}

#[macro_export]
macro_rules! param_str {
    ($name:expr) => { Param::new($name, Value::String("".to_string())) };
    ($name:expr, $value:expr) => { Param::new($name, Value::String($value.to_string())) };
}

#[macro_export]
macro_rules! param_bytes {
    ($name:expr, $value:expr) => { Param::new($name, Value::Bytes($value.len() as u16, $value)) };
}

fn to_le(s: &[u8]) -> Vec<u8> {
    let s = String::from_utf8_lossy(s);
    let bytes_u16: Vec<u16> = s.encode_utf16().collect();
    let bytes_u8: Vec<u8> = bytes_u16.iter().flat_map(|u| Vec::from(u.to_le_bytes())).collect();
    bytes_u8
}

pub fn msbt_value_from_po<R>(reader: &mut R, name_codes_fn: fn(&str) -> Option<(u16, u16)>, params_fn: fn(&str) -> Vec<Param>) -> Vec<Token>
where R: std::io::BufRead + std::io::Read + std::io::Seek {
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
            tokens.push(Token::Text(to_le(&prev_bytes)));
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
            let codes = match name_codes_fn(&tag_name) {
                Some(t) => t,
                _ => {
                    prev_bytes.push(b'[');
                    rdr.seek(std::io::SeekFrom::Start(tag_start)).unwrap();
                    continue
                }
            };
            let mut params = params_fn(tag_name);
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

    tokens.push(Token::Text(to_le(&prev_bytes)));
    tokens
}

pub fn po_value_from_msbt(msbt: &Msbt, message: &mut PotMessage, value: &[Token], codes_name_fn: fn(u16, u16) -> String, params_fn: fn(&str) -> Vec<Param>) {
    let mut name = "".to_string();
    let result = value.iter().map(|t| {
        match t {
            Token::TagStart(group, tag, _params) => {
                name = codes_name_fn(*group, *tag);
                let mut rdr = std::io::Cursor::new(_params);
                let mut params = params_fn(&name);
                if _params.len() > 0 && params.len() == 0 {
                    params.push(param_bytes!("bytes", vec![0; _params.len()]));
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
    message.strings = vec![result];
}
