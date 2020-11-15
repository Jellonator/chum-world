#[macro_export]
macro_rules! impl_tag_content {
    ($x:ty, $y:expr, $data: ident) => {
        impl XMLTag for $x {
            fn get_name(&self) -> &str {
                $y
            }
            fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
                vec![]
            }
            fn get_contents(&self) -> Option<&dyn XMLContent> {
                Some(&self.$data)
            }
            fn get_child_tags(&self) -> Vec<&dyn XMLTag> {
                vec![]
            }
        }
    };
}

#[macro_export]
macro_rules! impl_tag_tree {
    (
        $x:ty,
        $y:expr,
        attr => [ $( ($attrstr:expr, $attrdata:ident) ), * ],
        tags => [ $( $tag:ident ), *]
    ) => {
        impl XMLTag for $x {
            fn get_name(&self) -> &str {
                $y
            }
            fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
                // some weirdness here to prevent compiler warnings
                let v = vec![];
                $(
                    let mut v = v;
                    v.push(($attrstr, &self.$attrdata as &dyn XMLAttribute));
                )*
                v
            }
            fn get_contents(&self) -> Option<&dyn XMLContent> {
                None
            }
            fn get_child_tags<'a>(&self) -> Vec<&dyn XMLTag> {
                // some weirdness here to prevent compiler warnings
                let v = vec![];
                $(
                    let mut v = v;
                    {
                        use crate::util::xml::XMLVectorifyTag;
                        v.extend(self.$tag.vectorify());
                    }
                )*
                v
            }
        }
    };
}

#[macro_export]
macro_rules! impl_tag_enum {
    ($t:ident,
        $(
            $vname:ident => (
                $y:expr,
                attr => [ $( ($attrstr:expr, $attrdata:ident) ), * ],
                tags => [ $( $tag:ident ), *]
                $( , content => $cval:ident )?
            )
        ), *
    ) => {
        impl XMLTag for $t {
            fn get_name(&self) -> &str {
                match self {
                    $(
                        $t::$vname {..}=> $y,
                    )*
                }
            }
            fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
                // some weirdness here to prevent compiler warnings
                match self {
                    $(
                        $t::$vname {
                            $(
                                $attrdata,
                            )*
                            ..
                        } => {
                            let v = vec![];
                            $(
                                let mut v = v;
                                v.push(($attrstr, $attrdata as &dyn XMLAttribute));
                            )*
                            v
                        }
                    )*,
                }
            }
            fn get_contents(&self) -> Option<&dyn XMLContent> {
                match self {
                    $(
                        $t::$vname {
                            $(
                                $cval,
                            )?
                            ..
                        } => {
                            None
                            $(
                                .or(Some($cval as &dyn XMLContent))
                            )?
                        }
                    )*,
                }
                // None $(
                //     .or(&self.$cval as &dyn XMLContent)
                // )?
            }
            fn get_child_tags<'a>(&self) -> Vec<&dyn XMLTag> {
                // some weirdness here to prevent compiler warnings
                match self {
                    $(
                        $t::$vname {
                            $(
                                $tag,
                            )*
                            ..
                        } => {
                            let v = vec![];
                            $(
                                let mut v = v;
                                {
                                    use crate::util::xml::XMLVectorifyTag;
                                    v.extend($tag.vectorify());
                                }
                            )*
                            v
                        }
                    )*,
                }
            }
        }
    };
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

