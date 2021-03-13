use crate::util;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::warp::*;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct WarpView {
    pub inner: Warp,
}

#[methods]
impl WarpView {
    fn new(_owner: &Resource) -> Self {
        WarpView {
            inner: Default::default(),
        }
    }

    impl_view!(WarpView, Warp, "WARP", |_builder: &ClassBuilder<Self>| {});

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
        self.inner = Warp::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_materials(&self, _owner: &Resource) -> VariantArray {
        let arr = VariantArray::new();
        for value in self.inner.material_ids.iter() {
            arr.push(*value);
        }
        arr.into_shared()
    }

    #[export]
    pub fn get_vertices(&self, _owner: &Resource) -> VariantArray {
        let arr = VariantArray::new();
        for value in self.inner.vertices.iter() {
            arr.push(*value);
        }
        arr.into_shared()
    }

    #[export]
    pub fn get_texcoords(&self, _owner: &Resource) -> VariantArray {
        let arr = VariantArray::new();
        for value in self.inner.texcoords.iter() {
            arr.push(*value);
        }
        arr.into_shared()
    }

    #[export]
    pub fn get_size(&self, _owner: &Resource) -> f32 {
        self.inner.size
    }
}
