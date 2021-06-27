use std::io::{BufReader, BufWriter};
use msbt::{Msbt, Encoding};
use potty::PotMessage;
use msbt::section::txt2::Token;
use std::fs::File;
use potty_msbt::{
    param_u8, param_u16,
    param_str, param_bytes,
    tag_code_maps,
    helper::{Param, Value, self}
};

tag_code_maps! {
    (0x00, 0x00, RUBY, "Ruby"),
    (0x00, 0x01, FONT_FACE, "Font"),
    (0x00, 0x02, FONT_SIZE, "Size"),
    (0x00, 0x03, COLOR, "Color"),
    (0x00, 0x04, PAGE_BREAK, "PageBreak"),
    (0x01, 0x00, PAUSE, "Pause"),
    (0x01, 0x03, PAUSE_AUTO, "PauseAuto"),
    (0x01, 0x04, CHOICE2, "Choice2"),
    (0x01, 0x05, CHOICE3, "Choice3"),
    (0x01, 0x06, CHOICE4, "Choice4"),
    (0x01, 0x07, ICON, "Icon"),
    (0x01, 0x08, CHOICE4FLAGS, "Choice4Flags"),
    (0x01, 0x09, CHOICE4UNKNOWN, "Choice4Unknown"),
    (0x01, 0x0A, CHOICE1, "Choice1"),
    (0x02, 0x03, HORSE_ACTIVE, "ActiveHorse"),
    (0x02, 0x04, HORSE_STABLE, "StableHorse"),
    (0x03, 0x01, SOUND1, "Sound1"),
    (0x04, 0x01, SOUND2, "Sound2"),
    (0x04, 0x02, ANIMATION, "Animation"),
    (0x05, 0x00, PAUSE_SHORT, "PauseShort"),
    (0x05, 0x01, PAUSE_MID, "PauseMid"),
    (0x05, 0x02, PAUSE_LONG, "PauseLong"),
    (0xC9, 0x05, GENDER, "Gender"),
    (0xC9, 0x06, SPSWITCH, "SPSwitch")
}

pub const FONT_FACES: [(&str, &str); 2] = [
    ("0", "hylian"), ("65535", "unset")
];

pub const COLOR_NAMES: [(&str, &str); 7] = [
    ("0", "red"), ("1", "green"), ("2", "blue"), ("3", "gray"),
    ("4", "white"), ("5", "orange"), ("65535", "unset")
];

fn new_params(name: &str) -> Vec<Param> {
    let (group, tag) = tag_name_to_codes(&name).unwrap();
    let mut params = match name {
        tag_name::RUBY => vec![
            param_u16!("width"),
            param_str!("rt")
        ],
        tag_name::FONT_FACE => vec![
            param_u16!("face", 0, &FONT_FACES)
        ],
        tag_name::FONT_SIZE => vec![
            param_u16!("percent")
        ],
        tag_name::COLOR => vec![
            param_u16!("name", 0, &COLOR_NAMES)
        ],
        tag_name::PAUSE | tag_name::PAUSE_AUTO => vec![
            param_u16!("frames"),
            param_u16!("stub")
        ],
        tag_name::CHOICE2 => vec![
            param_u16!("label1"),
            param_u16!("label2"),
            param_u8!("select_idx"),
            param_u8!("cancel_idx")
        ],
        tag_name::CHOICE3 => vec![
            param_u16!("label1"),
            param_u16!("label2"),
            param_u16!("label3"),
            param_u8!("select_idx"),
            param_u8!("cancel_idx")
        ],
        tag_name::CHOICE4 => vec![
            param_u16!("label1"),
            param_u16!("label2"),
            param_u16!("label3"),
            param_u16!("label4"),
            param_u8!("select_idx"),
            param_u8!("cancel_idx")
        ],
        tag_name::ICON | tag_name::SOUND2 => vec![
            param_u8!("id"),
            param_bytes!("stub", vec![0xCD]),
        ],
        tag_name::CHOICE4FLAGS => vec![
            param_u16!("label1"),
            param_str!("flag1"),
            param_u16!("label2"),
            param_str!("flag2"),
            param_u16!("label3"),
            param_str!("flag3"),
            param_u16!("label4"),
            param_str!("flag4"),
            param_u8!("select_idx"),
            param_u8!("cancel_idx")
        ],
        tag_name::CHOICE4UNKNOWN => vec![
            param_u16!("label1"),
            param_str!("flag1"),
            param_u16!("label2"),
            param_str!("flag2"),
            param_u16!("label3"),
            param_str!("flag3"),
            param_u16!("label4"),
            param_str!("flag4"),
            param_u16!("unk5"),
            param_str!("name5")
        ],
        tag_name::CHOICE1 => vec![
            param_u16!("label"),
            param_bytes!("stub", vec![0x01, 0xCD])
        ],
        tag_name::ANIMATION => vec![
            param_str!("name")
        ],
        tag_name::GENDER => vec![
            param_str!("masculine"),
            param_str!("feminine"),
            param_str!("unk")
        ],
        tag_name::SPSWITCH => vec![
            param_str!("singular"),
            param_str!("plural"),
            param_str!("plural2")
        ],
        _ => vec![]
    };

    if group == 0x02 {
        params.extend(match tag {
            0x01 | 0x02 | 0x09 | 0x0B | 0x0C | 0x0E |
            0x0F | 0x10 | 0x11 | 0x12 | 0x13 => vec![
                param_str!("name"),
                param_u16!("")
            ],
            _ => vec![]
        });
    };

    params
}

pub fn po_value_from_msbt(msbt: &Msbt, message: &mut PotMessage, value: &[Token]) {
    helper::po_value_from_msbt(&msbt, message, &value, tag_codes_to_name, new_params)
}

pub fn msbt_value_from_po(message: &PotMessage) -> Vec<Token> {
    let mut curse = std::io::Cursor::new(&message.strings[0]);
    helper::msbt_value_from_po(&mut curse, tag_name_to_codes, new_params)
}

use std::path::Path;
use std::ffi::OsStr;

fn main() -> std::io::Result<()> {
    for arg in std::env::args().skip(1) {
        let from_type = Path::new(&arg).extension().and_then(OsStr::to_str).unwrap();
        match from_type {
            // msbt to po
            "msbt" =>{
                let file_name = &arg.strip_suffix(".msbt").unwrap();
                let file_msbt = File::open(format!("{}.msbt", file_name))?;
                let mut reader = BufReader::new(file_msbt);
                let pot = potty_msbt::po_from_msbt(&mut reader, po_value_from_msbt);
                let mut file_po = File::create(format!("{}.po", file_name))?;
                pot.write(&mut file_po)?;
            },
            // po to msbt
            _ => {
                let file_name = &arg.strip_suffix(".po").unwrap();
                let file_msbt = File::open(format!("{}.po", file_name))?;
                let mut reader = BufReader::new(file_msbt);
                let msbt = potty_msbt::msbt_from_po(&mut reader, msbt_value_from_po);
                let file_msbt = BufWriter::new(File::create(format!("{}.msbt", file_name)).unwrap());
                msbt.write_to(file_msbt).unwrap();
            }
        }
    }
    Ok(())
}