/// Get the actual data type for the given type information.
macro_rules! chum_struct_get_type {
    // special override rules
    (ignore [$($inner:tt)*] $default:expr) => {()};
    // regular rules
    (u8) => {::std::primitive::u8};
    (i8) => {::std::primitive::i8};
    (u16) => {::std::primitive::u16};
    (i16) => {::std::primitive::i16};
    (u32) => {::std::primitive::u32};
    (i32) => {::std::primitive::i32};
    (enum [$repr:tt] $name:ty) => {$name};
    (flags [$repr:tt] {$($_name:ident),*}) => {$repr};
    (custom [$repr:tt] $_min:expr, $_max:expr) => {$repr};
    (f32) => {::std::primitive::f32};
    (Mat4x4) => {$crate::common::Mat4x4};
    (Mat3x3) => {$crate::common::Mat3x3};
    (Vector2) => {$crate::common::Vector2};
    (Vector3) => {$crate::common::Vector3};
    (Vector3 rgb) => {$crate::common::Vector3};
    (Color) => {$crate::common::Color};
    (reference) => {::std::primitive::i32};
    (reference $typename:ident) => {::std::primitive::i32};
    (fixed array [$($inner:tt)*] $len:literal) => {[chum_struct_get_type!($($inner)*);$len]};
    (dynamic array [$lentype:tt] [$($inner:tt)*] $_default:expr) => {Vec<chum_struct_get_type!($($inner)*)>};
    (struct $t:ty) => {$t}
}

/// Determines how to structure each type
macro_rules! chum_struct_structure {
    ([u8],$value:expr) => {Integer($value as ::std::primitive::i64, U8)};
    ([i8],$value:expr) => {Integer($value as ::std::primitive::i64, I8)};
    ([u16],$value:expr) => {Integer($value as ::std::primitive::i64, U16)};
    ([i16],$value:expr) => {Integer($value as ::std::primitive::i64, I16)};
    ([u32],$value:expr) => {Integer($value as ::std::primitive::i64, U32)};
    ([i32],$value:expr) => {Integer($value as ::std::primitive::i64, I32)};
    ([enum [$repr:tt] $name:ty],$value:expr) => {
        Integer($crate::structure::ChumEnum::to_u32(&$value) as i64, Enum(
            $crate::structure::ChumEnum::get_names(&$value)
        ))
    };
    ([flags [$repr:tt] {$($name:ident),*}],$value:expr) => {
        Integer($value as ::std::primitive::i64, Flags(
            vec![
                $(
                    stringify!($name).to_owned(),
                )*
            ]
        ))
    };
    ([custom [$repr:tt] $min:expr, $max:expr],$value:expr) => {
        Integer($value as ::std::primitive::i64, Custom($min,$max))
    };
    ([f32],$value:expr) => {Float($value)};
    ([Mat4x4],$value:expr) => {Transform3D($value)};
    ([Mat3x3],$value:expr) => {Transform2D($value)};
    ([Vector2],$value:expr) => {Vec2($value)};
    ([Vector3],$value:expr) => {Vec3($value)};
    ([Vector3 rgb],$value:expr) => {
        Color(
            $crate::common::Color::new(
                $value.x, $value.y, $value.z, 1.0f32
            ),
            ColorInfo {
                has_alpha: false
            }
        )
    };
    ([Color],$value:expr) => {Color($value, ColorInfo{has_alpha: true})};
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
            default_value: ::std::boxed::Box::new(Integer(0,U8)),
        })
    };
    ([dynamic array [$lentype:tt] [$($inner:tt)*] $default:expr],$value:expr) => {
        Array(ArrayData{
            can_resize: false,
            data: $value
                .iter()
                .map(|x| chum_struct_structure!([$($inner)*], *x))
                .collect(),
            default_value: ::std::boxed::Box::new(chum_struct_structure!([$($inner)*], $default)),
        })
    };
    ([struct $t:ty],$value:expr) => {
        $value.structure()
    }
}

