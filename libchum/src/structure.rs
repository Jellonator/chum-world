use crate::common;

pub trait ChumStruct: Sized {
    fn structure(&self) -> ChumStructVariant;
    fn destructure(data: ChumStructVariant) -> Result<Self, Box<dyn std::error::Error>>;
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
pub enum ChumStructVariant {
    Integer(i64, IntType),
    Float(f32),
    Transform(common::Mat4x4),
    Vec2(common::Vector2),
    Vec3(common::Vector3),
    Array(Vec<ChumStructVariant>, Box<ChumStructVariant>),
    Struct(Vec<(String, ChumStructVariant)>),
}

#[derive(Debug, Clone)]
pub enum ChumPathElement<'a> {
    Index(usize),
    Member(&'a str),
}

pub type ChumPath<'a> = [ChumPathElement<'a>];

macro_rules! chum_path_element {
    ( [$x:expr] ) => {
        ChumPathElement::Index($x)
    };
    ( $x:expr ) => {
        ChumPathElement::Member(&stringify!($x))
    };
}

#[macro_export]
macro_rules! chum_path {
    ( $( $x:tt). * ) => {
        &[
            $(
                chum_path_element!( $x ),
            )*
        ]
    };
}

impl ChumStructVariant {
    pub fn get<'a>(&self, path: &ChumPath<'a>) -> Option<&ChumStructVariant> {
        use ChumPathElement::*;
        use ChumStructVariant::*;
        match (self, path.first()) {
            (Array(ref v, _), Some(Index(i))) => v.get(*i),
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

    pub fn get_transform(&self) -> Option<&common::Mat4x4> {
        use ChumStructVariant::*;
        match *self {
            Transform(ref x) => Some(x),
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
    pub fn get_array(&self) -> Option<&[ChumStructVariant]> {
        use ChumStructVariant::*;
        match self {
            Array(ref x, _) => Some(x),
            _ => None,
        }
    }
    pub fn get_array_item(&self, index: usize) -> Option<&ChumStructVariant> {
        use ChumStructVariant::*;
        match self {
            Array(ref x, _) => x.get(index),
            _ => None,
        }
    }
    pub fn get_array_default(&self) -> Option<&ChumStructVariant> {
        use ChumStructVariant::*;
        match self {
            Array(_, ref x) => Some(x),
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
}
