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
    pub const SYSTEM_RUBY: &str = "System:Ruby";
    pub const SYSTEM_FONT: &str = "System:Font";
    pub const SYSTEM_SIZE: &str = "System:Size";
    pub const SYSTEM_COLOR: &str = "System:Color";
    pub const SYSTEM_PAGEBREAK: &str = "System:PageBreak";
    pub const USERNAME_MINE: &str = "UserName:MyName";
    pub const USERNAME_TARGET: &str = "UserName:TargetName";
    pub const USERNAME_SENDER: &str = "UserName:SenderName";
    pub const USERNAME_RECEIVER: &str = "UserName:ReceiverName";
    pub const DAYTIME_MONTH: &str = "DayTime:Month";
    pub const DAYTIME_DAY: &str = "DayTime:Day";
    pub const DAYTIME_HOUR: &str = "DayTime:Hour";
    pub const DAYTIME_MINUTE: &str = "DayTime:Minute";
    pub const SOFTINFO_TITLEINFO: &str = "SoftInfo:TitleInfoTag";
    pub const SOFTINFO_NAME: &str = "SoftInfo:NameTag";
    pub const SOFTINFO_SIZE: &str = "SoftInfo:SizeTag";
    pub const SOFTINFO_STATUS: &str = "SoftInfo:StatusTag";
    pub const SOFTINFO_PUB: &str = "SoftInfo:PubTag";
    pub const SOFTINFO_OTHERNUMBER: &str = "SoftInfo:otherNumber";
    pub const SOFTINFO_OTHERNUMBER2: &str = "SoftInfo:otherNumber2";
    pub const SOFTINFO_DSIWARE_SD: &str = "SoftInfo:DSiWare_SD";
    pub const SETTINGINFO_NAME: &str = "SettingInfo:NameTag";
    pub const SETTINGINFO_NUMFRIEND_SENDER: &str = "SettingInfo:NumFriend_Sender";
    pub const SETTINGINFO_NUMFRIEND_RECEIVER: &str = "SettingInfo:NumFriend_Receiver";
    pub const NANDINFO_SENDTOTAL_TWL: &str = "NandInfo:SendTotal_TWL";
    pub const NANDINFO_RECEIVEREST_TWL: &str = "NandInfo:ReceiveRest_TWL";
    pub const POINTINFO_RESTNUMBER: &str = "PointInfo:RestNumberTag";
    pub const POINTINFO_SENDPOINT: &str = "PointInfo:SendPointTag";
    pub const POINTINFO_UNIT: &str = "PointInfo:UnitTag";
    pub const DATAINFO_NAME: &str = "DataInfo:NameTag";
    pub const DATAINFO_SIZE: &str = "DataInfo:SizeTag";
    pub const ERRORINFO_ERRORCODE: &str = "ErrorInfo:ErrorCode";
    pub const BLINK_TIMETAG: &str = "Blink:TimeTag";
    pub const PROGRESSINFO_INDEX: &str = "ProgressInfo:Index";
    pub const PROGRESSINFO_TOTAL: &str = "ProgressInfo:Total";
    pub const MIGRATEINFO_INTERVAL: &str = "MigrateInfo:Interval";
    pub const MIGRATEINFO_RESTNUM: &str = "MigrateInfo:restNum";
}