/// Determines how to destructure each type.
macro_rules! chum_struct_destructure {
    ([u8],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::u8};
    ([i8],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::i8};
    ([u16],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::u16};
    ([i16],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::i16};
    ([u32],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::u32};
    ([i32],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::i32};
    ([enum [$repr:tt] $name:ty],$value:expr) => {
        {
            use $crate::structure::ChumEnum;
            <$name>::from_u32($value.get_i64().unwrap() as u32).unwrap()
        }
    };
    ([flags [$repr:tt] {$($name:ident),*}],$value:expr) => {$value.get_i64().unwrap() as $repr};
    ([custom [$repr:tt] $min:expr, $max:expr],$value:expr) => {$value.get_i64().unwrap() as $repr};
    ([f32],$value:expr) => {$value.get_f32().unwrap()};
    ([Mat4x4],$value:expr) => {*$value.get_transform3d().unwrap()};
    ([Mat3x3],$value:expr) => {*$value.get_transform2d().unwrap()};
    ([Vector2],$value:expr) => {*$value.get_vec2().unwrap()};
    ([Vector3],$value:expr) => {*$value.get_vec3().unwrap()};
    ([Vector3 rgb],$value:expr) => {
        {
            let col = $value.get_color().unwrap();
            $crate::common::Vector3::new(
                col.x, col.y, col.z
            )
        }
    };
    ([Color],$value:expr) => {*$value.get_color().unwrap()};
    ([reference],$value:expr) => {$value.get_reference_id().unwrap()};
    ([reference $typename:ident],$value:expr) => {$value.get_reference_id().unwrap()};
    ([fixed array [$($inner:tt)*] $len:literal],$value:expr) => {
        {
            use ::std::mem::{self, MaybeUninit};
            unsafe {
                let mut arr: [MaybeUninit<chum_struct_get_type!($($inner)*)>; $len] = {
                    MaybeUninit::uninit().assume_init()
                };
                for i in 0..$len {
                    arr[i] = MaybeUninit::new(chum_struct_destructure!(
                        [$($inner)*],
                        $value.get_array_item(i).unwrap()));
                }
                mem::transmute::<_, [chum_struct_get_type!($($inner)*); $len]>(arr)
            }
        }
    };
    ([dynamic array [$lentype:tt] [$($inner:tt)*] $default:expr],$value:expr) => {
        $value
            .get_array()
            .unwrap()
            .iter()
            .map(|x| chum_struct_destructure!([$($inner)*], x))
            .collect()
    };
    ([struct $t:ty],$value:expr) => {
        {
            $crate::structure::ChumStruct::destructure($value).unwrap()
        }
    }
}

/// Process a structure function.
/// Results in `None` if the value is ignored.
macro_rules! process_structure {
    ($name:ident,[ignore [$($inner:tt)*] $default:expr],$value:expr) => {None};
    ($name:ident,[$($inner:tt)*],$value:expr) => {
        Some((
            stringify!($name).to_owned(),
            chum_struct_structure!([$($inner)*],$value)
        ))
    };
}

/// Process a destructure function.
/// Results in `()` if the value is ignored.
macro_rules! process_destructure {
    ($name:ident,$data:expr,[ignore [$($inner:tt)*] $default:expr]) => {()};
    ($name:ident,$data:expr,[$($inner:tt)*]) => {
        chum_struct_destructure!(
            [$($inner)*],
            $data.get_struct_item(stringify!($name)).unwrap()
        )
    };
}

