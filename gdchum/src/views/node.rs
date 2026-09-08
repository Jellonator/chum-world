use crate::util::{self};
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::node;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct NodeView {
    pub inner: node::Node,
}

impl_view!(
    NodeView,
    node::Node,
    "NODE",
    |builder: &ClassBuilder<Self>| {
        builder
            .property("global_transform")
            .with_getter(Self::get_global_transform)
            .with_setter(Self::set_global_transform)
            .done();
        builder
            .property("local_transform")
            .with_getter(Self::get_local_transform)
            .with_setter(Self::set_local_transform)
            .done();
        builder
            .property("parent_id")
            .with_getter(Self::get_parent_id)
            .with_setter(Self::set_parent_id)
            .done();
        builder
            .property("resource_id")
            .with_getter(Self::get_resource_id)
            .with_setter(Self::set_resource_id)
            .done();
    },

    fn new(_owner: &Resource) -> Self {
        NodeView {
            inner: node::Node::default(),
        }
    }

    #[export]
    pub fn set_global_transform(&mut self, owner: TRef<Resource>, value: Transform) -> () {
        self.inner.global_transform = util::godot_to_transform3d(&value);
        let inverse = Transform {
            basis: value.basis.inverted(),
            origin: -value.origin,
        };
        self.inner.global_transform_inverse = util::godot_to_transform3d(&inverse);
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_global_transform(&self, _owner: TRef<Resource>) -> Transform {
        util::transform3d_to_godot(&self.inner.global_transform)
    }

    #[export]
    pub fn set_local_transform(&mut self, owner: TRef<Resource>, value: Transform) -> () {
        self.inner.local_transform = util::godot_to_transform3d(&value);
        self.inner.local_translation = util::convert_vector3d_to_euclid(value.origin);
        self.inner.local_rotation = util::convert_quat_to_euclid(value.basis.to_quat());
        self.inner.local_scale = util::convert_vector3d_to_euclid(value.basis.scale());
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_local_transform(&self, _owner: TRef<Resource>) -> Transform {
        util::transform3d_to_godot(&self.inner.local_transform)
    }

    #[export]
    pub fn get_parent_id(&self, _owner: TRef<Resource>) -> i32 {
        self.inner.node_parent_id
    }

    #[export]
    pub fn set_parent_id(&mut self, owner: TRef<Resource>, value: i32) -> () {
        self.inner.node_parent_id = value;
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_resource_id(&self, _owner: TRef<Resource>) -> i32 {
        self.inner.resource_id
    }

    #[export]
    pub fn set_resource_id(&mut self, owner: TRef<Resource>, value: i32) -> () {
        self.inner.resource_id = value;
        owner.emit_signal("modified", &[]);
    }

    #[method]
    pub fn get_structure(&self) -> Variant {
        use libchum::structure::ChumStruct;
        let data = self.inner.structure();
        util::struct_to_dict(&data).into_shared().to_variant()
    }

    #[export]
    pub fn import_structure(&mut self, owner: &Resource, data: Dictionary) -> () {
        use libchum::structure::ChumStruct;
        let structure = util::dict_to_struct(&data);
        self.inner = node::Node::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }
);
