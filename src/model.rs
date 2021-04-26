use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct MsbtInfo {
    pub group_count: u32,
    pub atr1: Option<Vec<u8>>,
    pub ato1: Option<Vec<u8>>,
    pub tsy1: Option<Vec<u8>>,
    pub nli1: Option<Nli1>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nli1 {
  pub id_count: u32,
  pub global_ids: BTreeMap<u32, u32>,
}
