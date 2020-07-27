use gdnative::*;
use libchum::common;
use libchum::structure::{ArrayData, ChumStructVariant, IntType};

pub fn vec3_to_godot(value: &common::Vector3) -> Vector3 {
    Vector3::new(value.x, value.y, value.z)
}

pub fn vec2_to_godot(value: &common::Vector2) -> Vector2 {
    Vector2::new(value.x, value.y)
}

pub fn mat4x4_to_transform(tx: &common::Mat4x4) -> Transform {
    let mat = tx.as_slice();
    Transform {
        basis: Basis {
            elements: [
                Vector3::new(mat[0], mat[4], mat[8]),
                Vector3::new(mat[1], mat[5], mat[9]),
                Vector3::new(mat[2], mat[6], mat[10]),
            ],
        },
        origin: Vector3::new(mat[3], mat[7], mat[11]),
    }
}

pub fn mat3x3_to_transform2d(tx: &common::Mat3x3) -> Transform2D {
    let mat = tx.as_slice();
    Transform2D::row_major(mat[0], mat[3], mat[1], mat[4], mat[2], mat[5])
}

pub fn transform_to_mat4x4(value: &Transform) -> common::Mat4x4 {
    common::Mat4x4::from_row_slice(
        &[
            value.basis.elements[0].x,
            value.basis.elements[0].y,
            value.basis.elements[0].z,
            0.0,
            value.basis.elements[1].x,
            value.basis.elements[1].y,
            value.basis.elements[1].z,
            0.0,
            value.basis.elements[2].x,
            value.basis.elements[2].y,
            value.basis.elements[2].z,
            0.0,
            value.origin.x,
            value.origin.y,
            value.origin.z,
            1.0,
        ]
    )
}

pub fn transform2d_to_mat3x3(value: &Transform2D) -> common::Mat3x3 {
    let array = value.to_row_major_array();
    common::Mat3x3::from_row_slice(
        &[
            array[0], array[1], 0.0, array[2], array[3], 0.0, array[4], array[5], 1.0,
        ]
    )
}

/// Convert a Godot Dictionary to a ChumStructVariant
pub fn dict_to_struct(dict: &Dictionary) -> ChumStructVariant {
    let value_type = dict.get_ref(&"type".into()).try_to_string().unwrap();
    match value_type.as_str() {
        "enum" => {
            let value = dict.get_ref(&"value".into()).try_to_i64().unwrap();
            let names = dict.get_ref(&"names".into()).try_to_string_array().unwrap();
            let mut value_names: Vec<String> = Vec::new();
            for i in 0..names.len() {
                value_names.push(names.get(i).to_string());
            }
            ChumStructVariant::Integer(value, IntType::Enum(value_names))
        }
        "flags" => {
            let value = dict.get_ref(&"value".into()).try_to_i64().unwrap();
            let names = dict.get_ref(&"names".into()).try_to_string_array().unwrap();
            let mut value_names: Vec<String> = Vec::new();
            for i in 0..names.len() {
                value_names.push(names.get(i).to_string());
            }
            ChumStructVariant::Integer(value, IntType::Flags(value_names))
        }
        "integer" => {
            let value = dict.get_ref(&"value".into()).try_to_i64().unwrap();
            let int_type = dict.get_ref(&"integer".into()).try_to_string().unwrap();
            let t = match int_type.as_str() {
                "I8" => IntType::I8,
                "U8" => IntType::U8,
                "I16" => IntType::I16,
                "U16" => IntType::U16,
                "I32" => IntType::I32,
                "U32" => IntType::U32,
                "custom" => {
                    let vmin = dict.get_ref(&"min".into()).try_to_i64().unwrap();
                    let vmax = dict.get_ref(&"max".into()).try_to_i64().unwrap();
                    IntType::Custom(vmin, vmax)
                }
                _ => panic!("Invalid integer type"),
            };
            ChumStructVariant::Integer(value, t)
        }
        "float" => {
            let value = dict.get_ref(&"value".into()).try_to_f64().unwrap() as f32;
            ChumStructVariant::Float(value)
        }
        "vec2" => {
            let value = dict.get_ref(&"value".into()).try_to_vector2().unwrap();
            ChumStructVariant::Vec2(common::Vector2::new(value.x, value.y))
        }
        "vec3" => {
            let value = dict.get_ref(&"value".into()).try_to_vector3().unwrap();
            ChumStructVariant::Vec3(common::Vector3::new(value.x, value.y, value.z))
        }
        "transform3d" => {
            let value = dict.get_ref(&"value".into()).try_to_transform().unwrap();
            let mat = transform_to_mat4x4(&value);
            ChumStructVariant::Transform3D(mat)
        }
        "transform2d" => {
            let value = dict.get_ref(&"value".into()).try_to_transform2d().unwrap();
            let mat = transform2d_to_mat3x3(&value);
            ChumStructVariant::Transform2D(mat)
        }
        "color" => {
            let value = dict.get_ref(&"value".into()).try_to_color().unwrap();
            ChumStructVariant::Color(common::Color::new(
                value.r, value.g, value.b, value.a
            ))
        }
        "reference" => {
            let value = dict.get_ref(&"value".into()).try_to_i64().unwrap() as i32;
            let typename = dict.get_ref(&"reference".into());
            let reference = match typename.get_type() {
                VariantType::Nil => None,
                VariantType::GodotString => Some(typename.try_to_string().unwrap()),
                _ => panic!(),
            };
            ChumStructVariant::Reference(value, reference)
        }
        "array" => {
            let array = dict.get(&"value".into()).try_to_array().unwrap();
            let default_dict = dict.get(&"default".into()).try_to_dictionary().unwrap();
            let can_resize = dict.get(&"can_resize".into()).try_to_bool().unwrap();
            let default_value = dict_to_struct(&default_dict);
            let mut values = Vec::new();
            for i in 0..array.len() {
                values.push(dict_to_struct(
                    &array.get_ref(i).try_to_dictionary().unwrap(),
                ));
            }
            ChumStructVariant::Array(ArrayData {
                data: values,
                default_value: Box::new(default_value),
                can_resize,
            })
        }
        "struct" => {
            let values_dict = dict.get(&"value".into()).try_to_dictionary().unwrap();
            let values_order = dict.get(&"order".into()).try_to_string_array().unwrap();
            let mut values = Vec::new();
            for i in 0..values_order.len() {
                let name = values_order.get(i).to_string();
                let value_dict = values_dict
                    .get(&Variant::from_str(&name))
                    .try_to_dictionary()
                    .unwrap();
                let value = dict_to_struct(&value_dict);
                values.push((name, value));
            }
            ChumStructVariant::Struct(values)
        }
        other => panic!("Invalid variant type {}", other),
    }
}

