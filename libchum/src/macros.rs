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
    ([ignore $type:tt $default:expr]) => {()};
    ([
        custom_structure $type:tt
        structure: $structure:expr;
        destructure: $destructure:expr;
    ]) => {chum_struct_get_type!($type)};
    ([
        custom_binary $type:tt
        read: $structure:expr;
        write: $destructure:expr;
    ]) => {chum_struct_get_type!($type)};
    ([option $type:tt $default:expr]) => {Option<chum_struct_get_type!($type)>};
    // regular rules
    ([void]) => {()};
    ([u8]) => {::std::primitive::u8};
    ([i8]) => {::std::primitive::i8};
    ([u16]) => {::std::primitive::u16};
    ([i16]) => {::std::primitive::i16};
    ([u32]) => {::std::primitive::u32};
    ([i32]) => {::std::primitive::i32};
    ([enum [$repr:tt] $name:ty]) => {$name};
    ([flags [$repr:tt] {$($_name:ident),*}]) => {$repr};
    ([int_custom [$repr:tt] $_min:expr, $_max:expr]) => {$repr};
    ([f32]) => {::std::primitive::f32};
    ([Transform3D]) => {$crate::common::Transform3D};
    ([Transform2D]) => {$crate::common::Transform2D};
    ([Vector2]) => {$crate::common::Vector2};
    ([Vector3]) => {$crate::common::Vector3};
    ([Vector3 rgb]) => {$crate::common::Vector3};
    ([Color]) => {$crate::common::ColorRGBA};
    ([Quaternion]) => {$crate::common::Quaternion};
    ([reference]) => {::std::primitive::i32};
    ([reference $typename:ident]) => {::std::primitive::i32};
    ([fixed array $type:tt $len:literal]) => {[chum_struct_get_type!($type);$len]};
    ([dynamic array [$lentype:tt] $type:tt $_default:expr]) => {Vec<chum_struct_get_type!($type)>};
    ([struct $t:ty]) => {$t}
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
    ([int_custom [$repr:tt] $min:expr, $max:expr],$value:expr) => {
        Integer($value as ::std::primitive::i64, Custom($min,$max))
    };
    ([f32],$value:expr) => {Float($value)};
    ([Transform3D],$value:expr) => {Transform3D($value)};
    ([Transform2D],$value:expr) => {Transform2D($value)};
    ([Vector2],$value:expr) => {Vec2($value)};
    ([Vector3],$value:expr) => {Vec3($value)};
    ([Vector3 rgb],$value:expr) => {
        Color(
            $crate::common::ColorRGBA {
                r: $value.x, g: $value.y, b: $value.z, a: 1.0f32
            },
            ColorInfo {
                has_alpha: false
            }
        )
    };
    ([Color],$value:expr) => {Color($value, ColorInfo{has_alpha: true})};
    ([Quaternion],$value:expr) => {
        {
            Vec3($crate::common::quat_to_euler($value))
        }
    };
    ([reference],$value:expr) => {Reference($value,None)};
    ([reference $typename:ident],$value:expr) => {
        Reference($value,Some(stringify!($typename).to_owned()))
    };
    ([fixed array $type:tt $len:literal],$value:expr) => {
        Array(ArrayData{
            can_resize: false,
            data: $value
                .iter()
                .map(|x| chum_struct_structure!($type, *x))
                .collect(),
            // some random default value that won't be used anyways
            default_value: || {Integer(0,U8)},
        })
    };
    ([dynamic array [$lentype:tt] $type:tt $default:expr],$value:expr) => {
        {
            let x: &Vec<chum_struct_get_type!($type)> = &$value;
            Array(ArrayData{
                can_resize: true,
                data: x
                    .iter()
                    .map(|x| chum_struct_structure!($type, *x))
                    .collect(),
                default_value: || {chum_struct_structure!($type, $default)},
            })
        }
    };
    ([struct $t:ty],$value:expr) => {
        $value.structure()
    };
    ([option $type:tt $default:expr],$value:expr) => {
        Optional {
            value: $value
                .as_ref()
                .map(|x: &chum_struct_get_type!($type)| {
                    Box::new(chum_struct_structure!($type,*x))
                }),
            default_value: || {chum_struct_structure!($type, $default)}
        }
    };
}

