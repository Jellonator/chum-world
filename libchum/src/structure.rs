use crate::common;
use crate::format::TotemFormat;
use crate::util::error;
use std::io::{self, Read, Write};

pub trait ChumStruct: Sized {
    fn structure(&self) -> ChumStructVariant;
    fn destructure(data: &ChumStructVariant) -> Result<Self, Box<dyn std::error::Error>>;
}

pub trait ChumEnum: Sized {
    fn from_u32(value: u32) -> Option<Self>;
    fn to_u32(&self) -> u32;
    fn get_names(&self) -> Vec<String>;
}

pub trait ChumBinary: Sized {
    fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> error::StructUnpackResult<Self>;
    fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    Custom(i64, i64),
    Enum(Vec<String>),
    Flags(Vec<String>),
}

impl IntType {
    pub fn get_range(&self) -> (i64, i64) {
        match self {
            IntType::I8 => (i8::MIN as i64, i8::MAX as i64),
            IntType::U8 => (u8::MIN as i64, u8::MAX as i64),
            IntType::I16 => (i16::MIN as i64, i16::MAX as i64),
            IntType::U16 => (u16::MIN as i64, u16::MAX as i64),
            IntType::I32 => (i32::MIN as i64, i32::MAX as i64),
            IntType::U32 => (u32::MIN as i64, u32::MAX as i64),
            IntType::Custom(a, b) => (*a, *b),
            IntType::Enum(ref v) => (0, v.len() as i64),
            IntType::Flags(ref v) => (0, 2i64.pow(v.len() as u32) - 1),
        }
    }
}

#[derive(Clone)]
pub struct ArrayData {
    pub data: Vec<ChumStructVariant>,
    pub default_value: Box<ChumStructVariant>,
    pub can_resize: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct ColorInfo {
    pub has_alpha: bool,
}

#[derive(Clone)]
pub enum ChumStructVariant {
    Integer(i64, IntType),
    Float(f32),
    Transform3D(common::Mat4x4),
    Transform2D(common::Mat3x3),
    Vec2(common::Vector2),
    Vec3(common::Vector3),
    Color(common::Color, ColorInfo),
    Reference(i32, Option<String>),
    Array(ArrayData),
    Struct(Vec<(String, ChumStructVariant)>),
    Optional {
        value: Option<Box<ChumStructVariant>>,
        default_value: Box<ChumStructVariant>
    }
}

#[derive(Debug, Clone)]
pub enum ChumPathElement<'a> {
    Index(usize),
    Member(&'a str),
}

pub type ChumPath<'a> = [ChumPathElement<'a>];

