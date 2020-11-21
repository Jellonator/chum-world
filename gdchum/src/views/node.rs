use gdnative::prelude::*;
use gdnative::api::Resource;
use libchum::reader::node;
use crate::chumfile::ChumFile;
use crate::util;
use std::error::Error;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct NodeView {
    pub inner: node::Node,
}

#[methods]
impl NodeView {
    fn new(_owner: &Resource) -> Self {
        NodeView {
            inner: node::Node::default()
        }
    }

    fn _register(builder: &ClassBuilder<Self>) {
        builder
            .add_property("global_transform")
            .with_getter(Self::get_global_transform)
            .with_setter(Self::set_global_transform)
            .done();
        builder
            .add_property("local_transform")
            .with_getter(Self::get_local_transform)
            .with_setter(Self::set_local_transform)
            .done();
        builder
            .add_property("parent_id")
            .with_getter(Self::get_parent_id)
            .with_setter(Self::set_parent_id)
            .done();
        builder
            .add_property("resource_id")
            .with_getter(Self::get_resource_id)
            .with_setter(Self::set_resource_id)
            .done();
    }

    pub fn set_data(&mut self, data: node::Node) {
        self.inner = data;
    }

    #[export]
    pub fn load(&mut self, _owner: &Resource, data: Instance<ChumFile, Shared>) {
        if let Err(e) = self.load_from(data) {
            display_err!("Error while loading NODE into view: {}", e);
        }
    }

    #[export]
    pub fn save(&self, _owner: &Resource, data: Instance<ChumFile, Shared>) {
        use libchum::binary::ChumBinary;
        let mut v: Vec<u8> = Vec::new();
        unsafe{data.assume_safe()}.map_mut(|chumfile,_| {
            self.inner.write_to(&mut v, chumfile.get_format()).unwrap();
            chumfile.replace_data_with_vec(v);
        }).unwrap();
    }

    pub fn load_from(&mut self, data: Instance<ChumFile, Shared>) -> Result<(), Box<dyn Error>> {
        use libchum::binary::ChumBinary;
        unsafe {
            let data = data.assume_safe();
            self.inner = data.map(|cfile, _| {
                cfile.borrow_data(|mut inner_data| {
                    node::Node::read_from(&mut inner_data, cfile.get_format())
                })
            })??;
        }
        Ok(())
    }

    #[export]
    pub fn set_global_transform(&mut self, _owner: TRef<Resource>, value: Transform) {
        self.inner.global_transform = util::transform_to_mat4x4(&value);
        println!("TX:\n{:?}", self.inner.global_transform);
        let inverse = Transform {
            basis: value.basis.inverted(),
            origin: -value.origin
        };
        self.inner.global_transform_inverse = util::transform_to_mat4x4(&inverse);
    }

    #[export]
    pub fn get_global_transform(&self, _owner: TRef<Resource>) -> Transform {
        util::mat4x4_to_transform(&self.inner.global_transform)
    }

    #[export]
    pub fn set_local_transform(&mut self, _owner: TRef<Resource>, value: Transform) {
        self.inner.local_transform = util::transform_to_mat4x4(&value);
        self.inner.local_translation = util::godot_to_vec3(&value.origin);
        self.inner.local_rotation = util::godot_to_quat(&value.basis.to_quat());
        self.inner.local_scale = util::godot_to_vec3(&value.basis.to_scale());
    }

    #[export]
    pub fn get_local_transform(&self, _owner: TRef<Resource>) -> Transform {
        util::mat4x4_to_transform(&self.inner.local_transform)
    }

    #[export]
    pub fn get_parent_id(&self, _owner: TRef<Resource>) -> i32 {
        self.inner.node_parent_id
    }

    #[export]
    pub fn set_parent_id(&mut self, _owner: TRef<Resource>, value: i32) {
        self.inner.node_parent_id = value;
    }

    #[export]
    pub fn get_resource_id(&self, _owner: TRef<Resource>) -> i32 {
        self.inner.resource_id
    }

    #[export]
    pub fn set_resource_id(&mut self, _owner: TRef<Resource>, value: i32) {
        self.inner.resource_id = value;
    }
}