/// Determines how to destructure each type.
macro_rules! chum_struct_destructure {
    ([u8],$value:expr) => {
        $value.get_i64().unwrap() as ::std::primitive::u8
    };
    ([i8],$value:expr) => {
        $value.get_i64().unwrap() as ::std::primitive::i8
    };
    ([u16],$value:expr) => {
        $value.get_i64().unwrap() as ::std::primitive::u16
    };
    ([i16],$value:expr) => {
        $value.get_i64().unwrap() as ::std::primitive::i16
    };
    ([u32],$value:expr) => {
        $value.get_i64().unwrap() as ::std::primitive::u32
    };
    ([i32],$value:expr) => {
        $value.get_i64().unwrap() as ::std::primitive::i32
    };
    ([enum [$repr:tt] $name:ty],$value:expr) => {{
        use $crate::structure::ChumEnum;
        <$name>::from_u32($value.get_i64().unwrap() as u32).unwrap()
    }};
    ([flags [$repr:tt] {$($name:ident),*}],$value:expr) => {
        $value.get_i64().unwrap() as $repr
    };
    ([int_custom [$repr:tt] $min:expr, $max:expr],$value:expr) => {
        $value.get_i64().unwrap() as $repr
    };
    ([f32],$value:expr) => {
        $value.get_f32().unwrap()
    };
    ([Transform3D],$value:expr) => {
        *$value.get_transform3d().unwrap()
    };
    ([Transform2D],$value:expr) => {
        *$value.get_transform2d().unwrap()
    };
    ([Vector2],$value:expr) => {
        *$value.get_vec2().unwrap()
    };
    ([Vector3],$value:expr) => {
        *$value.get_vec3().unwrap()
    };
    ([Vector3 rgb],$value:expr) => {{
        let col = $value.get_color().unwrap();
        $crate::common::Vector3::new(col.r, col.g, col.b)
    }};
    ([Color],$value:expr) => {
        *$value.get_color().unwrap()
    };
    ([Quaternion],$value:expr) => {{
        let vec = *$value.get_vec3().unwrap();
        $crate::common::Quaternion::from_euler(vec)
    }};
    ([reference],$value:expr) => {
        $value.get_reference_id().unwrap()
    };
    ([reference $typename:ident],$value:expr) => {
        $value.get_reference_id().unwrap()
    };
    ([fixed array $type:tt $len:literal],$value:expr) => {{
        use ::std::mem::{self, MaybeUninit};
        unsafe {
            let mut arr: [MaybeUninit<chum_struct_get_type!($type)>; $len] =
                { MaybeUninit::uninit().assume_init() };
            for i in 0..$len {
                arr[i] = MaybeUninit::new(chum_struct_destructure!(
                    $type,
                    $value.get_array_item(i).unwrap()
                ));
            }
            mem::transmute::<_, [chum_struct_get_type!($type); $len]>(arr)
        }
    }};
    ([dynamic array [$lentype:tt] $type:tt $default:expr],$value:expr) => {
        $value
            .get_array()
            .unwrap()
            .iter()
            .map(|x| chum_struct_destructure!($type, x))
            .collect()
    };
    ([struct $t:ty],$value:expr) => {{
        $crate::structure::ChumStruct::destructure($value).unwrap()
    }};
    ([option $type:tt $default:expr],$value:expr) => {
        $value
            .get_optional_value()
            .unwrap()
            .map(|x| chum_struct_destructure!($type, x))
    };
}

/// Process a structure function.
/// Results in `None` if the value is ignored.
macro_rules! process_structure {
    ($name:ident,[void],$value:expr,$self:expr) => {
        None
    };
    ($name:ident,[ignore $type:tt $default:expr],$value:expr,$self:expr) => {
        None
    };
    ($name:ident,[
        custom_structure $type:tt
        structure: $structure:expr;
        destructure: $destructure:expr;
    ],$value:expr,$self:expr) => {
        $structure($self)
    };
    ($name:ident,[
        custom_binary $type:tt
        read: $structure:expr;
        write: $destructure:expr;
    ],$value:expr,$self:expr) => {
        process_structure!($name, $type, $value, $self)
    };
    ($name:ident,$type:tt,$value:expr,$self:expr) => {
        Some((
            stringify!($name).to_owned(),
            chum_struct_structure!($type, $value),
        ))
    };
}

