use crate::common;
use quick_xml::{self, events::*};
use std::error::Error;
use std::fmt;
use std::io::Write;

#[derive(Clone, Debug)]
pub enum SerializeError {
    SpaceInString(String),
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SerializeError::*;
        match self {
            SpaceInString(s) => write!(f, "Content data contains space: {}", s),
        }
    }
}

impl Error for SerializeError {}

pub trait XMLContent {
    fn serialize_content(&self) -> Result<String, SerializeError>;
}

pub trait XMLAttribute {
    fn serialize_attribute(&self) -> Option<String>;
}

pub struct TagWriter<'a> {
    writer: quick_xml::Writer<&'a mut dyn Write>,
}

impl<'a> TagWriter<'a> {
    pub fn add_tag(&mut self, tag: &dyn XMLTag) -> Result<(), Box<dyn Error>> {
        let attributes: Vec<_> = tag
            .get_attributes()
            .iter()
            .filter_map(|x| x.1.serialize_attribute().map(|y| (x.0, y)))
            .collect();
        let data_start = BytesStart::owned_name(tag.get_name())
            .with_attributes(attributes.iter().map(|x| (x.0, x.1.as_str())));
        let content = tag.get_contents();
        let children = tag.get_child_tags();
        if children.len() > 0 || content.is_some() {
            // start tag
            self.writer.write_event(Event::Start(data_start))?;
            // content
            if let Some(inner) = content {
                self.writer
                    .write_event(Event::Text(BytesText::from_plain_str(
                        inner.serialize_content()?.as_str(),
                    )))?;
            }
            // write children
            for child in children {
                self.add_tag(child)?;
            }
            // end tag
            let data_end = BytesEnd::owned(tag.get_name().bytes().collect());
            self.writer.write_event(Event::End(data_end))?;
        } else {
            // empty tag
            self.writer.write_event(Event::Empty(data_start))?;
        }
        Ok(())
    }
}

pub trait XMLTag {
    fn get_name(&self) -> &str;
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)>;
    fn get_contents(&self) -> Option<&dyn XMLContent>;
    fn get_child_tags(&self) -> Vec<&dyn XMLTag>;
}

macro_rules! impl_xml {
    ($x:ty) => {
        impl XMLContent for $x {
            fn serialize_content(&self) -> Result<String, SerializeError> {
                Ok(self.to_string())
            }
        }
        impl XMLAttribute for $x {
            fn serialize_attribute(&self) -> Option<String> {
                Some(self.to_string())
            }
        }
    };
}

impl_xml!(u8);
impl_xml!(i8);
impl_xml!(u16);
impl_xml!(i16);
impl_xml!(u32);
impl_xml!(i32);
impl_xml!(u64);
impl_xml!(i64);
impl_xml!(f32);
impl_xml!(f64);
impl_xml!(usize);
impl_xml!(isize);
impl_xml!(bool);

impl XMLContent for &str {
    fn serialize_content(&self) -> Result<String, SerializeError> {
        for c in self.chars() {
            if c.is_whitespace() {
                return Err(SerializeError::SpaceInString(self.to_string()));
            }
        }
        Ok(self.to_string())
    }
}

impl XMLAttribute for &str {
    fn serialize_attribute(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl XMLContent for String {
    fn serialize_content(&self) -> Result<String, SerializeError> {
        Ok(self.to_string())
    }
}

impl XMLAttribute for String {
    fn serialize_attribute(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<T> XMLContent for Vec<T>
where
    T: XMLContent,
{
    fn serialize_content(&self) -> Result<String, SerializeError> {
        match self.len() {
            0 => Ok(String::new()),
            1 => self[0].serialize_content(),
            _ => {
                let mut ret = self[0].serialize_content()?;
                for x in &self[1..] {
                    ret += &format!(" {}", x.serialize_content()?);
                }
                Ok(ret)
            }
        }
    }
}

impl<T> XMLAttribute for Vec<T>
where
    T: XMLAttribute,
{
    fn serialize_attribute(&self) -> Option<String> {
        match self.len() {
            0 => None,
            1 => self[0].serialize_attribute(),
            _ => {
                let mut ret = self[0].serialize_attribute().unwrap_or(String::new());
                for x in &self[1..] {
                    ret += &format!(" {}", x.serialize_attribute().unwrap_or(String::new()));
                }
                Some(ret)
            }
        }
    }
}

impl<T> XMLAttribute for Option<T>
where
    T: XMLAttribute,
{
    fn serialize_attribute(&self) -> Option<String> {
        self.as_ref().and_then(|x| x.serialize_attribute())
    }
}

impl XMLContent for common::Mat4x4 {
    fn serialize_content(&self) -> Result<String, SerializeError> {
        let mut s = self.mat[0].to_string();
        for value in &self.mat[1..] {
            s += &format!(" {}", value);
        }
        Ok(s)
    }
}

pub fn write_to(
    data: &dyn XMLTag,
    writer: &mut dyn Write,
    pretty: bool,
) -> Result<(), Box<dyn Error>> {
    // let xml = quick_xml::Writer::new(writer);
    let xml = if pretty {
        quick_xml::Writer::new_with_indent(writer, 0x20, 2)
    } else {
        quick_xml::Writer::new(writer)
    };
    let mut tagwriter = TagWriter { writer: xml };
    tagwriter.add_tag(data)
}

pub fn write_to_str(data: &dyn XMLTag, pretty: bool) -> Result<String, Box<dyn Error>> {
    let mut output = Vec::new();
    write_to(data, &mut output as &mut dyn Write, pretty)?;
    Ok(String::from_utf8(output)?)
}
