use libchum::reader::lod;
use gdnative::prelude::*;
use gdnative::api::Resource;
use crate::util;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct LodView {
    pub inner: lod::Lod,
}

#[methods]
impl LodView {
    fn new(_owner: &Resource) -> Self {
        LodView { inner:  Default::default() }
    }

    impl_view_node_resource!(LodView, lod::Lod, "LOD",
        |builder: &ClassBuilder<Self>| {
        }
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
        self.inner = lod::Lod::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }
}