/// Process a destructure function.
/// Results in `()` if the value is ignored.
macro_rules! process_destructure {
    ($name:ident,$data:expr,[void]) => {
        ()
    };
    ($name:ident,$data:expr,[ignore $type:tt $default:expr]) => {
        ()
    };
    ($name:ident,$data:expr,[
        custom_structure $type:tt
        structure: $structure:expr;
        destructure: $destructure:expr;
    ]) => {
        $destructure($data)
    };
    ($name:ident,$data:expr,[
        custom_binary $type:tt
        read: $structure:expr;
        write: $destructure:expr;
    ]) => {
        process_destructure!($name, $data, $type)
    };
    ($name:ident,$data:expr,$type:tt) => {
        chum_struct_destructure!($type, $data.get_struct_item(stringify!($name)).unwrap())
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
/// * [int_custom [repr] min max]: int_custom integer in [min, max] range
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
/// * [option [type] default]: An optional value.
///     `default` is the value that will be given if it is empty, then given a value.
/// * [ignore [type] value]: A value that will be ignored and not shown in the structure editor.
///     When writing to a file, `value` will be used.
/// * [custom_structure [type]
///   structure: |value: &Self| -> Option<ChumStructVariant>;
///   destructure: |data: &ChumStructVariant| -> [type];]
///     A custom structure/destructure function.
///     Return `None` from structure for it to not appear in the structure editor.
/// * [custom_binary [type]
///   read: |value: &Self, file: &mut Write, fmt: TotemFormat| -> StructUnpackResult<[type]>;
///   write: |data: &[type], file: &mut Read, fmt: TotemFormat| -> io::Result<()>;]
///     A custom binary read/write function.
///     The type &Self is actually a special version of Self where each property is Option<property type>.
///     This is necessary for data to be safely read from the struct at run time.
#[macro_export]
macro_rules! chum_struct {
    (
        $(
            #[$a:meta]
        )*
        pub struct $structname:ident {
            $(
                pub $name:ident : $type:tt
            ),* $(,)? // this is just so that the last comma is optional
        }
    ) => {
        $(
            #[$a]
        )*
        pub struct $structname {
            $(
                pub $name : chum_struct_get_type!($type),
            )*
        }
        chum_struct_impl! {
            impl ChumStruct for $structname {
                $(
                    $name: $type
                ),*
            }
        }
    };
}

/// Special macro that just implements ChumStruct instead of also defining the struct.
/// Useful for Generic types.
#[macro_export]
macro_rules! chum_struct_impl {
    (
        impl ChumStruct for $structname:ty {
            $(
                $name:ident : $type:tt
            ),* $(,)? // this is just so that the last comma is optional
        }
    ) => {
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
                            process_structure!($name,$type,self.$name,self)
                        },
                    )*
                ].into_iter().filter_map(|e|e).collect())
            }
            fn destructure(data: &$crate::structure::ChumStructVariant) -> Result<Self, ::std::boxed::Box<dyn ::std::error::Error>> {
                Ok(
                    Self {
                        $(
                            $name: process_destructure!($name,data,$type),
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
#[macro_export]
macro_rules! chum_struct_binary_read {
    ([ignore $type:tt $default:expr],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        chum_struct_binary_read!($type, $file, $fmt, $struct, $path, $self).map(|_| ())
    };
    ([void],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        Ok(())
    };
    ([
        custom_structure $type:tt
        structure: $structure:expr;
        destructure: $destructure:expr;
    ],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        chum_struct_binary_read!($type, $file, $fmt, $struct, $path, $self)
    };
    ([
        custom_binary $type:tt
        read: $read:expr;
        write: $write:expr;
    ],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $read($self, $file, $fmt)
    };
    ([u8],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_u8($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([i8],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_i8($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([u16],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_u16($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([i16],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_i16($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([u32],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_u32($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([i32],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_i32($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([enum [$repr:tt] $name:ty],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {{
        use $crate::structure::ChumEnum;
        chum_struct_binary_read!([$repr], $file, $fmt, $struct, $path, $self)
            .map_err(|e| Box::<dyn ::std::error::Error>::from(Box::new(e)))
            .and_then(|x| {
                <$name>::from_u32(x as u32).ok_or_else(|| {
                    Box::new($crate::util::error::EnumerationError {
                        enum_name: stringify!($name).to_owned(),
                        value: x as i64,
                    })
                    .into()
                })
            })
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: e,
            })
    }};
    ([flags [$repr:tt] {$($name:ident),*}],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        chum_struct_binary_read!([$repr], $file, $fmt, $struct, $path, $self)
    };
    ([int_custom [$repr:tt] $min:expr, $max:expr],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        chum_struct_binary_read!([$repr], $file, $fmt, $struct, $path, $self)
    };
    ([f32],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_f32($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([Transform3D],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $crate::common::read_transform3d($file, $fmt).map_err(|e| {
            $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            }
        })
    };
    ([Transform2D],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $crate::common::read_transform2d($file, $fmt).map_err(|e| {
            $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            }
        })
    };
    ([Vector2],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $crate::common::read_vec2($file, $fmt).map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e),
        })
    };
    ([Vector3],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $crate::common::read_vec3($file, $fmt).map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e),
        })
    };
    ([Vector3 rgb],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $crate::common::read_vec3($file, $fmt).map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e),
        })
    };
    ([Color],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $crate::common::read_color_rgba($file, $fmt).map_err(|e| {
            $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            }
        })
    };
    ([Quaternion],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $crate::common::read_quat($file, $fmt).map_err(|e| $crate::util::error::StructUnpackError {
            structname: $struct.to_owned(),
            structpath: $path.to_owned(),
            error: Box::new(e),
        })
    };
    ([reference],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_i32($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([reference $typename:ident],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        $fmt.read_i32($file)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
    };
    ([fixed array $type:tt $len:literal],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {{
        use ::std::mem::{self, MaybeUninit};
        unsafe {
            let mut arr: [MaybeUninit<chum_struct_get_type!($type)>; $len] =
                { MaybeUninit::uninit().assume_init() };
            for i in 0..$len {
                arr[i] = MaybeUninit::new(chum_struct_binary_read!(
                    $type,
                    $file,
                    $fmt,
                    $struct,
                    format!("{}[{}]", $path, i),
                    $self
                )?);
            }
            Ok(mem::transmute::<_, [chum_struct_get_type!($type); $len]>(
                arr,
            ))
        }
    }};
    ([dynamic array [$lentype:tt] $type:tt $default:expr],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {{
        chum_struct_binary_read!([$lentype], $file, $fmt, $struct, $path, $self)
            .map_err(|e| $crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new(e),
            })
            .and_then(|size| {
                let mut vec =
                    Vec::with_capacity((size as usize).min($crate::common::SAFE_CAPACITY_BIG));
                for i in 0..size {
                    vec.push(chum_struct_binary_read!(
                        $type,
                        $file,
                        $fmt,
                        $struct,
                        format!("{}[{}]", $path, i),
                        $self
                    )?)
                }
                Ok(vec)
            })
    }};
    ([struct $t:ty],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {
        match <$t>::read_from($file, $fmt) {
            Ok(value) => Ok(value),
            Err(e) => Err(e.structuralize($struct, &$path)),
        }
    };
    ([option $type:tt $default:expr],$file:expr,$fmt:expr,$struct:expr,$path:expr,$self:expr) => {{
        let has_value =
            $fmt.read_u8($file)
                .map_err(|e| $crate::util::error::StructUnpackError {
                    structname: $struct.to_owned(),
                    structpath: $path.to_owned(),
                    error: Box::new(e),
                })?;
        match has_value {
            0 => Ok(None),
            1 => Ok(Some(chum_struct_binary_read!(
                $type, $file, $fmt, $struct, $path, $self
            )?)),
            o => Err($crate::util::error::StructUnpackError {
                structname: $struct.to_owned(),
                structpath: $path.to_owned(),
                error: Box::new($crate::util::error::EnumerationError {
                    enum_name: "Optional".to_string(),
                    value: o as i64,
                }),
            }),
        }
    }};
}

#[macro_export]
macro_rules! chum_struct_binary_write {
    ([ignore $type:tt $default:expr],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        chum_struct_binary_write!($type, $file, $fmt, &$default, $this)
    };
    ([void],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        ::std::io::Result::<()>::Ok(())
    };
    ([
        custom_structure $type:tt
        structure: $structure:expr;
        destructure: $destructure:expr;
    ],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        chum_struct_binary_write!($type, $file, $fmt, $value, $this)
    };
    ([
        custom_binary $type:tt
        read: $read:expr;
        write: $write:expr;
    ],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $write($value, $file, $fmt)
    };
    ([u8],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_u8($file, *$value)
    };
    ([i8],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_i8($file, *$value)
    };
    ([u16],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_u16($file, *$value)
    };
    ([i16],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_i16($file, *$value)
    };
    ([u32],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_u32($file, *$value)
    };
    ([i32],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_i32($file, *$value)
    };
    ([enum [$repr:tt] $name:ty],$file:expr,$fmt:expr,$value:expr,$this:expr) => {{
        let ivalue = $crate::structure::ChumEnum::to_u32($value) as $repr;
        chum_struct_binary_write!([$repr], $file, $fmt, &ivalue, $this)
    }};
    ([flags [$repr:tt] {$($name:ident),*}],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        chum_struct_binary_write!([$repr], $file, $fmt, $value, $this)
    };
    ([int_custom [$repr:tt] $min:expr, $max:expr],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        chum_struct_binary_write!([$repr], $file, $fmt, $value, $this)
    };
    ([f32],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_f32($file, *$value)
    };
    ([Transform3D],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $crate::common::write_transform3d($value, $file, $fmt)
    };
    ([Transform2D],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $crate::common::write_transform2d($value, $file, $fmt)
    };
    ([Vector2],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $crate::common::write_vec2($value, $file, $fmt)
    };
    ([Vector3],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $crate::common::write_vec3($value, $file, $fmt)
    };
    ([Vector3 rgb],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $crate::common::write_vec3($value, $file, $fmt)
    };
    ([Color],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $crate::common::write_color_rgba($value, $file, $fmt)
    };
    ([Quaternion],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $crate::common::write_quat($value, $file, $fmt)
    };
    ([reference],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_i32($file, *$value)
    };
    ([reference $typename:ident],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        $fmt.write_i32($file, *$value)
    };
    ([fixed array $type:tt $len:literal],$file:expr,$fmt:expr,$value:expr,$this:expr) => {{
        for i in 0..$len {
            chum_struct_binary_write!($type, $file, $fmt, &$value[i], $this)?;
        }
        // fun cheat
        ::std::io::Result::<()>::Ok(())
    }};
    ([dynamic array [$lentype:tt] $type:tt $default:expr],$file:expr,$fmt:expr,$value:expr,$this:expr) => {{
        let lenval = $value.len() as $lentype;
        chum_struct_binary_write!([$lentype], $file, $fmt, &lenval, $this)?;
        for value in $value.iter() {
            chum_struct_binary_write!($type, $file, $fmt, value, $this)?;
        }
        // fun cheat again
        ::std::io::Result::<()>::Ok(())
    }};
    ([struct $t:ty],$file:expr,$fmt:expr,$value:expr,$this:expr) => {
        <$t as $crate::binary::ChumBinary>::write_to($value, $file, $fmt)
    };
    ([option $type:tt $default:expr],$file:expr,$fmt:expr,$value:expr,$this:expr) => {{
        match $value {
            Some(ref x) => {
                $fmt.write_u8($file, 1)?;
                chum_struct_binary_write!($type, $file, $fmt, x, $this)
            }
            None => $fmt.write_u8($file, 0),
        }
    }};
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
                pub $name:ident : $type:tt
            ),* $(,)? // this is just so that the last comma is optional
        }
    ) => {
        chum_struct! {
            $(
                #[$a]
            )*
            pub struct $structname {
                $(
                    pub $name : $type,
                )*
            }
        }
        chum_struct_binary_impl_private! {
            impl ChumBinary for $structname {
                $(
                    $name: $type
                ),*
            }
        }
    }
}

/// ONLY implements ChumBinary.
macro_rules! chum_struct_binary_impl_private {
    (
        impl ChumBinary for $structname:ty {
            $(
                $name:ident : $type:tt
            ),* $(,)? // this is just so that the last comma is optional
        }
    ) => {
        impl $crate::binary::ChumBinary for $structname {
            fn read_from(file: &mut dyn ::std::io::Read, fmt: $crate::format::TotemFormat)
            -> $crate::util::error::StructUnpackResult<Self> {
                // well this is stupid :)
                // Declaring another struct with the same name, purely so that
                // chum_binary can access data before the entire structure has been read.
                // It's all Option so it's not exactly zero-cost, but it should be fast enough.
                pub struct Inner {
                    $(
                        $name: Option<chum_struct_get_type!($type)>
                    ),*
                }
                let mut value = Inner {
                    $(
                        $name: None
                    ),*
                };
                $(
                    value.$name = Some(
                        chum_struct_binary_read!($type, file, fmt, stringify!($structname), stringify!($name), &value)?
                    );
                )*
                Ok(Self {
                    $(
                        $name: value.$name.unwrap()
                    ),*
                })
            }
            fn write_to(&self, writer: &mut dyn ::std::io::Write, fmt: $crate::format::TotemFormat) -> ::std::io::Result<()> {
                $(
                    chum_struct_binary_write!($type, writer, fmt, &self.$name, self)?;
                )*
                Ok(())
            }
        }
    };
}

/// Special macro that just implements ChumStruct and ChumBinary instead of also defining the struct.
/// Useful for Generic types.
#[macro_export]
macro_rules! chum_struct_binary_impl {
    (
        impl ChumBinary for $structname:ty {
            $(
                $name:ident : $type:tt
            ),* $(,)? // this is just so that the last comma is optional
        }
    ) => {
        chum_struct_impl! {
            impl ChumStruct for $structname {
                $(
                    $name: $type
                ),*
            }
        }
        chum_struct_binary_impl_private! {
            impl ChumBinary for $structname {
                $(
                    $name: $type
                ),*
            }
        }
    };
}

/// Special macro that implements ChumBinary and ChumStruct for enumerated types
#[macro_export]
macro_rules! chum_struct_enum {
    (
        $(
            #[$a:meta]
        )*
        pub enum $enumname:ident $enumtype:tt {
            $(
                $variantname:ident: $variantpattern:tt => {
                    $(
                        $name:ident : $type:tt = $default:expr
                    ),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        $(
            #[$a]
        )*
        pub enum $enumname {
            $(
                $variantname {
                    $(
                        $name: chum_struct_get_type!($type)
                    ),*
                }
            ),*
        }
        impl $crate::structure::ChumStruct for $enumname {
            fn structure(&self) -> $crate::structure::ChumStructVariant {
                #![allow(unused_imports)]
                use $crate::structure::ChumStructVariant::*;
                use $crate::structure::IntType::*;
                use $crate::structure::ArrayData;
                use $crate::structure::ColorInfo;
                use $crate::structure::VariantOption;
                let optionvec: Vec<VariantOption> = vec! [
                    $(
                        VariantOption {
                            name: stringify!($variantname).to_owned(),
                            default_value: || {
                                Struct(vec![
                                    $(
                                        {
                                            process_structure!($name,$type,($default),self)
                                        },
                                    )*
                                ].into_iter().filter_map(|e|e).collect())
                            }
                        }
                    ),*
                ];
                match self {
                    $(
                        $enumname::$variantname {
                            $(
                                ref $name
                            ),*
                        } => {
                            Variant {
                                current: stringify!($variantname).to_owned(),
                                options: optionvec,
                                value: Box::new(
                                    Struct(vec![
                                        $(
                                            {
                                                process_structure!($name,$type,*$name,self)
                                            },
                                        )*
                                    ].into_iter().filter_map(|e|e).collect())
                                )
                            }
                        }
                    ),*
                }
            }
            fn destructure(data: &$crate::structure::ChumStructVariant) -> Result<Self, ::std::boxed::Box<dyn ::std::error::Error>> {
                // unimplemented!()
                let inner_name = data.get_variant_name().unwrap();
                let inner_data = data.get_variant_data().unwrap();
                match inner_name {
                    $(
                        stringify!($variantname) => {
                            Ok(
                                $enumname::$variantname {
                                    $(
                                        $name: process_destructure!($name,inner_data,$type)
                                    ),*
                                }
                            )
                        },
                    )*
                    o => {
                        Err(
                            Box::new(
                                $crate::util::error::ChumStructVariantError {
                                    expected: vec![
                                        $(
                                            stringify!($variantname).to_owned()
                                        ),*
                                    ],
                                    value: o.to_owned()
                                }
                            )
                        )
                    }
                }
            }
        }
        impl $crate::binary::ChumBinary for $enumname {
            fn read_from(file: &mut dyn ::std::io::Read, fmt: $crate::format::TotemFormat)
            -> $crate::util::error::StructUnpackResult<Self> {
                let invariant = chum_struct_binary_read!($enumtype,file,fmt,stringify!($enumname),"",())?;
                Ok(match invariant {
                    $(
                        $variantpattern => {
                            pub struct Inner {
                                $(
                                    $name: Option<chum_struct_get_type!($type)>
                                ),*
                            }
                            #[allow(unused_mut,unused_variables)]
                            let mut value = Inner {
                                $(
                                    $name: None
                                ),*
                            };
                            $(
                                value.$name = Some(
                                    chum_struct_binary_read!($type, file, fmt, stringify!($structname), stringify!($name), &value)?
                                );
                            )*
                            $enumname::$variantname {
                                $(
                                    $name: value.$name.unwrap()
                                ),*
                            }
                        }
                    ),*
                    _ => panic!()
                })
            }
            fn write_to(&self, writer: &mut dyn ::std::io::Write, fmt: $crate::format::TotemFormat) -> ::std::io::Result<()> {
                match self {
                    $(
                        $enumname::$variantname {
                            $(
                                $name
                            ),*
                        } => {
                            chum_struct_binary_write!($enumtype,writer,fmt,&($variantpattern),self)?;
                            $(
                                chum_struct_binary_write!($type,writer,fmt,$name,self)?;
                            )*
                        }
                    ),*
                }
                Ok(())
            }
        }
    };
}

/*
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
        pub v_custom: [int_custom [i32] 0, 100],
        pub b: [enum [u8] MyEnum],
        pub c: [flags [u8] {A, B, C}],
        pub v_reference1: [reference],
        pub v_reference2: [reference MATERIAL],
        pub v_struct: [struct Foobar],
        pub v_array_struct: [dynamic array [u32] [struct Foobar] Foobar{v_enum: MyEnum::Zero}],
        pub v_array_u8: [fixed array [u8] 100],
        pub v_vec_u8: [dynamic array [u32] [u8] 100u8],
        pub v_array_custom: [fixed array [int_custom [i32] 0, 100] 100],
    }
}

chum_struct_generate_readwrite! {
    pub struct Example2 {
        pub x: [option [u8] 0],
        pub y: [option [u8] 0],
        pub z: [
            option [
                dynamic array
                [u32]
                [struct Foobar]
                Foobar {
                    v_enum: MyEnum::Zero
                }
            ]
            Vec::new()
        ],
        pub t: [option [struct Foobar] Foobar{v_enum: MyEnum::Zero}],
        pub w: [option [fixed array [int_custom [i32] 0, 100] 100] [0i32; 100]],
        pub q: [
            option [
                option [
                    option [
                        fixed array [u8] 100
                    ]
                    [0u8; 100]
                ]
                None
            ]
            None
        ]
    }
}
*/
