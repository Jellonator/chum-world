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
pub struct ArrayData {
    pub data: Vec<ChumStructVariant>,
    pub default_value: Box<ChumStructVariant>,
    pub can_resize: bool,
}

#[derive(Clone)]
pub enum ChumStructVariant {
    Integer(i64, IntType),
    Float(f32),
    Transform3D(common::Mat4x4),
    Transform2D(common::Mat3x3),
    Vec2(common::Vector2),
    Vec3(common::Vector3),
    Color(common::Color),
    Reference(i32, Option<String>),
    Array(ArrayData),
    Struct(Vec<(String, ChumStructVariant)>),
}

#[derive(Debug, Clone)]
pub enum ChumPathElement<'a> {
    Index(usize),
    Member(&'a str),
}

pub type ChumPath<'a> = [ChumPathElement<'a>];

#[allow(unused_macros)]
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

macro_rules! chum_struct_get_type {
    (u8) => {u8};
    (i8) => {i8};
    (u16) => {u16};
    (i16) => {i16};
    (u32) => {u32};
    (i32) => {i32};
    (enum {$($_name:ident),*}) => {i64};
    (flags {$($_name:ident),*}) => {i64};
    (custom $_min:expr, $_max:expr) => {i64};
    (f32) => {f32};
    (Mat4x4) => {common::Mat4x4};
    (Mat3x3) => {common::Mat3x3};
    (Vector2) => {common::Vector2};
    (Vector3) => {common::Vector3};
    (Color) => {common::Color};
    (reference) => {i32};
    (reference $typename:ident) => {i32};
    (fixed array [$($inner:tt)*] $len:literal) => {[chum_struct_get_type!($($inner)*);$len]};
    (dynamic array [$($inner:tt)*] $_default:expr) => {Vec<chum_struct_get_type!($($inner)*)>};
    (struct $t:ty) => {$t}
}

macro_rules! chum_struct_structure {
    ([u8],$value:expr) => {Integer($value as i64, U8)};
    ([i8],$value:expr) => {Integer($value as i64, I8)};
    ([u16],$value:expr) => {Integer($value as i64, U16)};
    ([i16],$value:expr) => {Integer($value as i64, I16)};
    ([u32],$value:expr) => {Integer($value as i64, U32)};
    ([i32],$value:expr) => {Integer($value as i64, I32)};
    ([enum {$($name:ident),*}],$value:expr) => {
        Integer($value as i64, Enum(
            vec![
                $(
                    stringify!($name).to_owned(),
                )*
                ]
            ))
        };
        ([flags {$($name:ident),*}],$value:expr) => {
        Integer($value as i64, Enum(
            vec![
                $(
                    stringify!($name).to_owned(),
                )*
                ]
        ))
    };
    ([custom $min:expr, $max:expr],$value:expr) => {
        Integer($value as i64, Custom($min,$max))
    };
    ([f32],$value:expr) => {Float($value)};
    ([Mat4x4],$value:expr) => {Transform3D($value)};
    ([Mat3x3],$value:expr) => {Transform2D($value)};
    ([Vector2],$value:expr) => {Vec2($value)};
    ([Vector3],$value:expr) => {Vec3($value)};
    ([Color],$value:expr) => {Color($value)};
    ([reference],$value:expr) => {Reference($value,None)};
    ([reference $typename:ident],$value:expr) => {
        Reference($value,Some(stringify!($typename).to_owned()))
    };
    ([fixed array [$($inner:tt)*] $len:literal],$value:expr) => {
        Array(ArrayData{
            can_resize: false,
            data: $value
                .iter()
                .map(|x| chum_struct_structure!([$($inner)*], *x))
                .collect(),
            // some random default value that won't be used anyways
            default_value: Box::new(Integer(0,U8)),//<chum_struct_get_type!($($inner:tt)*)>::default()
        })
    };
    ([dynamic array [$($inner:tt)*] $default:expr],$value:expr) => {
        Array(ArrayData{
            can_resize: false,
            data: $value
                .iter()
                .map(|x| chum_struct_structure!([$($inner)*], *x))
                .collect(),
            default_value: Box::new(chum_struct_structure!([$($inner)*], $default)),
        })
    };
    ([struct $t:ty],$value:expr) => {
        $value.structure()
    }
}

#[macro_export]
macro_rules! chum_struct {
    (pub struct $structname:ident { 
        $(
            pub $name:ident : [$($inner:tt)*]
        ),* $(,)? // this is just so that the last comma is optional
    } ) => {
        pub struct $structname {
            $(
                pub $name : chum_struct_get_type!($($inner)*),
            )*
        }
        impl ChumStruct for $structname {
            fn structure(&self) -> ChumStructVariant {
                #![allow(unused_imports)]
                use crate::structure::ChumStructVariant::*;
                use crate::structure::IntType::*;
                use crate::structure::ArrayData;
                Struct(vec![
                    $(
                        (
                            stringify!($name).to_owned(),
                            chum_struct_structure!([$($inner)*],self.$name)
                        ),
                    )*
                ])
            }
            fn destructure(_data: ChumStructVariant) -> Result<Self, Box<dyn std::error::Error>> {
                unimplemented!()
            }
        }
    };
}

// chum_struct! {
//     pub struct Foobar {

//     }
// }

// chum_struct! {
//     pub struct Example {
//         pub v_u8: [u8],
//         pub v_i8: [i8],
//         pub v_custom: [custom 0, 100],
//         pub b: [enum {Foo, Bar}],
//         pub c: [flags {A, B, C}],
//         pub v_reference1: [reference],
//         pub v_reference2: [reference MATERIAL],
//         pub v_struct: [struct Foobar],
//         pub v_array_struct: [dynamic array [struct Foobar] Foobar{}],
//         pub v_array_u8: [fixed array [u8]],
//         pub v_array_custom: [fixed array [custom 0, 100]],
//     }
// }

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
            Color(ref x) => Some(x),
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
}
