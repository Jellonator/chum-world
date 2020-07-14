pub trait ChumStruct: Sized {
    fn structure(&self) -> ChumStructVariant;
    fn destructure(data: ChumStructVariant) -> Result<Self, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
pub enum ChumStructVariant {
    I8(i8),
    U8(u8),
    I16(u16),
    U16(u16),
    I32(i32),
    U32(u32),
    F32(f32),
    Array(Vec<Box<ChumStructVariant>>),
    Struct(Vec<(String, Box<ChumStructVariant>)>)
}

#[derive(Debug, Clone)]
pub enum ChumPathElement<'a> {
    Index(usize),
    Member(&'a str)
}

pub type ChumPath<'a> = [ChumPathElement<'a>];

#[macro_export]
macro_rules! chum_path_element {
    ( [$x:expr] ) => {
        ChumPathElement::Index( $x )
    };
    ( $x:expr ) => {
        ChumPathElement::Member( &stringify!( $x ) )
    }
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
        use ChumStructVariant::*;
        use ChumPathElement::*;
        match (self, path.first()) {
            (Array(ref v), Some(Index(i))) => {
                v.get(*i).map(|x| x.as_ref())
            }
            (Struct(ref data), Some(Member(ref name))) => {
                for (id, value) in data {
                    if id == name {
                        return Some(value.as_ref())
                    }
                }
                None
            }
            (_, _) => None
        }
    }

    pub fn get_i64(&self) -> Option<i64> {
        use ChumStructVariant::*;
        match *self {
            I8(x)  => Some(x as i64),
            U8(x)  => Some(x as i64),
            I16(x) => Some(x as i64),
            U16(x) => Some(x as i64),
            I32(x) => Some(x as i64),
            U32(x) => Some(x as i64),
            _ => None
        }
    }
    pub fn get_i64_range(&self) -> Option<(i64, i64)> {
        use ChumStructVariant::*;
        match *self {
            I8(_)  => Some((i8::MIN as i64, i8::MAX as i64)),
            U8(_)  => Some((u8::MIN as i64, u8::MAX as i64)),
            I16(_) => Some((i16::MIN as i64, i16::MAX as i64)),
            U16(_) => Some((u16::MIN as i64, u16::MAX as i64)),
            I32(_) => Some((i32::MIN as i64, i32::MAX as i64)),
            U32(_) => Some((u32::MIN as i64, u32::MAX as i64)),
            _ => None
        }
    }
    pub fn get_f32(&self) -> Option<f32> {
        use ChumStructVariant::*;
        match *self {
            F32(x)  => Some(x),
            _ => None
        }
    }
    pub fn get_array(&self) -> Option<&[Box<ChumStructVariant>]> {
        use ChumStructVariant::*;
        match self {
            Array(ref x)  => Some(x.as_slice()),
            _ => None
        }
    }
    pub fn get_array_mut(&mut self) -> Option<&mut[Box<ChumStructVariant>]> {
        use ChumStructVariant::*;
        match self {
            Array(ref mut x)  => Some(x.as_mut_slice()),
            _ => None
        }
    }
    pub fn get_struct(&self) -> Option<&[(String, Box<ChumStructVariant>)]> {
        use ChumStructVariant::*;
        match self {
            Struct(ref x)  => Some(x.as_slice()),
            _ => None
        }
    }
    pub fn get_struct_mut(&mut self) -> Option<&mut[(String, Box<ChumStructVariant>)]> {
        use ChumStructVariant::*;
        match self {
            Struct(ref mut x)  => Some(x.as_mut_slice()),
            _ => None
        }
    }
}