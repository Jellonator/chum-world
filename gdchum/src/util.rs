use gdnative::api::Engine;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::common;
use libchum::structure::{ArrayData, ChumStructVariant, ColorInfo, IntType};

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct StructGenerator {
    pub generator: fn() -> ChumStructVariant,
}

#[methods]
impl StructGenerator {
    fn new(_owner: &Resource) -> Self {
        StructGenerator {
            generator: || ChumStructVariant::Struct(vec![]),
        }
    }

    #[export]
    pub fn generate(&self, _owner: &Resource) -> Dictionary<Unique> {
        let fnptr = self.generator;
        struct_to_dict(&fnptr())
    }

    fn set_generator(&mut self, value: fn() -> ChumStructVariant) {
        self.generator = value
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MessageLevel {
    Information,
    Warning,
    Error,
}

impl MessageLevel {
    pub fn to_i64(&self) -> i64 {
        match self {
            MessageLevel::Information => 0,
            MessageLevel::Warning => 1,
            MessageLevel::Error => 2,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            MessageLevel::Information => "INFO",
            MessageLevel::Warning => "WARN",
            MessageLevel::Error => "ERR",
        }
    }
}

pub fn get_filename(name: &str) -> &str {
    match name.rfind('>') {
        Some(pos) => &name[pos + 1..],
        None => name,
    }
}

pub fn get_basename(name: &str) -> &str {
    let name = get_filename(name);
    match name.find('.') {
        Some(pos) => &name[..pos],
        None => name,
    }
}

pub fn push_message(value: &str, level: MessageLevel) {
    println!("{}: {}", level.get_name(), value);
    use gdnative::api::MainLoop;
    let engine = Engine::godot_singleton();
    unsafe {
        let mainloop: &MainLoop = engine.get_main_loop().unwrap().assume_safe().as_ref();
        let scenetree: &SceneTree = mainloop.cast().unwrap();
        let root = scenetree.root().unwrap();
        let lognode = root.assume_safe().get_node("MessageOverlay").unwrap();
        lognode
            .assume_safe()
            .call("push", &[value.to_variant(), level.to_i64().to_variant()]);
    }
}

#[macro_export]
macro_rules! display_info {
    ($($arg:tt)*) => {
        use $crate::util::{MessageLevel, push_message};
        push_message(&format!($($arg)*), MessageLevel::Information)
    };
}

#[macro_export]
macro_rules! display_warn {
    ($($arg:tt)*) => {
        use $crate::util::{MessageLevel, push_message};
        push_message(&format!($($arg)*), MessageLevel::Warning)
    };
}

#[macro_export]
macro_rules! display_err {
    ($($arg:tt)*) => {
        use $crate::util::{MessageLevel, push_message};
        push_message(&format!($($arg)*), MessageLevel::Error)
    };
}

pub fn transform3d_to_godot(tx: &common::Transform3D) -> Transform {
    Transform {
        basis: Basis {
            elements: [
                Vector3::new(tx.m11, tx.m21, tx.m31),
                Vector3::new(tx.m12, tx.m22, tx.m32),
                Vector3::new(tx.m13, tx.m23, tx.m33),
            ],
        },
        origin: Vector3::new(tx.m41, tx.m42, tx.m43),
    }
}

pub fn godot_to_transform3d(tx: &Transform) -> common::Transform3D {
    common::Transform3D::new(
        tx.basis.elements[0].x,
        tx.basis.elements[1].x,
        tx.basis.elements[2].x,
        0.0,
        tx.basis.elements[0].y,
        tx.basis.elements[1].y,
        tx.basis.elements[2].y,
        0.0,
        tx.basis.elements[0].z,
        tx.basis.elements[1].z,
        tx.basis.elements[2].z,
        0.0,
        tx.origin.x,
        tx.origin.y,
        tx.origin.z,
        1.0,
    )
}

/*
pub fn quat_to_godot(value: &common::Quaternion) -> Quat {
    Quat::quaternion(value[0], value[1], value[2], value[3])
}

pub fn godot_to_quat(value: &Quat) -> common::Quaternion {
    common::Quaternion::new(value.r, value.i, value.j, value.k)
}

pub fn vec3_to_godot(value: &common::Vector3) -> Vector3 {
    Vector3::new(value.x, value.y, value.z)
}

pub fn vec2_to_godot(value: &common::Vector2) -> Vector2 {
    Vector2::new(value.x, value.y)
}

pub fn godot_to_vec2(value: &Vector2) -> common::Vector2 {
    common::Vector2::new(value.x, value.y)
}

pub fn godot_to_vec3(value: &Vector3) -> common::Vector3 {
    common::Vector3::new(value.x, value.y, value.z)
}

pub fn mat4x4_to_transform(tx: &common::Mat4x4) -> Transform {
    let mat = tx.as_slice();
    Transform {
        basis: Basis {
            elements: [
                Vector3::new(mat[0], mat[1], mat[2]),
                Vector3::new(mat[4], mat[5], mat[6]),
                Vector3::new(mat[8], mat[9], mat[10]),
            ],
        },
        origin: Vector3::new(mat[3], mat[7], mat[11]),
    }
}

pub fn mat3x3_to_transform2d(tx: &common::Mat3x3) -> Transform2D {
    let mat = tx.as_slice();
    Transform2D::new(mat[0], mat[3], mat[1], mat[4], mat[2], mat[5])
}

pub fn transform_to_mat4x4(value: &Transform) -> common::Mat4x4 {
    common::Mat4x4::from_row_slice(&[
        value.basis.elements[0].x,
        value.basis.elements[0].y,
        value.basis.elements[0].z,
        value.origin.x,
        value.basis.elements[1].x,
        value.basis.elements[1].y,
        value.basis.elements[1].z,
        value.origin.y,
        value.basis.elements[2].x,
        value.basis.elements[2].y,
        value.basis.elements[2].z,
        value.origin.z,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

pub fn transform2d_to_mat3x3(value: &Transform2D) -> common::Mat3x3 {
    let array = value.to_array();
    common::Mat3x3::from_row_slice(&[
        array[0], array[2], array[4], array[1], array[3], array[5], 0.0, 0.0, 1.0,
    ])
}
*/

/// Convert a Godot Dictionary to a ChumStructVariant
pub fn dict_to_struct(dict: &Dictionary) -> ChumStructVariant {
    let value_type = dict.get("type").try_to_string().unwrap();
    match value_type.as_str() {
        "enum" => {
            let value = dict.get("value").try_to_i64().unwrap();
            let names = dict.get("names").try_to_string_array().unwrap();
            let mut value_names: Vec<String> = Vec::new();
            for i in 0..names.len() {
                value_names.push(names.get(i).to_string());
            }
            ChumStructVariant::Integer(value, IntType::Enum(value_names))
        }
        "flags" => {
            let value = dict.get("value").try_to_i64().unwrap();
            let names = dict.get("names").try_to_string_array().unwrap();
            let mut value_names: Vec<String> = Vec::new();
            for i in 0..names.len() {
                value_names.push(names.get(i).to_string());
            }
            ChumStructVariant::Integer(value, IntType::Flags(value_names))
        }
        "integer" => {
            let value = dict.get("value").try_to_i64().unwrap();
            let int_type = dict.get("integer").try_to_string().unwrap();
            let t = match int_type.as_str() {
                "I8" => IntType::I8,
                "U8" => IntType::U8,
                "I16" => IntType::I16,
                "U16" => IntType::U16,
                "I32" => IntType::I32,
                "U32" => IntType::U32,
                "custom" => {
                    let vmin = dict.get("min").try_to_i64().unwrap();
                    let vmax = dict.get("max").try_to_i64().unwrap();
                    IntType::Custom(vmin, vmax)
                }
                _ => panic!("Invalid integer type"),
            };
            ChumStructVariant::Integer(value, t)
        }
        "float" => {
            let value = dict.get("value").try_to_f64().unwrap() as f32;
            ChumStructVariant::Float(value)
        }
        "vec2" => {
            let value = dict.get("value").try_to_vector2().unwrap();
            ChumStructVariant::Vec2(common::Vector2::new(value.x, value.y))
        }
        "vec3" => {
            let value = dict.get("value").try_to_vector3().unwrap();
            ChumStructVariant::Vec3(common::Vector3::new(value.x, value.y, value.z))
        }
        "transform3d" => {
            let value = dict.get("value").try_to_transform().unwrap();
            let mat = godot_to_transform3d(&value);
            ChumStructVariant::Transform3D(mat)
        }
        "transform2d" => {
            let value = dict.get("value").try_to_transform2d().unwrap();
            ChumStructVariant::Transform2D(value)
        }
        "color" => {
            let value = dict.get("value").try_to_color().unwrap();
            let alpha = dict.get("has_alpha").try_to_bool().unwrap();
            ChumStructVariant::Color(
                common::ColorRGBA::new(value.r, value.g, value.b, value.a),
                ColorInfo { has_alpha: alpha },
            )
        }
        "reference" => {
            let value = dict.get("value").try_to_i64().unwrap() as i32;
            let typename = dict.get("reference");
            let reference = match typename.get_type() {
                VariantType::Nil => None,
                VariantType::GodotString => Some(typename.try_to_string().unwrap()),
                _ => panic!(),
            };
            ChumStructVariant::Reference(value, reference)
        }
        "array" => {
            let array = dict.get("value").try_to_array().unwrap();
            let can_resize = dict.get("can_resize").try_to_bool().unwrap();
            let mut values = Vec::new();
            for i in 0..array.len() {
                values.push(dict_to_struct(&array.get(i).try_to_dictionary().unwrap()));
            }
            ChumStructVariant::Array(ArrayData {
                data: values,
                default_value: || unimplemented!(),
                can_resize,
            })
        }
        "struct" => {
            let values_dict = dict.get("value").try_to_dictionary().unwrap();
            let values_order = dict.get("order").try_to_string_array().unwrap();
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
        "option" => {
            let value = if dict.contains("value") {
                let value_dict = dict.get("value").try_to_dictionary().unwrap();
                Some(Box::new(dict_to_struct(&value_dict)))
            } else {
                None
            };
            ChumStructVariant::Optional {
                value,
                default_value: || unimplemented!(),
            }
        }
        "variant" => ChumStructVariant::Variant {
            current: dict.get("current").try_to_string().unwrap(),
            options: vec![],
            value: Box::new(dict_to_struct(
                &dict.get("value").try_to_dictionary().unwrap(),
            )),
        },
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
pub fn struct_to_dict(value: &ChumStructVariant) -> Dictionary<Unique> {
    match value {
        ChumStructVariant::Integer(value, IntType::Enum(ref names)) => {
            let dict = Dictionary::new();
            let mut namearray: StringArray = StringArray::new();
            namearray.resize(names.len() as i32);
            {
                let mut write = namearray.write();
                for i in 0..names.len() {
                    write[i] = GodotString::from(&names[i]);
                }
            }
            dict.insert("type", "enum");
            dict.insert("value", *value);
            dict.insert("names", Variant::from_string_array(&namearray));
            dict
        }
        ChumStructVariant::Integer(value, IntType::Flags(ref names)) => {
            let dict = Dictionary::new();
            let mut namearray: StringArray = StringArray::new();
            namearray.resize(names.len() as i32);
            {
                let mut write = namearray.write();
                for i in 0..names.len() {
                    write[i] = GodotString::from(&names[i]);
                }
            }
            dict.insert("type", "flags");
            dict.insert("value", *value);
            dict.insert("names", namearray);
            dict
        }
        ChumStructVariant::Integer(value, ref t) => {
            let dict = Dictionary::new();
            let (minv, maxv) = t.get_range();
            dict.insert("type", "integer");
            dict.insert("value", *value);
            dict.insert("min", minv);
            dict.insert("max", maxv);
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
            dict.insert("integer", int_type_name);
            dict
        }
        ChumStructVariant::Float(value) => {
            let dict = Dictionary::new();
            dict.insert("type", "float");
            dict.insert("value", *value as f64);
            dict
        }
        ChumStructVariant::Vec2(value) => {
            let dict = Dictionary::new();
            dict.insert("type", "vec2");
            dict.insert("value", Vector2::new(value.x, value.y));
            dict
        }
        ChumStructVariant::Vec3(value) => {
            let dict = Dictionary::new();
            dict.insert("type", "vec3");
            dict.insert("value", Vector3::new(value.x, value.y, value.z));
            dict
        }
        ChumStructVariant::Transform3D(value) => {
            let dict = Dictionary::new();
            let transform = transform3d_to_godot(value);
            dict.insert("type", "transform3d");
            dict.insert("value", transform);
            dict
        }
        ChumStructVariant::Transform2D(value) => {
            let dict = Dictionary::new();
            dict.insert("type", "transform2d");
            dict.insert("value", value);
            dict
        }
        ChumStructVariant::Color(color, info) => {
            let dict = Dictionary::new();
            dict.insert("type", "color");
            dict.insert("has_alpha", info.has_alpha);
            dict.insert("value", Color::rgba(color.r, color.g, color.b, color.a));
            dict
        }
        ChumStructVariant::Reference(value, ref typename) => {
            let dict = Dictionary::new();
            dict.insert("type", "reference");
            dict.insert("value", *value as i64);
            dict.insert(
                "reference",
                match typename {
                    Some(ref val) => Variant::from_str(val),
                    None => Variant::new(),
                },
            );
            dict
        }
        ChumStructVariant::Array(ref data) => {
            let dict = Dictionary::new();
            let default_value = Instance::<StructGenerator, Unique>::new();
            default_value
                .map_mut(|gen, _| gen.set_generator(data.default_value))
                .unwrap();
            let array: VariantArray<Unique> = VariantArray::new();
            for value in data.data.iter() {
                let valuedict = struct_to_dict(value);
                array.push(valuedict);
            }
            dict.insert("type", "array");
            dict.insert("value", array);
            dict.insert("default", default_value);
            dict.insert("can_resize", data.can_resize);
            dict
        }
        ChumStructVariant::Struct(ref vec) => {
            let dict = Dictionary::new();
            let values = Dictionary::new();
            let mut order = StringArray::new();
            order.resize(vec.len() as i32);
            {
                let mut write = order.write();
                for (i, (name, value)) in vec.iter().enumerate() {
                    write[i] = GodotString::from(name);
                    let valuedict = struct_to_dict(value);
                    values.insert(name, valuedict);
                }
            }
            dict.insert("type", "struct");
            dict.insert("value", values);
            dict.insert("order", order);
            dict
        }
        ChumStructVariant::Optional {
            ref value,
            default_value,
        } => {
            let dict = Dictionary::new();
            let default_value_generator = Instance::<StructGenerator, Unique>::new();
            default_value_generator
                .map_mut(|gen, _| gen.set_generator(*default_value))
                .unwrap();
            dict.insert("type", "option");
            dict.insert("default", default_value_generator);
            if let Some(ref inner) = value {
                let inner_dict = struct_to_dict(inner);
                dict.insert("value", inner_dict);
            }
            dict
        }
        ChumStructVariant::Variant {
            ref current,
            ref value,
            ref options,
        } => {
            let dict = Dictionary::new();
            dict.insert("type", "variant");
            dict.insert("current", current);
            let value = struct_to_dict(value);
            dict.insert("value", value);
            let option_dict = Dictionary::new();
            let order = VariantArray::new();
            for option in options.iter() {
                let default_value = Instance::<StructGenerator, Unique>::new();
                default_value
                    .map_mut(|gen, _| gen.set_generator(option.default_value))
                    .unwrap();
                option_dict.insert(&option.name, default_value);
                order.push(&option.name);
            }
            dict.insert("options", option_dict);
            dict.insert("order", order);
            dict
        }
    }
}
