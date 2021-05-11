extern crate potty_msbt;

use potty_msbt::game::botw;
use std::str::FromStr;
use potty::{PotMessage, PotComment};
use std::io::{BufReader, Result};
use std::fs::File;
use msbt::{Msbt};

fn main() -> Result<()> {
    for arg in std::env::args().skip(1) {
        let file_name = &arg.strip_suffix(".msbt").unwrap();
        let file_msbt = File::open(format!("{}.msbt", file_name))?;
        let mut reader = BufReader::new(file_msbt);
        let pot = potty_msbt::po_from_msbt(&mut reader, botw::tokens_to_string);
        let mut file_po = File::create(format!("{}.po", file_name))?;
        pot.write(&mut file_po)?;
    }

    Ok(())
}
