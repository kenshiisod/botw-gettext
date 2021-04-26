pub mod botw;
pub mod cardboard;

pub const MARKER_START: u16 = 0x0E;
pub const MARKER_END: u16 = 0x0F;

use crate::msbtw::{Tag, Node, Param, ParamKind};
use crate::game::botw as render;
use std::io::{Read, BufRead, Seek};
use regex::Regex;

pub fn bytes_to_nodes<R: std::io::Read + std::io::Seek>(reader: &mut R) -> Vec<Node> {
    let mut rdr = byteordered::ByteOrdered::le(reader);
    let mut nodes: Vec<Node> = Vec::new();
    let mut prev_tag: Option<Tag> = None;
    let mut prev_bytes = Vec::new();

    while let Ok(byte) = rdr.read_u16() {
        match byte {
            MARKER_START => {
                if let Some(lt) = prev_tag {
                    nodes.push(Node::Tag(lt));
                }

                nodes.push(Node::Text(String::from_utf16(&prev_bytes).unwrap()));
                prev_bytes = Vec::new();

                let mut bytes = [0; 4];
                rdr.read_exact(&mut bytes).unwrap();
                let mut tag = render::new_tag_by_bytes(bytes);

                let expected_params_size = rdr.read_u16().unwrap() as usize;
                tag.apply_bytes(&mut rdr);
                let params_size: usize = tag.params.iter().map(|p| p.to_bytes().len()).sum();

                if params_size > expected_params_size {
                    panic!("Params exceed tag boundary: tag={}, exp: {}, act: {}", tag.name, expected_params_size, params_size);
                }

                let remain = expected_params_size - params_size;
                if remain > 0 {
                    let mut param = Param::new("bytes", ParamKind::Bytes(remain as u16));
                    param.apply_bytes(&mut rdr);
                    tag.params.push(param);
                }

                prev_tag = Some(tag);
            },
            MARKER_END => {
                if let Some(ref mut lt) = prev_tag {
                    lt.contents.push(Node::Text(String::from_utf16_lossy(&prev_bytes)));
                    prev_bytes = Vec::new();
                }
            },
            0x00 => {
                rdr.seek(std::io::SeekFrom::Current(-2)).unwrap();
                // Some games e.g. mario & luigi have bytes after null
                // Just strip them for now
                let mut padding_end = Vec::new();
                rdr.read_to_end(&mut padding_end).unwrap();
                // prev_bytes.extend(hex::encode_upper(&padding_end).encode_utf16());
            },
            _ => prev_bytes.push(byte)
        };
    };

    if let Some(lt) = prev_tag {
        nodes.push(Node::Tag(lt));
    }

    let text = String::from_utf16_lossy(&prev_bytes).to_string();

    nodes.push(Node::Text(text));
    nodes
}

pub fn bbcode_to_nodes<R: std::io::BufRead + std::io::Read + std::io::Seek>(reader: &mut R) -> Vec<Node> {
    let mut rdr = byteordered::ByteOrdered::le(reader);
    let mut nodes: Vec<Node> = Vec::new();
    let mut prev_bytes = Vec::new();

    let params_re = Regex::new(r#"([^=\s]+)="([^"\\]*(?:\\.[^"\\]*)*)"#).unwrap();

    while let Ok(byte) = rdr.read_u8() {
        if byte != b'[' {
            prev_bytes.push(byte);
            continue;
        }

        let tag_start = rdr.stream_position().unwrap();
        let is_closing = rdr.read_u8().unwrap() == b'/';
        if !is_closing {
            rdr.seek(std::io::SeekFrom::Current(-1)).unwrap();
        }

        if !prev_bytes.is_empty() {
            nodes.push(Node::Text(String::from_utf8_lossy(&prev_bytes).to_string()));
            prev_bytes = Vec::new();
        }

        let mut opening_u8 = Vec::new();
        rdr.read_until(b']', &mut opening_u8).unwrap();

        let cnts = String::from_utf8_lossy(&opening_u8[..opening_u8.len()-1]);
        let mut parts = cnts.splitn(2, " ");
        let tag_name = parts.next().unwrap();

        if is_closing {
            let pos = nodes.iter().rev().position(|n| {
                match n {
                    Node::Tag(t) => t.name == tag_name,
                    _ => false
                }
            });
            if let Some(p) = pos {
                let pos = nodes.len() - p;
                let old: Vec<_> = nodes.splice(pos.., vec![]).collect();
                let node = &mut nodes[pos-1];
                if let Node::Tag(ref mut t) = node {
                    t.contents.extend(old);
                }
            }
        } else {
            let mut tag = match render::new_tag_by_name(&tag_name) {
                Some(t) => t,
                _ => {
                    prev_bytes.push(b'[');
                    rdr.seek(std::io::SeekFrom::Start(tag_start)).unwrap();
                    continue
                }
            };

            let params_str = parts.next().unwrap();
            for cap in params_re.captures_iter(params_str) {
                let pp = tag.params.iter_mut().find(|p| p.name == &cap[1]);
                if let Some(p) = pp {
                    p.value = cap[2].replace("\\\"", "\"")
                        .replace("\\r", "\r").replace("\\t", "\t").to_string();
                }
            }
            nodes.push(Node::Tag(tag));
        }
    }

    let text = String::from_utf8_lossy(&prev_bytes).to_string();
    nodes.push(Node::Text(text));
    nodes
}

pub fn to_string<R>(reader: &mut R) -> String
where R: std::io::Seek + std::io::BufRead + std::io::Read {
    bytes_to_nodes(reader).iter().map(|n| n.to_string())
        .collect::<String>()
}

pub fn to_bytes<R>(reader: &mut R) -> Vec<u8>
where R: std::io::Seek + std::io::BufRead + std::io::Read {
    let mut bytes: Vec<u8> = bbcode_to_nodes(reader)
        .iter().flat_map(|n| n.to_bytes().into_iter()).collect();
    bytes.extend(&0_u16.to_le_bytes());
    bytes
}
