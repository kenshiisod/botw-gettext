use msbt::{Msbt, Encoding};
use potty::PotMessage;
use msbt::section::txt2::Token;
use std::io::BufReader;
use std::fs::File;
use potty_msbt::{
    param_u16, param_str, param_bytes,
    tag_code_maps,
    helper::{Param, Value}
};

tag_code_maps! {
    (0x00, 0x00, SYSTEM_RUBY, "System:Ruby"),
    (0x00, 0x01, SYSTEM_FONT, "System:Font"),
    (0x00, 0x02, SYSTEM_SIZE, "System:Size"),
    (0x00, 0x03, SYSTEM_COLOR, "System:Color"),
    (0x00, 0x04, SYSTEM_PAGEBREAK, "System:PageBreak"),
    (0x01, 0x00, USERNAME_MINE, "UserName:MyName"),
    (0x01, 0x01, USERNAME_TARGET, "UserName:TargetName"),
    (0x01, 0x02, USERNAME_SENDER, "UserName:SenderName"),
    (0x01, 0x03, USERNAME_RECEIVER, "UserName:ReceiverName"),
    (0x02, 0x00, DAYTIME_MONTH, "DayTime:Month"),
    (0x02, 0x01, DAYTIME_DAY, "DayTime:Day"),
    (0x02, 0x02, DAYTIME_HOUR, "DayTime:Hour"),
    (0x02, 0x03, DAYTIME_MINUTE, "DayTime:Minute"),
    (0x03, 0x00, SOFTINFO_TITLEINFO, "SoftInfo:TitleInfoTag"),
    (0x03, 0x01, SOFTINFO_NAME, "SoftInfo:NameTag"),
    (0x03, 0x02, SOFTINFO_SIZE, "SoftInfo:SizeTag"),
    (0x03, 0x03, SOFTINFO_STATUS, "SoftInfo:StatusTag"),
    (0x03, 0x04, SOFTINFO_PUB, "SoftInfo:PubTag"),
    (0x03, 0x05, SOFTINFO_OTHERNUMBER, "SoftInfo:otherNumber"),
    (0x03, 0x06, SOFTINFO_OTHERNUMBER2, "SoftInfo:otherNumber2"),
    (0x03, 0x07, SOFTINFO_DSIWARE_SD, "SoftInfo:DSiWare_SD"),
    (0x04, 0x00, SETTINGINFO_NAME, "SettingInfo:NameTag"),
    (0x04, 0x01, SETTINGINFO_NUMFRIEND_SENDER, "SettingInfo:NumFriend_Sender"),
    (0x04, 0x02, SETTINGINFO_NUMFRIEND_RECEIVER, "SettingInfo:NumFriend_Receiver"),
    (0x05, 0x00, NANDINFO_SENDTOTAL_TWL, "NandInfo:SendTotal_TWL"),
    (0x05, 0x01, NANDINFO_RECEIVEREST_TWL, "NandInfo:ReceiveRest_TWL"),
    (0x06, 0x00, POINTINFO_RESTNUMBER, "PointInfo:RestNumberTag"),
    (0x06, 0x01, POINTINFO_SENDPOINT, "PointInfo:SendPointTag"),
    (0x06, 0x02, POINTINFO_UNIT, "PointInfo:UnitTag"),
    (0x07, 0x00, DATAINFO_NAME, "DataInfo:NameTag"),
    (0x07, 0x01, DATAINFO_SIZE, "DataInfo:SizeTag"),
    (0x08, 0x00, ERRORINFO_ERRORCODE, "ErrorInfo:ErrorCode"),
    (0x09, 0x00, BLINK_TIMETAG, "Blink:TimeTag"),
    (0x0A, 0x00, PROGRESSINFO_INDEX, "ProgressInfo:Index"),
    (0x0A, 0x01, PROGRESSINFO_TOTAL, "ProgressInfo:Total"),
    (0x0B, 0x00, MIGRATEINFO_INTERVAL, "MigrateInfo:Interval"),
    (0x0B, 0x01, MIGRATEINFO_RESTNUM, "MigrateInfo:restNum")
}

pub const COLOR_NAMES: [(&str, &str); 4] = [
    ("0", "white"), ("1", "red"), ("2", "blue"), ("65535", "unset")
];

fn new_params(group: u16, tag: u16) -> Vec<Param> {
    let name = tag_codes_to_name(group, tag);
    let params = match name.as_str() {
        tag_name::SYSTEM_RUBY => vec![
            param_u16!("width"),
            param_str!("rt")
        ],
        tag_name::SYSTEM_FONT => vec![
            param_u16!("face", 0, &[])
        ],
        tag_name::SYSTEM_SIZE => vec![
            param_u16!("percent")
        ],
        tag_name::SYSTEM_COLOR => vec![
            param_u16!("name", 0, &COLOR_NAMES)
        ],
        tag_name::BLINK_TIMETAG => vec![
            param_u16!("Interval", 0)
        ],
        tag_name::PROGRESSINFO_INDEX | tag_name::PROGRESSINFO_TOTAL => vec![
            param_u16!("keta", 0)
        ],
        _ => vec![]
    };

    params
}

pub fn po_value_from_msbt(msbt: &Msbt, message: &mut PotMessage, value: &[Token]) {
    let mut name = "".to_string();
    let result = value.iter().map(|t| {
        match t {
            Token::TagStart(group, tag, _params) => {
                name = tag_codes_to_name(*group, *tag);
                let mut rdr = std::io::Cursor::new(_params);
                let mut params = new_params(*group, *tag);
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

fn main() -> std::io::Result<()> {
    for arg in std::env::args().skip(1) {
        let file_name = &arg.strip_suffix(".msbt").unwrap();
        let file_msbt = File::open(format!("{}.msbt", file_name))?;
        let mut reader = BufReader::new(file_msbt);
        let pot = potty_msbt::po_from_msbt(&mut reader, po_value_from_msbt);
        let mut file_po = File::create(format!("{}.po", file_name))?;
        pot.write(&mut file_po)?;
    }

    Ok(())
}
