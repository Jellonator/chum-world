use crate::util;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::omni::*;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct OmniView {
    pub inner: Omni,
}

#[methods]
impl OmniView {
    fn new(_owner: &Resource) -> Self {
        OmniView {
            inner: Default::default(),
        }
    }

    impl_view_node_resource!(OmniView, Omni, "OMNI", |_builder: &ClassBuilder<Self>| {});

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
        self.inner = Omni::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }
}