/// Automatically generate ChumStruct trait for the given type.
/// This does a lot of stuff under the hood, so the type system is drastically changed to accomidate.
/// In addition, to comply with `chum_struct_generate_readwrite!`, these types also have to have
/// specific sizes for binary serialization. So, these types heavily reflect that.
/// Supported types:
/// * [u8], [i8], [u16], [i16], [u32], [i32]:
/// * [f32]: floats
/// * [enum [repr] EnumName]: enums generated by `chum_enum!`
///     [repr] is the integer representation, e.g. [u16].
///     [repr] does nothing without `chum_struct_generate_readwrite`
/// * [flags [repr] {A, B, C, ...}]: flag integers
///     [repr] is the integer representation, e.g. [u16].
/// * [custom [repr] min max]: custom integer in [min, max] range
///     [repr] is the integer representation, e.g. [u16].
/// * [Mat4x4]: common::Mat4x4
/// * [Mat3x3]: common::Mat3x3
/// * [Vector2]: common::Vector2
/// * [Vector3]: common::Vector3
/// * [Vector3 rbg]: common::Vector3
///     gets converted to and from Color instead.
/// * [Color]: common::Color
/// * [reference <type>]: reference to another file.
///     Actual type is [i32].
///     <type> is an optional reference type, e.g. 'BITMAP'
/// * [fixed array [type] len]: Fixed length array.
///     Gets converted into [type; len].
/// * [dynamic array [lentype] [type] default]: Dynamically sized array.
///     [lentype] is the type of the length. The length must come directly before the data in the binary.
///     [lentype] does nothing without `chum_struct_generate_readwrite`
///     Gets converted into Vec<type>.
///     Adding new elements through the editor will add an instance of `default`.
/// * [struct type]: A structure generated by `chum_struct`.
///     This type will refer to the `type` struct.
#[macro_export]
macro_rules! chum_struct {
    (
        $(
            #[$a:meta]
        )*
        pub struct $structname:ident {
            $(
                pub $name:ident : [$($inner:tt)*]
            ),* $(,)? // this is just so that the last comma is optional
        }
    ) => {
        $(
            #[$a]
        )*
        pub struct $structname {
            $(
                pub $name : chum_struct_get_type!($($inner)*),
            )*
        }
        impl $crate::structure::ChumStruct for $structname {
            fn structure(&self) -> $crate::structure::ChumStructVariant {
                #![allow(unused_imports)]
                use $crate::structure::ChumStructVariant::*;
                use $crate::structure::IntType::*;
                use $crate::structure::ArrayData;
                use $crate::structure::ColorInfo;
                Struct(vec![
                    $(
                        {
                            process_structure!($name,[$($inner)*],self.$name)
                        },
                    )*
                ].into_iter().filter_map(|e|e).collect())
            }
            fn destructure(data: &$crate::structure::ChumStructVariant) -> Result<Self, ::std::boxed::Box<dyn ::std::error::Error>> {
                Ok(
                    Self {
                        $(
                            $name: process_destructure!($name,data,[$($inner)*]),
                        )*
                    }
                )
            }
        }
    };
}

macro_rules! one {
    ($x: ident) => {
        1u32
    };
}

macro_rules! get_index {
    ($($y:ident),*) => {
        {
            let mut x = 0u32;
            $(
                x += one!($y);
            )*
            x
        }
    }
}

/// Generate an enumeration to use with ChumStruct.
/// Essentially a C-style enum that counts up from 0.
#[macro_export]
macro_rules! chum_enum {
    (
        $(
            #[$a:meta]
        )*
        pub enum $enumname:ident {
            $(
                $name:ident
            ),* $(,)? // this is just so that the last comma is optional
        }
    ) => {
        $(
            #[$a]
        )*
        #[repr(u32)]
        pub enum $enumname {
            $(
                $name,
            )*
        }
        impl $crate::structure::ChumEnum for $enumname {
            fn to_u32(&self) -> u32 {
                return *self as u32
            }
            fn from_u32(value: u32) -> Option<Self> {
                if value >= get_index!($($name),*) {
                    None
                } else {
                    unsafe {
                        ::std::option::Option::Some(::std::mem::transmute::<u32,Self>(value))
                    }
                }
            }
            fn get_names(&self) -> Vec<String> {
                vec![
                    $(
                        stringify!($name).to_owned()
                    ),*
                ]
            }
        }
    };
}

