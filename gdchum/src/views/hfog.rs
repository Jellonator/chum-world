use crate::util;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::hfog::*;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct HFogView {
    pub inner: HFog,
}

impl_view!(
    HFogView,
    HFog,
    "HFOG",
    (|_builder: &ClassBuilder<Self>| {}),
    
    fn new(_owner: &Resource) -> Self {
        Self {
            inner: Default::default(),
        }
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
        self.inner = HFog::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }
);
