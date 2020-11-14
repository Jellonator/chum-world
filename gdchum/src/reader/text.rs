use crate::chumfile::ChumFile;
use gdnative::prelude::*;
use std::char;

pub enum TextType {
    FullText(GodotString),
    ReadOnlyText(GodotString),
    ErrText,
}

pub fn read_text(data: &Vec<u8>) -> TextType {
    match data.len() {
        x if x < 4 => TextType::ErrText,
        _x => {
            let mut valid = true;
            let s = data[4..]
                .iter()
                .map(|x| match *x {
                    9 | 10 | 11 | 13 | 32..=126 => (*x) as char,
                    _ => {
                        valid = false;
                        char::REPLACEMENT_CHARACTER
                    }
                })
                .collect::<String>();
            let godots = GodotString::from_str(s);
            if valid {
                TextType::FullText(godots)
            } else {
                TextType::ReadOnlyText(godots)
            }
        }
    }
}

pub fn read_text_from_res(data: &ChumFile) -> Dictionary<Unique> {
    let mut dict = Dictionary::new();
    match read_text(&data.get_data_as_vec()) {
        TextType::ErrText => {
            dict.insert("exists", false);
            dict.insert("readonly", true);
            dict.insert("text", "");
        }
        TextType::ReadOnlyText(s) => {
            dict.insert("exists", true);
            dict.insert("readonly", true);
            dict.insert("text", s);
        }
        TextType::FullText(s) => {
            dict.insert("exists", true);
            dict.insert("readonly", false);
            dict.insert("text", s);
        }
    }
    dict
}