tag_code_maps! {
    (0x00, 0x00) => tag_name::SYSTEM_RUBY,
    (0x00, 0x01) => tag_name::SYSTEM_FONT,
    (0x00, 0x02) => tag_name::SYSTEM_SIZE,
    (0x00, 0x03) => tag_name::SYSTEM_COLOR,
    (0x00, 0x04) => tag_name::SYSTEM_PAGEBREAK,
    (0x01, 0x00) => tag_name::USERNAME_MINE,
    (0x01, 0x01) => tag_name::USERNAME_TARGET,
    (0x01, 0x02) => tag_name::USERNAME_SENDER,
    (0x01, 0x03) => tag_name::USERNAME_RECEIVER,
    (0x02, 0x00) => tag_name::DAYTIME_MONTH,
    (0x02, 0x01) => tag_name::DAYTIME_DAY,
    (0x02, 0x02) => tag_name::DAYTIME_HOUR,
    (0x02, 0x03) => tag_name::DAYTIME_MINUTE,
    (0x03, 0x00) => tag_name::SOFTINFO_TITLEINFO,
    (0x03, 0x01) => tag_name::SOFTINFO_NAME,
    (0x03, 0x02) => tag_name::SOFTINFO_SIZE,
    (0x03, 0x03) => tag_name::SOFTINFO_STATUS,
    (0x03, 0x04) => tag_name::SOFTINFO_PUB,
    (0x03, 0x05) => tag_name::SOFTINFO_OTHERNUMBER,
    (0x03, 0x06) => tag_name::SOFTINFO_OTHERNUMBER2,
    (0x03, 0x07) => tag_name::SOFTINFO_DSIWARE_SD,
    (0x04, 0x00) => tag_name::SETTINGINFO_NAME,
    (0x04, 0x01) => tag_name::SETTINGINFO_NUMFRIEND_SENDER,
    (0x04, 0x02) => tag_name::SETTINGINFO_NUMFRIEND_RECEIVER,
    (0x05, 0x00) => tag_name::NANDINFO_SENDTOTAL_TWL,
    (0x05, 0x01) => tag_name::NANDINFO_RECEIVEREST_TWL,
    (0x06, 0x00) => tag_name::POINTINFO_RESTNUMBER,
    (0x06, 0x01) => tag_name::POINTINFO_SENDPOINT,
    (0x06, 0x02) => tag_name::POINTINFO_UNIT,
    (0x07, 0x00) => tag_name::DATAINFO_NAME,
    (0x07, 0x01) => tag_name::DATAINFO_SIZE,
    (0x08, 0x00) => tag_name::ERRORINFO_ERRORCODE,
    (0x09, 0x00) => tag_name::BLINK_TIMETAG,
    (0x0A, 0x00) => tag_name::PROGRESSINFO_INDEX,
    (0x0A, 0x01) => tag_name::PROGRESSINFO_TOTAL,
    (0x0B, 0x00) => tag_name::MIGRATEINFO_INTERVAL,
    (0x0B, 0x01) => tag_name::MIGRATEINFO_RESTNUM
}

// fn new_tag(name: &str, bytes: &[u8]) -> Tag {
//     let mut tag = Tag::new(name, &bytes);
//     let params = new_tag_params(&tag);
//     tag.params = params;
//     tag
// }

// pub const COLOR_NAMES: [(&str, &str); 4] = [
//     ("0", "white"), ("1", "red"), ("2", "blue"), ("65535", "unset")
// ];

// fn new_tag_params(tag: &Tag) -> Vec<Param> {
//     match tag.name.as_str() {
//         "System:Ruby" => vec![
//             Param::new("width", Value::U16(0)),
//             Param::new("rt", Value::String("".to_string()))
//         ],
//         "System:Font" => vec![
//             Param::new_mapped("face", Value::U16(0), &[])
//         ],
//         "System:Size" => vec![
//             Param::new("percent", Value::U16(0))
//         ],
//         "System:Color" => vec![
//             Param::new_mapped("name", Value::U16(0), &COLOR_NAMES)
//         ],
//         "Blink:Time" => vec![
//             Param::new("Interval", Value::U16(0))
//         ],
//         "ProgressInfo:Index" | "ProgressInfo:Total" => vec![
//             Param::new("keta", Value::U16(0))
//         ],
//         _ => vec![]
//     }
// }

// pub fn new_tag_by_name(name: &str) -> Tag {
//     let bytes = match TAG_NAME_TO_BYTES.get(name) {
//         Some(b) => b.to_vec(),
//         _ => {
//             let mut group_tag = name.split(":");
//             let group = group_tag.next().unwrap_or("");
//             let tag = group_tag.next().unwrap_or("");
//             let mut result = Vec::new();
//             result.extend(hex::decode(group).unwrap());
//             result.extend(hex::decode(tag).unwrap());
//             result.to_vec()
//         }
//     };
//     new_tag(name, &bytes)
// }

// pub fn new_tag_by_bytes(bytes: [u8; 4]) -> Tag {
//     let name = match TAG_BYTES_TO_NAME.get(&bytes) {
//         Some(s) => s.to_string(),
//         _ => {
//             let mut result = String::new();
//             result.push_str(&hex::encode_upper(&bytes[..2]));
//             result.push(':');
//             result.push_str(&hex::encode_upper(&bytes[2..]));
//             result
//         }
//     };
//     new_tag(name.as_str(), &bytes)
// }
