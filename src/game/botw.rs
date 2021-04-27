use crate::msbtw::{Tag, Param, Value};
use phf::{phf_map};

pub const MARKER_START: u16 = 0x0E;
pub const MARKER_END: u16 = 0x0F;

macro_rules! tag_bimap_static {
    ($($matcher:pat => $result:expr),*) => {
        pub static TAG_BYTES_TO_NAME: phf::Map<[u8; 4], &str> = phf_map! {
            $($matcher => $result),*
        };
        pub static TAG_NAME_TO_BYTES: phf::Map<&str, [u8; 4]> = phf_map! {
            $($result => $matcher),*
        };
    }
}

tag_bimap_static! {
    [0x00, 0, 0x00, 0] => "Ruby",
    [0x00, 0, 0x01, 0] => "Font",
    [0x00, 0, 0x02, 0] => "Size",
    [0x00, 0, 0x03, 0] => "Color",
    [0x00, 0, 0x04, 0] => "PageBreak",
    [0x01, 0, 0x00, 0] => "Pause",
    [0x01, 0, 0x03, 0] => "PauseAuto",
    [0x01, 0, 0x04, 0] => "Choice2",
    [0x01, 0, 0x05, 0] => "Choice3",
    [0x01, 0, 0x06, 0] => "Choice4",
    [0x01, 0, 0x07, 0] => "Icon",
    [0x01, 0, 0x08, 0] => "Choice4Flags",
    [0x01, 0, 0x09, 0] => "Choice4Unknown",
    [0x01, 0, 0x0A, 0] => "Choice1",
    [0x02, 0, 0x03, 0] => "ActiveHorse",
    [0x02, 0, 0x04, 0] => "StableHorse",
    [0x03, 0, 0x01, 0] => "Sound1",
    [0x04, 0, 0x01, 0] => "Sound2",
    [0x04, 0, 0x02, 0] => "Animation",
    [0x05, 0, 0x00, 0] => "PauseShort",
    [0x05, 0, 0x01, 0] => "PauseMid",
    [0x05, 0, 0x02, 0] => "PauseLong",
    [0xC9, 0, 0x05, 0] => "Gender",
    [0xC9, 0, 0x06, 0] => "SPSwitch"
}

fn new_tag(name: &str, bytes: &[u8]) -> Tag {
    let mut tag = Tag::new(name, &bytes);
    let params = new_tag_params(&tag);
    tag.params = params;
    tag
}

pub const FONT_FACES: [(&str, &str); 2] = [
    ("0", "hylian"), ("65535", "unset")
];

pub const COLOR_NAMES: [(&str, &str); 7] = [
    ("0", "red"), ("1", "green"), ("2", "blue"), ("3", "gray"),
    ("4", "white"), ("5", "orange"), ("65535", "unset")
];

fn new_tag_params(tag: &Tag) -> Vec<Param> {
    let group_byte = tag.bytes[0];
    let tag_byte = tag.bytes[2];
    match group_byte {
        0x00 => match tag_byte {
            0x00 => vec![
                Param::new("width", Value::U16(0)),
                Param::new("rt", Value::String("".to_string()))
            ],
            0x01 => vec![
                Param::new_mapped("face", Value::U16(0), &FONT_FACES)
            ],
            0x02 => vec![
                Param::new("percent", Value::U16(0))
            ],
            0x03 => vec![
                Param::new_mapped("name", Value::U16(0), &COLOR_NAMES)
            ],
            _ => vec![]
        },
        0x01 => match tag_byte {
            0x00 | 0x03 => vec![
                Param::new("frames", Value::U16(0)),
                Param::new_stubbed(Value::U16(0))
            ],
            0x04 => vec![
                Param::new("label1", Value::U16(0)),
                Param::new("label2", Value::U16(0)),
                Param::new("select_idx", Value::U8(0)),
                Param::new("cancel_idx", Value::U8(0))
            ],
            0x05 => vec![
                Param::new("label1", Value::U16(0)),
                Param::new("label2", Value::U16(0)),
                Param::new("label3", Value::U16(0)),
                Param::new("select_idx", Value::U8(0)),
                Param::new("cancel_idx", Value::U8(0))
            ],
            0x06 => vec![
                Param::new("label1", Value::U16(0)),
                Param::new("label2", Value::U16(0)),
                Param::new("label3", Value::U16(0)),
                Param::new("label4", Value::U16(0)),
                Param::new("select_idx", Value::U8(0)),
                Param::new("cancel_idx", Value::U8(0))
            ],
            0x07 => vec![
                Param::new("id", Value::U8(0)),
                Param::new_stubbed(Value::Bytes(1, vec![0xCD])),
            ],
            0x08 => vec![
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
            ],
            0x09 => vec![
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
            ],
            0x0A => vec![
                Param::new("label", Value::U16(0)),
                Param::new_stubbed(Value::Bytes(2, vec![0x01, 0xCD]))
            ],
            _ => vec![]
        },
        0x02 => match tag_byte {
            0x01 | 0x02 | 0x09 | 0x0B | 0x0C | 0x0E |
            0x0F | 0x10 | 0x11 | 0x12 | 0x13 => vec![
                Param::new("name", Value::String("".to_string())),
                Param::new_stubbed(Value::U16(0))
            ],
            _ => vec![]
        },
        0x04 => match tag_byte {
            0x01 => vec![
                Param::new("id", Value::U8(0)),
                Param::new_stubbed(Value::Bytes(1, vec![0xCD]))
            ],
            0x02 => vec![
                Param::new("name", Value::String("".to_string()))
            ],
            _ => vec![]
        },
        0xC9 => match tag_byte {
            0x05 => vec![
                Param::new("masculine", Value::String("".to_string())),
                Param::new("feminine", Value::String("".to_string())),
                Param::new("unk", Value::String("".to_string()))
            ],
            0x06 => vec![
                Param::new("singular", Value::String("".to_string())),
                Param::new("plural", Value::String("".to_string())),
                Param::new("plural2", Value::String("".to_string()))
            ],
            _ => vec![]
        },
        _ => vec![]
    }
}

pub fn new_tag_by_name(name: &str) -> Option<Tag> {
    let bytes = match TAG_NAME_TO_BYTES.get(name) {
        Some(b) => b.to_vec(),
        _ => {
            let mut group_tag = name.split(":");
            let group = group_tag.next()?;
            let tag = group_tag.next()?;
            let mut result = hex::decode(group).ok()?;
            result.push(0);
            result.extend(hex::decode(tag).unwrap());
            result.push(0);
            result
        }
    };
    Some(new_tag(name, &bytes))
}

pub fn new_tag_by_bytes(bytes: [u8; 4]) -> Tag {
    let name = match TAG_BYTES_TO_NAME.get(&bytes) {
        Some(s) => s.to_string(),
        _ => format!("{:02X}:{:02X}", bytes[0], bytes[2])
    };
    new_tag(name.as_str(), &bytes)
}
