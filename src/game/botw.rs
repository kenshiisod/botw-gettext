use crate::msbtw::{Node, Tag, Param, ParamKind};
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

fn new_tag_params(tag: &Tag) -> Vec<Param> {
    let mut params = Vec::new();
    let group_byte = tag.bytes[0];
    let tag_byte = tag.bytes[2];
    match group_byte {
        0x00 => match tag_byte {
            0x00 => {
                params.push(Param::new("width", ParamKind::U16));
                params.push(Param::new("rt", ParamKind::String));
            },
            0x01 => {
                params.push(Param::new_mapped("face", ParamKind::U16, &[
                    ("0", "hylian"), ("65535", "unset")
                ]));
            },
            0x02 => params.push(Param::new("percent", ParamKind::U16)),
            0x03 => params.push(Param::new_mapped("name", ParamKind::U16, &[
                ("0", "red"), ("1", "green"), ("2", "blue"), ("3", "Gray"),
                ("4", "white"), ("5", "orange"), ("65535", "unset")
            ])),
            _ => ()
        },
        0x01 => match tag_byte {
            0x00 | 0x03 => {
                params.push(Param::new("frames", ParamKind::U16));
                params.push(Param::new_stubbed(ParamKind::U16, "0"));
            },
            0x04 => {
                params.push(Param::new("label1", ParamKind::U16));
                params.push(Param::new("label2", ParamKind::U16));
                params.push(Param::new("selectednum", ParamKind::U8));
                params.push(Param::new("cancelnum", ParamKind::U8));
            },
            0x05 => {
                params.push(Param::new("label1", ParamKind::U16));
                params.push(Param::new("label2", ParamKind::U16));
                params.push(Param::new("label3", ParamKind::U16));
                params.push(Param::new("selectednum", ParamKind::U8));
                params.push(Param::new("cancelnum", ParamKind::U8));
            },
            0x06 => {
                params.push(Param::new("label1", ParamKind::U16));
                params.push(Param::new("label2", ParamKind::U16));
                params.push(Param::new("label3", ParamKind::U16));
                params.push(Param::new("label4", ParamKind::U16));
                params.push(Param::new("selectednum", ParamKind::U8));
                params.push(Param::new("cancelnum", ParamKind::U8));
            },
            0x07 => {
                params.push(Param::new("id", ParamKind::U8));
                params.push(Param::new_stubbed(ParamKind::Bytes(1), "CD"));
            },
            0x08 => {
                params.push(Param::new("label1", ParamKind::U16));
                params.push(Param::new("flag1", ParamKind::String));
                params.push(Param::new("label2", ParamKind::U16));
                params.push(Param::new("flag2", ParamKind::String));
                params.push(Param::new("label3", ParamKind::U16));
                params.push(Param::new("flag3", ParamKind::String));
                params.push(Param::new("label4", ParamKind::U16));
                params.push(Param::new("flag4", ParamKind::String));
                params.push(Param::new("selectednum", ParamKind::U8));
                params.push(Param::new("cancelnum", ParamKind::U8));
            },
            0x09 => {
                params.push(Param::new("label1", ParamKind::U16));
                params.push(Param::new("flag1", ParamKind::String));
                params.push(Param::new("label2", ParamKind::U16));
                params.push(Param::new("flag2", ParamKind::String));
                params.push(Param::new("label3", ParamKind::U16));
                params.push(Param::new("flag3", ParamKind::String));
                params.push(Param::new("label4", ParamKind::U16));
                params.push(Param::new("flag4", ParamKind::String));
                params.push(Param::new("unk5", ParamKind::U16));
                params.push(Param::new("name5", ParamKind::String));
            },
            0x0A => {
                params.push(Param::new("label", ParamKind::U16));
                params.push(Param::new_stubbed(ParamKind::Bytes(2), "01CD"));
            },
            _ => ()
        },
        0x02 => match tag_byte {
            0x01 | 0x02 | 0x09 | 0x0B | 0x0C | 0x0E | 0x0F | 0x10 | 0x11 | 0x12 | 0x13 => {
                params.push(Param::new("name", ParamKind::String));
                params.push(Param::new_stubbed(ParamKind::U16, "0"));
            },
            _ => ()
        },
        0x04 => match tag_byte {
            0x01 => {
                params.push(Param::new("id", ParamKind::U8));
                params.push(Param::new_stubbed(ParamKind::Bytes(1), "CD"));
            },
            0x02 => params.push(Param::new("name", ParamKind::String)),
            _ => ()
        },
        0xC9 => match tag_byte {
            0x05 => {
                params.push(Param::new("masculine", ParamKind::String));
                params.push(Param::new("feminine", ParamKind::String));
                params.push(Param::new("unk", ParamKind::String));
            },
            0x06 => {
                params.push(Param::new("singular", ParamKind::String));
                params.push(Param::new("plural", ParamKind::String));
                params.push(Param::new("plural2", ParamKind::String));
            },
            _ => ()
        },
        _ => ()
    };
    params
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