// welcome to repretition hell
macro_rules! chum_struct_read {
    ([ignore [$($inner:tt)*] $default:expr],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        chum_struct_read!([$($inner)*],$file,$fmt,$struct,$path).map(|_| ())
    };
    ([u8],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_u8($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([i8],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_i8($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([u16],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_u16($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([i16],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_i16($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([u32],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_u32($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([i32],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_i32($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([enum [$repr:tt] $name:ty],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        {
            use $crate::structure::ChumEnum;
            chum_struct_read!([$repr],$file,$fmt,$struct,$path)
                .map_err(|e| Box::<dyn ::std::error::Error>::from(Box::new(e)))
                .and_then(|x|
                    <$name>::from_u32(x as u32)
                        .ok_or_else(|| Box::new($crate::util::error::EnumerationError {
                            enum_name: stringify!($name).to_owned(),
                            value: x as i64
                        }).into())
                )
                .map_err(|e| $crate::util::error::StructUnpackError {
                    structname: $struct.to_owned(),
                    structpath: $path.to_owned(),
                    error: e
                })
        }
    };
    ([flags [$repr:tt] {$($name:ident),*}],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        chum_struct_read!([$repr],$file,$fmt,$struct,$path)
    };
    ([custom [$repr:tt] $min:expr, $max:expr],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        chum_struct_read!([$repr],$file,$fmt,$struct,$path)
    };
    ([f32],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_f32($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([Mat4x4],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $crate::common::read_mat4($file, $fmt)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([Mat3x3],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $crate::common::read_mat3($file, $fmt)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([Vector2],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $crate::common::read_vec2($file, $fmt)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([Vector3],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $crate::common::read_vec3($file, $fmt)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([Vector3 rgb],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $crate::common::read_vec3($file, $fmt)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([Color],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $crate::common::read_color($file, $fmt)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([reference],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_i32($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([reference $typename:ident],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        $fmt.read_i32($file)
        .map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e)
        })
    };
    ([fixed array [$($inner:tt)*] $len:literal],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        {
            use ::std::mem::{self, MaybeUninit};
            unsafe {
                let mut arr: [MaybeUninit<chum_struct_get_type!($($inner)*)>; $len] = {
                    MaybeUninit::uninit().assume_init()
                };
                for i in 0..$len {
                    arr[i] = MaybeUninit::new(
                        chum_struct_read!([$($inner)*],$file,$fmt,$struct,format!("{}[{}]",$path,i))?
                    );
                }
                Ok(mem::transmute::<_, [chum_struct_get_type!($($inner)*); $len]>(arr))
            }
        }
    };
    ([dynamic array [$lentype:tt] [$($inner:tt)*] $default:expr],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        {
            chum_struct_read!([$lentype],$file,$fmt,$struct,$path)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e)
            }).and_then(|size| {
                let mut vec = Vec::with_capacity(size as usize);
                for i in 0..size {
                    vec.push(chum_struct_read!([$($inner)*],$file,$fmt,$struct,format!("{}[{}]",$path,i))?)
                }
                Ok(vec)
            })
        }
    };
    ([struct $t:ty],$file:expr,$fmt:expr,$struct:expr,$path:expr) => {
        match <$t>::read_from($file,$fmt) {
            Ok(value) => Ok(value),
            Err($crate::util::error::StructUnpackError {
                structname: _,
                structpath,
                error
            }) => Err($crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: format!("{}.{}", $path, structpath),
                error
            })
        }
    }
}

macro_rules! chum_struct_write {
    ([ignore [$($inner:tt)*] $default:expr],$file:expr,$fmt:expr,$value:expr) => {
        chum_struct_write!([$($inner)*],$file,$fmt,&$default)
    };
    ([u8],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_u8($file, *$value)
    };
    ([i8],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_i8($file, *$value)
    };
    ([u16],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_u16($file, *$value)
    };
    ([i16],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_i16($file, *$value)
    };
    ([u32],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_u32($file, *$value)
    };
    ([i32],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_i32($file, *$value)
    };
    ([enum [$repr:tt] $name:ty],$file:expr,$fmt:expr,$value:expr) => {
        {
            let ivalue = $crate::structure::ChumEnum::to_u32($value) as $repr;
            chum_struct_write!([$repr],$file,$fmt,&ivalue)
        }
    };
    ([flags [$repr:tt] {$($name:ident),*}],$file:expr,$fmt:expr,$value:expr) => {
        chum_struct_write!([$repr],$file,$fmt,$value)
    };
    ([custom [$repr:tt] $min:expr, $max:expr],$file:expr,$fmt:expr,$value:expr) => {
        chum_struct_write!([$repr],$file,$fmt,$value)
    };
    ([f32],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_f32($file, *$value)
    };
    ([Mat4x4],$file:expr,$fmt:expr,$value:expr) => {
        $crate::common::write_mat4($value, $file, $fmt)
    };
    ([Mat3x3],$file:expr,$fmt:expr,$value:expr) => {
        $crate::common::write_mat3($value, $file, $fmt)
    };
    ([Vector2],$file:expr,$fmt:expr,$value:expr) => {
        $crate::common::write_vec2($value, $file, $fmt)
    };
    ([Vector3],$file:expr,$fmt:expr,$value:expr) => {
        $crate::common::write_vec3($value, $file, $fmt)
    };
    ([Vector3 rgb],$file:expr,$fmt:expr,$value:expr) => {
        $crate::common::write_vec3($value, $file, $fmt)
    };
    ([Color],$file:expr,$fmt:expr,$value:expr) => {
        $crate::common::write_color($value, $file, $fmt)
    };
    ([reference],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_i32($file, *$value)
    };
    ([reference $typename:ident],$file:expr,$fmt:expr,$value:expr) => {
        $fmt.write_i32($file, *$value)
    };
    ([fixed array [$($inner:tt)*] $len:literal],$file:expr,$fmt:expr,$value:expr) => {
        {
            for i in 0..$len {
                chum_struct_write!([$($inner)*],$file,$fmt,&$value[i])?;
            }
            // fun cheat
            ::std::io::Result::<()>::Ok(())
        }
    };
    ([dynamic array [$lentype:tt] [$($inner:tt)*] $default:expr],$file:expr,$fmt:expr,$value:expr) => {
        {
            let lenval = $value.len() as $lentype;
            chum_struct_write!([$lentype], $file, $fmt, &lenval)?;
            for value in $value.iter() {
                chum_struct_write!([$($inner)*], $file, $fmt, value)?;
            }
            // fun cheat again
            ::std::io::Result::<()>::Ok(())
        }
    };
    ([struct $t:ty],$file:expr,$fmt:expr,$value:expr) => {
        <$t as $crate::structure::ChumBinary>::write_to($value, $file, $fmt)
    }
}

/// Generate a ChumStruct with the ability to read from/write to
/// binary files using the ChumBinary trait.
#[macro_export]
macro_rules! chum_struct_generate_readwrite {
    (
        $(
            #[$a:meta]
        )*
        pub struct $structname:ident {
            $(
                pub $name:ident : [$($inner:tt)*]
            ),* $(,)? // this is just so that the last comma is optional
        }
    ) => {
        chum_struct! {
            $(
                #[$a]
            )*
            pub struct $structname {
                $(
                    pub $name : [$($inner)*],
                )*
            }
        }
        impl $crate::structure::ChumBinary for $structname {
            fn read_from<R: ::std::io::Read>(file: &mut R, fmt: $crate::format::TotemFormat) -> $crate::util::error::StructUnpackResult<Self> {
                Ok(Self {
                    $(
                        $name: match chum_struct_read!([$($inner)*], file, fmt, stringify!($structname), stringify!($name)) {
                            Ok(value) => value,
                            Err(e) => {
                                return Err(e);
                            }
                        },
                    )*
                })
            }
            fn write_to<W: ::std::io::Write>(&self, writer: &mut W, fmt: $crate::format::TotemFormat) -> ::std::io::Result<()> {
                $(
                    chum_struct_write!([$($inner)*], writer, fmt, &self.$name)?;
                )*
                Ok(())
            }
        }
    }
}

chum_enum! {
    #[derive(Copy, Clone, Debug)]
    pub enum MyEnum {
        Zero,
        One,
        Two
    }
}

chum_struct_generate_readwrite! {
    pub struct Foobar {
        pub v_enum: [enum [u8] MyEnum]
    }
}

chum_struct_generate_readwrite! {
    pub struct Example {
        pub v_u8: [u8],
        pub v_junk: [ignore [u8] 0],
        pub v_i8: [i8],
        pub v_custom: [custom [i32] 0, 100],
        pub b: [enum [u8] MyEnum],
        pub c: [flags [u8] {A, B, C}],
        pub v_reference1: [reference],
        pub v_reference2: [reference MATERIAL],
        pub v_struct: [struct Foobar],
        pub v_array_struct: [dynamic array [u32] [struct Foobar] Foobar{v_enum: MyEnum::Zero}],
        pub v_array_u8: [fixed array [u8] 100],
        pub v_vec_u8: [dynamic array [u32] [u8] 100u8],
        pub v_array_custom: [fixed array [custom [i32] 0, 100] 100],
    }
}
