use gdnative::prelude::*;
use libchum::reader::collisionvol;
use gdnative::api::Resource;
use crate::util;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct CollisionVolView {
    pub inner: collisionvol::CollisionVol,
}

#[methods]
impl CollisionVolView {
    fn new(_owner: &Resource) -> Self {
        CollisionVolView { inner:  Default::default() }
    }

    impl_view_node_resource!(CollisionVolView, collisionvol::CollisionVol, "COLLISIONVOL",
        |_builder: &ClassBuilder<Self>| {}
    );

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
