extern crate potty_msbt;

use std::str::FromStr;
use potty::{PotMessage, PotComment};
use std::io::{BufReader, Result};
use std::fs::File;
use std::io::Cursor;
use potty_msbt::{game, msbtw::Node};
use msbt::{Msbt};
use byteordered::Endianness;

fn bytes_to_string(msbt: &Msbt, message: &mut PotMessage, raw_value: Option<&[u8]>) -> String {
    let mut raw_value = raw_value.unwrap().to_vec();
    if let Endianness::Big = msbt.header().endianness() {
        for chunk in raw_value.chunks_exact_mut(2) {
            chunk.swap(0, 1);
        }
    }
    let mut ruby = false;
    let mut reader = Cursor::new(&raw_value);
    let result = game::bytes_to_nodes(&mut reader).iter().map(|n| {
        if let Node::Tag(t) = n {
            // strip the ruby tags and flag the string as containing them
            if &t.bytes == &[0x00, 0, 0x00, 0] { // System:Color
                ruby = true;
                return t.contents.iter().map(|nn| nn.to_string()).collect()
            }
        }
        n.to_string()
    }).collect::<String>();
    if ruby {
        message.comments.push(PotComment::from_str("#,ruby").unwrap());
    }
    result
}

fn string_to_bytes(message: &PotMessage) -> Vec<u8> {
    let mut reader = Cursor::new(&message.strings[0]);
    let mut bytes: Vec<u8> = game::bbcode_to_nodes(&mut reader)
        .iter().flat_map(|n| n.to_bytes().into_iter()).collect();
    bytes.extend(&0_u16.to_le_bytes());
    // if let Endianness::Big = msbt.header().endianness() {
    //     for chunk in bytes.chunks_exact_mut(2) {
    //         chunk.swap(0, 1);
    //     }
    // }
    bytes
}

fn main() -> Result<()> {
    // for arg in std::env::args().skip(1) {
    //     let file_name = &arg.strip_suffix(".msbt").unwrap();
    //     let file_msbt = File::open(format!("{}.msbt", file_name))?;
    //     let mut reader = BufReader::new(file_msbt);
    //     let pot = potty_msbt::po_from_msbt(&mut reader, bytes_to_string);
    //     let mut file_po = File::create(format!("{}.po", file_name))?;
    //     pot.write(&mut file_po)?;
    // }

    for arg in std::env::args().skip(1) {
        let file_name = &arg.strip_suffix(".po").unwrap();
        let file_po = File::open(format!("{}.po", file_name))?;
        let mut reader = BufReader::new(file_po);
        let msbt = potty_msbt::msbt_from_po(&mut reader, string_to_bytes);
        let mut file_msbt = File::create(format!("{}.po", file_name))?;
        msbt.
        msbt(&mut file_msbt)?;
    }

    Ok(())
}
