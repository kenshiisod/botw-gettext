mod model;
pub mod game;

use byteordered::{Endianness};
use potty::{Pot, PotMessage};
use msbt::{Msbt, Encoding, builder::MsbtBuilder, section::txt2::Token};
use crate::model::{MsbtInfo, Nli1};
use std::io::{Read, Seek};

const EXTRAS_ID: &str = "_ReadOnly_MsbtExtras";
const VERSION_ID: &str = "_ReadOnly_Version";

pub fn po_from_msbt<R: Read + Seek>(reader: &mut R, parse_fn: fn (&Msbt, &mut PotMessage, &[Token]) -> String) -> Pot {
    let msbt = Msbt::from_reader(reader);
    let msbt = msbt.unwrap();
    let lbl1 = msbt.lbl1().unwrap();
    let txt2 = msbt.txt2().unwrap();
    let mut pot = Pot::new();

    for (i, label) in lbl1.labels().iter().enumerate() {
        let mut message = PotMessage::new();
        let value = parse_fn(&msbt, &mut message, &txt2.values()[i]);
        message.id = Some(label.name().to_string());
        message.strings.push(value);
        pot.messages.push(message);
    }

    let extras_obj = MsbtInfo{
        group_count: lbl1.groups().len() as u32,
        atr1: msbt.atr1().map(|a| a.unknown_bytes().to_vec()),
        ato1: msbt.ato1().map(|a| a.unknown_bytes().to_vec()),
        tsy1: msbt.tsy1().map(|a| a.unknown_bytes().to_vec()),
        nli1: msbt.nli1().map(|a| Nli1 {
            id_count: a.id_count(),
            global_ids: a.global_ids().clone(),
        }),
    };

    let mut extras_msg = PotMessage::new();
    extras_msg.id = Some(EXTRAS_ID.to_string());
    let binny = bincode::serialize(&extras_obj).unwrap();
    extras_msg.strings.push(base64::encode(binny));
    pot.messages.push(extras_msg);

    let mut version_msg = PotMessage::new();
    version_msg.id = Some(VERSION_ID.to_string());
    version_msg.strings.push(1.to_string());
    pot.messages.push(version_msg);

    pot
}

pub fn msbt_from_po<R: Read + Seek>(mut reader: &mut R, parse_fn: fn (&PotMessage) -> Vec<Token>) -> Msbt {
    let pot = Pot::read(&mut reader);
    let mut msbt_extras: Option<MsbtInfo> = None;
    let mut _potty_version = "";

    for message in &pot.messages {
        let context = message.context.clone().unwrap_or_default();
        let value: String = message.strings[0].clone();
        if context == EXTRAS_ID {
            msbt_extras = Some(bincode::deserialize(value.as_bytes()).unwrap());
        } else if context == VERSION_ID {
            _potty_version = &value;
        }
    }

    let msbt_extras = msbt_extras.unwrap();
    let mut builder = MsbtBuilder::new(Endianness::Little, Encoding::Utf16, Some(msbt_extras.group_count));
    if let Some(unknown_bytes) = msbt_extras.ato1 {
        builder = builder.ato1(msbt::section::Ato1::new_unlinked(unknown_bytes));
    }
    if let Some(unknown_bytes) = msbt_extras.atr1 {
    builder = builder.atr1(msbt::section::Atr1::new_unlinked(unknown_bytes));
    }
    if let Some(unknown_bytes) = msbt_extras.tsy1 {
        builder = builder.tsy1(msbt::section::Tsy1::new_unlinked(unknown_bytes));
    }
    if let Some(nli1) = msbt_extras.nli1 {
        builder = builder.nli1(msbt::section::Nli1::new_unlinked(nli1.id_count, nli1.global_ids));
    }
    for message in &pot.messages {
        let context = message.context.clone().unwrap_or_default();
        if context != EXTRAS_ID && context != VERSION_ID {
            builder = builder.add_label(context, parse_fn(&message));
        }
    }

    builder.build()
}