/// Convert a ChumStructVariant to a format usable by Godot
/// The following dictionary will one of the following formats:
/// | "enum" | "flags" | "integer" | "float" | "vec2" | "vec3" | "color" |
/// | "reference" | "transform3d" | "transform2d" | "array" | "struct" |
/// {
///     "type": "enum",
///     "value": int,
///     "names": PoolStringArray
/// }
/// {
///     "type": "flags",
///     "value": int,
///     "names": PoolStringArray
/// }
/// {
///     "type": "integer",
///     "value": int,
///     "integer": "U8" | "I8" | "U16" | "I16" | "U32" | "I32",
///     "min": int,
///     "max": int
/// }
/// {
///     "type": "float",
///     "value": float,
/// }
/// {
///     "type": "vec2",
///     "value": Vector2,
/// }
/// {
///     "type": "vec3",
///     "value": Vector3,
/// }
/// {
///     "type": "transform3d",
///     "value": Transform,
/// }
/// {
///     "type": "transform2d",
///     "value": Transform2D,
/// }
/// {
///     "type": "color",
///     "value": Color,
/// }
/// {
///     "type": "reference",
///     "value": i32,
///     "reference": String/null
/// }
/// {
///     "type": "array",
///     "value": Array<Dictionary>,
///     "default": Dictionary,
///     "can_resize": bool
/// }
/// {
///     "type": "struct",
///     "value": Dictionary<String, Dictionary>,
///     "order": PoolStringArray
/// }
pub fn struct_to_dict(value: &ChumStructVariant) -> Dictionary {
    match value {
        ChumStructVariant::Integer(value, IntType::Enum(ref names)) => {
            let mut dict = Dictionary::new();
            let mut namearray: StringArray = StringArray::new();
            for value in names.iter() {
                namearray.push(&value.into());
            }
            dict.set(&"type".into(), &"enum".into());
            dict.set(&"value".into(), &(*value).into());
            dict.set(&"names".into(), &Variant::from_string_array(&namearray));
            dict
        }
        ChumStructVariant::Integer(value, IntType::Flags(ref names)) => {
            let mut dict = Dictionary::new();
            let mut namearray: StringArray = StringArray::new();
            for value in names.iter() {
                namearray.push(&value.into());
            }
            dict.set(&"type".into(), &"flags".into());
            dict.set(&"value".into(), &(*value).into());
            dict.set(&"names".into(), &Variant::from_string_array(&namearray));
            dict
        }
        ChumStructVariant::Integer(value, ref t) => {
            let mut dict = Dictionary::new();
            let (minv, maxv) = t.get_range();
            dict.set(&"type".into(), &"integer".into());
            dict.set(&"value".into(), &(*value).into());
            dict.set(&"min".into(), &minv.into());
            dict.set(&"max".into(), &maxv.into());
            let int_type_name = match t {
                IntType::I8 => "I8",
                IntType::U8 => "U8",
                IntType::I16 => "I16",
                IntType::U16 => "U16",
                IntType::I32 => "I32",
                IntType::U32 => "U32",
                IntType::Custom(_, _) => "custom",
                IntType::Enum(_) | IntType::Flags(_) => panic!("Should not be reachable"),
            };
            dict.set(&"integer".into(), &int_type_name.into());
            dict
        }
        ChumStructVariant::Float(value) => {
            let mut dict = Dictionary::new();
            dict.set(&"type".into(), &"float".into());
            dict.set(&"value".into(), &Variant::from_f64(*value as f64));
            dict
        }
        ChumStructVariant::Vec2(value) => {
            let mut dict = Dictionary::new();
            dict.set(&"type".into(), &"vec2".into());
            dict.set(
                &"value".into(),
                &Variant::from_vector2(&Vector2::new(value.x, value.y)),
            );
            dict
        }
        ChumStructVariant::Vec3(value) => {
            let mut dict = Dictionary::new();
            dict.set(&"type".into(), &"vec3".into());
            dict.set(
                &"value".into(),
                &Variant::from_vector3(&Vector3::new(value.x, value.y, value.z)),
            );
            dict
        }
        ChumStructVariant::Transform3D(value) => {
            let mut dict = Dictionary::new();
            let transform = mat4x4_to_transform(value);
            dict.set(&"type".into(), &"transform3d".into());
            dict.set(&"value".into(), &Variant::from_transform(&transform));
            dict
        }
        ChumStructVariant::Transform2D(value) => {
            let mut dict = Dictionary::new();
            let transform = mat3x3_to_transform2d(value);
            dict.set(&"type".into(), &"transform2d".into());
            dict.set(&"value".into(), &Variant::from_transform2d(&transform));
            dict
        }
        ChumStructVariant::Color(color) => {
            let mut dict = Dictionary::new();
            dict.set(&"type".into(), &"color".into());
            dict.set(
                &"value".into(),
                &Variant::from_color(&Color::rgba(
                    color[0],
                    color[1],
                    color[2],
                    color[3]
                )),
            );
            dict
        }
        ChumStructVariant::Reference(value, ref typename) => {
            let mut dict = Dictionary::new();
            dict.set(&"type".into(), &"reference".into());
            dict.set(&"value".into(), &Variant::from_i64(*value as i64));
            dict.set(
                &"reference".into(),
                &match typename {
                    Some(ref val) => Variant::from_str(val),
                    None => Variant::new(),
                },
            );
            dict
        }
        ChumStructVariant::Array(ref data) => {
            let mut dict = Dictionary::new();
            let default_value = struct_to_dict(data.default_value.as_ref());
            let mut array: VariantArray = VariantArray::new();
            for value in data.data.iter() {
                let valuedict = struct_to_dict(value);
                array.push(&Variant::from_dictionary(&valuedict));
            }
            dict.set(&"type".into(), &"array".into());
            dict.set(&"value".into(), &Variant::from_array(&array));
            dict.set(&"default".into(), &Variant::from_dictionary(&default_value));
            dict.set(&"can_resize".into(), &Variant::from_bool(data.can_resize));
            dict
        }
        ChumStructVariant::Struct(ref vec) => {
            let mut dict = Dictionary::new();
            let mut values = Dictionary::new();
            let mut order = StringArray::new();
            for (name, value) in vec {
                order.push(&name.into());
                let valuedict = struct_to_dict(value);
                values.set(&name.into(), &Variant::from_dictionary(&valuedict));
            }
            dict.set(&"type".into(), &"struct".into());
            dict.set(&"value".into(), &Variant::from_dictionary(&values));
            dict.set(&"order".into(), &Variant::from_string_array(&order));
            dict
        }
    }
}
