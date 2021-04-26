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
    [0x00, 0, 0x00, 0] => "System:Ruby",
    [0x00, 0, 0x01, 0] => "System:Font",
    [0x00, 0, 0x02, 0] => "System:Size",
    [0x00, 0, 0x03, 0] => "System:Color",
    [0x00, 0, 0x04, 0] => "System:PageBreak",
    [0x01, 0, 0x00, 0] => "UserName:MyName",
    [0x01, 0, 0x01, 0] => "UserName:TargetName",
    [0x01, 0, 0x02, 0] => "UserName:SenderName",
    [0x01, 0, 0x03, 0] => "UserName:ReceiverName",
    [0x02, 0, 0x00, 0] => "DayTime:Month",
    [0x02, 0, 0x01, 0] => "DayTime:Day",
    [0x02, 0, 0x02, 0] => "DayTime:Hour",
    [0x02, 0, 0x03, 0] => "DayTime:Minute",
    [0x03, 0, 0x00, 0] => "SoftInfo:TitleInfoTag",
    [0x03, 0, 0x01, 0] => "SoftInfo:NameTag",
    [0x03, 0, 0x02, 0] => "SoftInfo:SizeTag",
    [0x03, 0, 0x03, 0] => "SoftInfo:StatusTag",
    [0x03, 0, 0x04, 0] => "SoftInfo:PubTag",
    [0x03, 0, 0x05, 0] => "SoftInfo:otherNumber",
    [0x03, 0, 0x06, 0] => "SoftInfo:otherNumber2",
    [0x03, 0, 0x07, 0] => "SoftInfo:DSiWare_SD",
    [0x04, 0, 0x00, 0] => "SettingInfo:NameTag",
    [0x04, 0, 0x01, 0] => "SettingInfo:NumFriend_Sender",
    [0x04, 0, 0x02, 0] => "SettingInfo:NumFriend_Receiver",
    [0x05, 0, 0x00, 0] => "NandInfo:SendTotal_TWL",
    [0x05, 0, 0x01, 0] => "NandInfo:ReceiveRest_TWL",
    [0x06, 0, 0x00, 0] => "PointInfo:RestNumberTag",
    [0x06, 0, 0x01, 0] => "PointInfo:SendPointTag",
    [0x06, 0, 0x02, 0] => "PointInfo:UnitTag",
    [0x07, 0, 0x00, 0] => "DataInfo:NameTag",
    [0x07, 0, 0x01, 0] => "DataInfo:SizeTag",
    [0x08, 0, 0x00, 0] => "ErrorInfo:ErrorCode",
    [0x09, 0, 0x00, 0] => "Blink:TimeTag",
    [0x0A, 0, 0x00, 0] => "ProgressInfo:Index",
    [0x0A, 0, 0x01, 0] => "ProgressInfo:Total",
    [0x0B, 0, 0x00, 0] => "MigrateInfo:Interval",
    [0x0B, 0, 0x01, 0] => "MigrateInfo:restNum"
}

fn new_tag(name: &str, bytes: &[u8]) -> Tag {
    let mut tag = Tag::new(name, &bytes);
    let params = new_tag_params(&tag);
    tag.params = params;
    tag
}

fn new_tag_params(tag: &Tag) -> Vec<Param> {
    let mut params = Vec::new();
    match tag.name.as_str() {
        "System:Ruby" => {
            params.push(Param::new("width", ParamKind::U16));
            params.push(Param::new("rt", ParamKind::String));
        },
        "System:Font" => {
            params.push(Param::new_mapped("face", ParamKind::U16, &[]));
        },
        "System:Size" => {
            params.push(Param::new("percent", ParamKind::U16));
        },
        "System:Color" => {
            params.push(Param::new_mapped("name", ParamKind::U16, &[
                ("0", "White"), ("1", "Red"), ("2", "Blue")
            ]));
        },
        "Blink:Time" => {
            params.push(Param::new("Interval", ParamKind::U16));
        },
        "ProgressInfo:Index" | "ProgressInfo:Total" => {
            params.push(Param::new("keta", ParamKind::U16));
        },
        _ => ()
    };
    params
}

pub fn new_tag_by_name(name: &str) -> Tag {
    let bytes = match TAG_NAME_TO_BYTES.get(name) {
        Some(b) => b.to_vec(),
        _ => {
            let mut group_tag = name.split(":");
            let group = group_tag.next().unwrap_or("");
            let tag = group_tag.next().unwrap_or("");
            let mut result = Vec::new();
            result.extend(hex::decode(group).unwrap());
            result.extend(hex::decode(tag).unwrap());
            result.to_vec()
        }
    };
    new_tag(name, &bytes)
}

pub fn new_tag_by_bytes(bytes: [u8; 4]) -> Tag {
    let name = match TAG_BYTES_TO_NAME.get(&bytes) {
        Some(s) => s.to_string(),
        _ => {
            let mut result = String::new();
            result.push_str(&hex::encode_upper(&bytes[..2]));
            result.push(':');
            result.push_str(&hex::encode_upper(&bytes[2..]));
            result
        }
    };
    new_tag(name.as_str(), &bytes)
}
