use crate::util;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::collisionvol;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct CollisionVolView {
    pub inner: collisionvol::CollisionVol,
}

#[methods]
impl CollisionVolView {
    fn new(_owner: &Resource) -> Self {
        CollisionVolView {
            inner: Default::default(),
        }
    }

    impl_view_node_resource!(
        CollisionVolView,
        collisionvol::CollisionVol,
        "COLLISIONVOL",
        |builder: &ClassBuilder<Self>| {
            builder
                .add_property("local_transform")
                .with_getter(Self::get_local_transform)
                .with_setter(Self::set_local_transform)
                .done();
        }
    );

    #[export]
    pub fn set_local_transform(&mut self, _owner: TRef<Resource>, value: Transform) {
        self.inner.local_transform = util::godot_to_transform3d(&value);
    }

    #[export]
    pub fn get_local_transform(&self, _owner: TRef<Resource>) -> Transform {
        util::transform3d_to_godot(&self.inner.local_transform)
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
        self.inner = collisionvol::CollisionVol::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }
}
