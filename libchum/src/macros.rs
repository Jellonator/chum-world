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

macro_rules! chum_struct_get_type {
    (u8) => {::std::primitive::u8};
    (i8) => {::std::primitive::i8};
    (u16) => {::std::primitive::u16};
    (i16) => {::std::primitive::i16};
    (u32) => {::std::primitive::u32};
    (i32) => {::std::primitive::i32};
    (enum {$($_name:ident),*}) => {::std::primitive::i64};
    (flags {$($_name:ident),*}) => {::std::primitive::i64};
    (custom $_min:expr, $_max:expr) => {::std::primitive::i64};
    (f32) => {::std::primitive::f32};
    (Mat4x4) => {$crate::common::Mat4x4};
    (Mat3x3) => {$crate::common::Mat3x3};
    (Vector2) => {$crate::common::Vector2};
    (Vector3) => {$crate::common::Vector3};
    (Color) => {$crate::common::Color};
    (reference) => {::std::primitive::i32};
    (reference $typename:ident) => {::std::primitive::i32};
    (fixed array [$($inner:tt)*] $len:literal) => {[chum_struct_get_type!($($inner)*);$len]};
    (dynamic array [$($inner:tt)*] $_default:expr) => {Vec<chum_struct_get_type!($($inner)*)>};
    (struct $t:ty) => {$t}
}

macro_rules! chum_struct_structure {
    ([u8],$value:expr) => {Integer($value as ::std::primitive::i64, U8)};
    ([i8],$value:expr) => {Integer($value as ::std::primitive::i64, I8)};
    ([u16],$value:expr) => {Integer($value as ::std::primitive::i64, U16)};
    ([i16],$value:expr) => {Integer($value as ::std::primitive::i64, I16)};
    ([u32],$value:expr) => {Integer($value as ::std::primitive::i64, U32)};
    ([i32],$value:expr) => {Integer($value as ::std::primitive::i64, I32)};
    ([enum {$($name:ident),*}],$value:expr) => {
        Integer($value as ::std::primitive::i64, Enum(
            vec![
                $(
                    stringify!($name).to_owned(),
                )*
            ]
        ))
    };
    ([flags {$($name:ident),*}],$value:expr) => {
        Integer($value as ::std::primitive::i64, Enum(
            vec![
                $(
                    stringify!($name).to_owned(),
                )*
            ]
        ))
    };
    ([custom $min:expr, $max:expr],$value:expr) => {
        Integer($value as ::std::primitive::i64, Custom($min,$max))
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
            default_value: ::std::boxed::Box::new(Integer(0,U8)),//<chum_struct_get_type!($($inner:tt)*)>::default()
        })
    };
    ([dynamic array [$($inner:tt)*] $default:expr],$value:expr) => {
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

macro_rules! chum_struct_destructure {
    ([u8],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::u8};
    ([i8],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::i8};
    ([u16],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::u16};
    ([i16],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::i16};
    ([u32],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::u32};
    ([i32],$value:expr) => {$value.get_i64().unwrap() as ::std::primitive::i32};
    ([enum {$($name:ident),*}],$value:expr) => {$value.get_i64().unwrap()};
    ([flags {$($name:ident),*}],$value:expr) => {$value.get_i64().unwrap()};
    ([custom $min:expr, $max:expr],$value:expr) => {$value.get_i64().unwrap()};
    ([f32],$value:expr) => {$value.get_f32().unwrap()};
    ([Mat4x4],$value:expr) => {*$value.get_transform3d().unwrap()};
    ([Mat3x3],$value:expr) => {*$value.get_transform2d().unwrap()};
    ([Vector2],$value:expr) => {*$value.get_vec2().unwrap()};
    ([Vector3],$value:expr) => {*$value.get_vec3().unwrap()};
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
    ([dynamic array [$($inner:tt)*] $default:expr],$value:expr) => {
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
    } ) => {
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
                Struct(vec![
                    $(
                        (
                            stringify!($name).to_owned(),
                            chum_struct_structure!([$($inner)*],self.$name)
                        ),
                    )*
                ])
            }
            fn destructure(data: &$crate::structure::ChumStructVariant) -> Result<Self, ::std::boxed::Box<dyn ::std::error::Error>> {
                Ok(
                    Self {
                        $(
                            $name: chum_struct_destructure!(
                                [$($inner)*],
                                data.get_struct_item(stringify!($name)).unwrap()),
                        )*
                    }
                )
            }
        }
    };
}

// chum_struct! {
//     pub struct Foobar {
//         pub v_i8: [i8]
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
//         pub v_array_struct: [dynamic array [struct Foobar] Foobar{v_i8:1}],
//         pub v_array_u8: [fixed array [u8] 100],
//         pub v_vec_u8: [dynamic array [u8] 100],
//         pub v_array_custom: [fixed array [custom 0, 100] 100],
//     }
// }