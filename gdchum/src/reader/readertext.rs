use crate::chumfile::ChumFile;
use gdnative::*;
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

pub fn read_text_from_res(data: &ChumFile) -> Dictionary {
    let mut dict = Dictionary::new();
    match read_text(&data.get_data_as_vec()) {
        TextType::ErrText => {
            dict.set(&"exists".into(), &false.into());
            dict.set(&"readonly".into(), &true.into());
            dict.set(&"text".into(), &"".into());
        }
        TextType::ReadOnlyText(s) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"readonly".into(), &true.into());
            dict.set(&"text".into(), &Variant::from(&s));
        }
        TextType::FullText(s) => {
            dict.set(&"exists".into(), &true.into());
            dict.set(&"readonly".into(), &false.into());
            dict.set(&"text".into(), &Variant::from(&s));
        }
    }
    dict
}
