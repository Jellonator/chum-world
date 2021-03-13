use crate::util;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::material;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct MaterialView {
    pub inner: material::Material,
}

#[methods]
impl MaterialView {
    fn new(_owner: &Resource) -> Self {
        MaterialView {
            inner: Default::default(),
        }
    }

    impl_view!(
        MaterialView,
        material::Material,
        "MATERIAL",
        |builder: &ClassBuilder<Self>| {
            builder
                .add_property("color")
                .with_getter(Self::get_color)
                .with_setter(Self::set_color)
                .done();
            builder
                .add_property("emission_color")
                .with_getter(Self::get_emission_color)
                .with_setter(Self::set_emission_color)
                .done();
            builder
                .add_property("transform")
                .with_getter(Self::get_transform)
                .with_setter(Self::set_transform)
                .done();
            builder
                .add_property("texture")
                .with_getter(Self::get_texture)
                .with_setter(Self::set_texture)
                .done();
            builder
                .add_property("reflection_texture")
                .with_getter(Self::get_reflection_texture)
                .with_setter(Self::set_reflection_texture)
                .done();
        }
    );

    #[export]
    pub fn get_color(&self, _owner: TRef<Resource>) -> Color {
        util::color_to_godot(&self.inner.color)
    }

    #[export]
    pub fn set_color(&mut self, owner: TRef<Resource>, value: Color) {
        self.inner.color = util::godot_to_color(&value);
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_emission_color(&self, _owner: TRef<Resource>) -> Color {
        util::color3_to_godot(&self.inner.emission)
    }

    #[export]
    pub fn set_emission_color(&mut self, owner: TRef<Resource>, value: Color) {
        self.inner.emission = util::godot_to_color3(&value);
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_transform(&self, _owner: TRef<Resource>) -> Transform2D {
        self.inner.transform.clone()
    }

    #[export]
    pub fn set_transform(&mut self, owner: TRef<Resource>, value: Transform2D) {
        self.inner.offset = Vector2::new(value.m31, value.m32);
        self.inner.rotation = util::get_transform2d_angle(&value);
        self.inner.scale = util::get_transform2d_scale(&value);
        self.inner.transform = value;
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_texture(&self, _owner: TRef<Resource>) -> i64 {
        self.inner.texture as i64
    }

    #[export]
    pub fn set_texture(&mut self, owner: TRef<Resource>, value: i64) {
        self.inner.texture = value as i32;
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_reflection_texture(&self, _owner: TRef<Resource>) -> i64 {
        self.inner.texture_reflection as i64
    }

    #[export]
    pub fn set_reflection_texture(&mut self, owner: TRef<Resource>, value: i64) {
        self.inner.texture_reflection = value as i32;
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_structure(&self, _owner: &Resource) -> Variant {
        use libchum::structure::ChumStruct;
        let data = self.inner.structure();
        util::struct_to_dict(&data).into_shared().to_variant()
    }

    #[export]
    pub fn import_structure(&mut self, owner: &Resource, data: Dictionary) {
        use libchum::structure::ChumStruct;
        let structure = util::dict_to_struct(&data);
        self.inner = material::Material::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }
}
