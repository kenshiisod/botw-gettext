use std::io::{Read, Seek, Write};

#[derive(Clone)]
pub enum Node {
    Text(String),
    Tag(Tag)
}

impl Node {
    pub fn to_string(&self) -> String {
        match self {
            Node::Text(s) => s.to_string(),
            Node::Tag(t) => t.to_string()
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Node::Text(s) => {
                let bytes_u16: Vec<u16> = s.encode_utf16().collect();
                let bytes_u8: Vec<u8> = bytes_u16.iter().flat_map(|u| Vec::from(u.to_le_bytes())).collect();
                bytes_u8
            },
            Node::Tag(t) => t.to_bytes()
        }
    }
}

#[derive(Clone)]
pub struct Tag {
    pub name: String,
    pub bytes: Vec<u8>,
    pub params: Vec<Param>,
    pub contents: Vec<Node>
}

impl Tag {
    pub fn new(name: &str, bytes: &[u8]) -> Self {
        Self{
            name: name.to_string(), bytes: bytes.to_vec(),
            params: Vec::new(), contents: Vec::new()
        }
    }
    pub fn new_with_params(name: &str, bytes: &[u8], params: Vec<Param>) -> Self {
        Self{
            name: name.to_string(), bytes: bytes.to_vec(),
            params: params, contents: Vec::new()
        }
    }
    pub fn apply_bytes<R>(&mut self, rdr: &mut R)
    where R: Read + Seek {
        for p in &mut self.params {
            p.apply_bytes(rdr)
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut cursor = byteordered::ByteOrdered::le(&mut result);
        cursor.write_u16(0x0E).unwrap();
        cursor.write_all(&self.bytes).unwrap();
        let param_bytes: Vec<u8> = self.params.iter().flat_map(|p| p.to_bytes().into_iter()).collect();
        cursor.write_u16(param_bytes.len() as u16).unwrap();
        cursor.write_all(&param_bytes).unwrap();
        result
    }
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let params = self.params.iter()
            .map(|ref p| p.to_string()).collect::<String>();
        result.push_str(match params.is_empty() {
            true => format!("[{}]", self.name),
            _ => format!("[{} {}]", self.name, params.trim_end())
        }.as_str());
        if !self.contents.is_empty() {
            result.push_str(&self.contents.iter().map(|c| c.to_string()).collect::<String>());
            result.push_str(format!("[/{}]", self.name).as_str());
        }
        result
    }
}

#[derive(Clone)]
pub enum ParamKind {
    U8,
    U16,
    String,
    Bytes(u16)
}

#[derive(Clone)]
pub struct Param {
    pub name: String,
    pub kind: ParamKind,
    pub value: String,
    pub stubbed: bool,
    pub map: Vec<(&'static str, &'static str)>
}

impl Param {
    pub fn new(name: &str, kind: ParamKind) -> Self {
        let value = match kind {
            ParamKind::U8 | ParamKind::U16
            => "0",
            _ => ""
        };
        Self{
            name: name.to_string(), kind: kind,
            value: value.to_string(), stubbed: false,
            map: Vec::new()
        }
    }
    pub fn new_mapped(name: &str, kind: ParamKind, map: &[(&'static str, &'static str)]) -> Self {
        Self{
            map: map.to_vec(), ..Self::new(name, kind)
        }
    }
    pub fn new_stubbed(kind: ParamKind, value: &str) -> Self {
        Self {
            stubbed: true, value: value.to_string(), ..Self::new("stubbed", kind)
        }
    }
    pub fn apply_bytes<R>(&mut self, rdr: &mut R)
    where R: Read + Seek {
        let value;
        let mut reader = byteordered::ByteOrdered::le(rdr);
        match self.kind {
            ParamKind::U8 => {
                value = format!("{}", reader.read_u8().expect("param: expected u8"));
            },
            ParamKind::U16 => {
                value = format!("{}", reader.read_u16().expect("param: expected u16"));
            },
            ParamKind::String => {
                let len = reader.read_u16().expect("param: expected string len byte");
                let mut val = vec![0; len as usize];
                reader.read_exact(&mut val).expect("param: expected string of specified len");
                let val_u16: Vec<u16> = val.chunks_exact(2)
                    .map(|x| u16::from_le_bytes([x[0], x[1]])).collect();
                value = String::from_utf16_lossy(&val_u16);
            },
            ParamKind::Bytes(len) => {
                let mut val = vec![0; len as usize];
                reader.read_exact(&mut val).expect("param: expected bytes of specified len");
                value = hex::encode_upper(val);
            }
        }
        if !self.stubbed {
            self.value = value;
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut writer = byteordered::ByteOrdered::le(&mut result);
        let sv = self.map.iter()
            .find(|m| m.1 == self.value)
            .map(|m| m.0).unwrap_or(self.value.as_str());
        match self.kind {
            ParamKind::U8 => {
                let value = sv.parse::<u8>().expect("param: expected u8");
                writer.write_u8(value).unwrap();
            },
            ParamKind::U16 => {
                let value = sv.parse::<u16>().expect("param: expected u16");
                writer.write_u16(value).unwrap();
            },
            ParamKind::String => {
                let bytes_u16: Vec<u16> = sv.encode_utf16().collect();
                let bytes_u8: Vec<u8> = bytes_u16.iter().flat_map(|u| Vec::from(u.to_le_bytes())).collect();
                writer.write_u16(bytes_u8.len() as u16).unwrap();
                writer.write(&bytes_u8).unwrap();
            },
            ParamKind::Bytes(len) => {
                if sv.len() / 2 != len.into() { panic!("param: expected bytes of specified len"); }
                writer.write_all(&hex::decode(&sv).unwrap()).unwrap();
            }
        }
        result
    }
    pub fn to_string(&self) -> String {
        if self.stubbed {
            return "".to_string()
        }
        let value = self.map.iter()
            .find(|m| m.0 == self.value)
            .map(|m| m.1).unwrap_or(self.value.as_str());
        if value.is_empty() {
            return format!("");
        }
        format!("{}=\"{}\" ", self.name, value)
    }
}
