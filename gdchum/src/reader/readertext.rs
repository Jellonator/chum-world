use crate::bytedata::ByteData;
use crate::chumfile::ChumFile;
use gdnative::*;

pub enum TextType {
    FullText(GodotString),
    ReadOnlyText(GodotString),
    ErrText,
}

pub fn read_text(data: &ByteData) -> TextType {
    match data.get_data().len() {
        x if x < 4 => TextType::ErrText,
        _x => {
            let mut valid = true;
            let s = data.get_data()[4..]
                .iter()
                .map(|x| {
                    if *x >= 32 && *x <= 126 {
                        (*x) as char
                    } else {
                        valid = false;
                        '�'
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
    data.get_bytedata()
        .script()
        .map(|x| {
            let mut dict = Dictionary::new();
            match read_text(x) {
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
        })
        .unwrap()
}