impl ChumStructVariant {
    pub fn get<'a>(&self, path: &ChumPath<'a>) -> Option<&ChumStructVariant> {
        use ChumPathElement::*;
        use ChumStructVariant::*;
        match (self, path.first()) {
            (Array(ref data), Some(Index(i))) => data.data.get(*i),
            (Struct(ref data), Some(Member(ref name))) => {
                for (id, value) in data {
                    if id == name {
                        return Some(value);
                    }
                }
                None
            }
            (_, _) => None,
        }
    }

    pub fn get_vec2(&self) -> Option<&common::Vector2> {
        use ChumStructVariant::*;
        match *self {
            Vec2(ref x) => Some(x),
            _ => None,
        }
    }

    pub fn get_vec3(&self) -> Option<&common::Vector3> {
        use ChumStructVariant::*;
        match *self {
            Vec3(ref x) => Some(x),
            _ => None,
        }
    }

    pub fn get_transform3d(&self) -> Option<&common::Mat4x4> {
        use ChumStructVariant::*;
        match *self {
            Transform3D(ref x) => Some(x),
            _ => None,
        }
    }
    pub fn get_transform2d(&self) -> Option<&common::Mat3x3> {
        use ChumStructVariant::*;
        match *self {
            Transform2D(ref x) => Some(x),
            _ => None,
        }
    }

    pub fn get_i64(&self) -> Option<i64> {
        use ChumStructVariant::*;
        match *self {
            Integer(x, _) => Some(x as i64),
            _ => None,
        }
    }
    pub fn get_i64_range(&self) -> Option<(i64, i64)> {
        use ChumStructVariant::*;
        match *self {
            Integer(_, ref x) => match x {
                IntType::I8 => Some((i8::MIN as i64, i8::MAX as i64)),
                IntType::U8 => Some((u8::MIN as i64, u8::MAX as i64)),
                IntType::I16 => Some((i16::MIN as i64, i16::MAX as i64)),
                IntType::U16 => Some((u16::MIN as i64, u16::MAX as i64)),
                IntType::I32 => Some((i32::MIN as i64, i32::MAX as i64)),
                IntType::U32 => Some((u32::MIN as i64, u32::MAX as i64)),
                IntType::Custom(a, b) => Some((*a, *b)),
                IntType::Enum(ref v) => Some((0, v.len() as i64)),
                IntType::Flags(ref v) => Some((0, 2i64.pow(v.len() as u32) - 1)),
            },
            _ => None,
        }
    }
    pub fn get_f32(&self) -> Option<f32> {
        use ChumStructVariant::*;
        match *self {
            Float(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_color(&self) -> Option<&common::Color> {
        use ChumStructVariant::*;
        match *self {
            Color(ref x, ref _info) => Some(x),
            _ => None,
        }
    }
    pub fn get_color_info(&self) -> Option<&ColorInfo> {
        use ChumStructVariant::*;
        match *self {
            Color(ref _x, ref info) => Some(info),
            _ => None,
        }
    }
    pub fn get_reference_id(&self) -> Option<i32> {
        use ChumStructVariant::*;
        match *self {
            Reference(x, _) => Some(x),
            _ => None,
        }
    }
    pub fn get_reference_type(&self) -> Option<&str> {
        use ChumStructVariant::*;
        match *self {
            Reference(_, ref y) => y.as_ref().map(|s| s.as_str()),
            _ => None,
        }
    }
    pub fn get_array(&self) -> Option<&[ChumStructVariant]> {
        use ChumStructVariant::*;
        match self {
            Array(ref x) => Some(&x.data),
            _ => None,
        }
    }
    pub fn get_array_item(&self, index: usize) -> Option<&ChumStructVariant> {
        use ChumStructVariant::*;
        match self {
            Array(ref x) => x.data.get(index),
            _ => None,
        }
    }
    pub fn get_array_default(&self) -> Option<&ChumStructVariant> {
        use ChumStructVariant::*;
        match self {
            Array(ref x) => Some(&x.default_value),
            _ => None,
        }
    }
    pub fn can_resize_array(&self) -> Option<bool> {
        use ChumStructVariant::*;
        match self {
            Array(ref x) => Some(x.can_resize),
            _ => None,
        }
    }
    pub fn get_struct(&self) -> Option<&[(String, ChumStructVariant)]> {
        use ChumStructVariant::*;
        match self {
            Struct(ref x) => Some(x),
            _ => None,
        }
    }
    pub fn get_struct_item(&self, name: &str) -> Option<&ChumStructVariant> {
        use ChumStructVariant::*;
        match self {
            Struct(ref data) => {
                for (id, value) in data {
                    if id == name {
                        return Some(value);
                    }
                }
                None
            }
            _ => None,
        }
    }
    pub fn get_optional_value(&self) -> Option<Option<&ChumStructVariant>> {
        use ChumStructVariant::*;
        use std::borrow::Borrow;
        match self {
            Optional {
                ref value,
                default_value: _,
            } => {
                Some(
                    value.as_ref().map(|x|x.borrow())
                )
            }
            _ => None,
        }
    }
    pub fn get_optional_default_value(&self) -> Option<&ChumStructVariant> {
        use ChumStructVariant::*;
        use std::borrow::Borrow;
        match self {
            Optional {
                value: _,
                ref default_value,
            } => {
                Some(default_value.borrow())
            }
            _ => None,
        }
    }
}
