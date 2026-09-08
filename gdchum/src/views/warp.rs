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

impl_view!(
    WarpView,
    Warp,
    "WARP",
    (|_builder: &ClassBuilder<Self>| {}),

    fn new(_owner: &Resource) -> Self {
        WarpView {
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
        self.inner = Warp::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }

    #[method]
    pub fn get_materials(&self) -> VariantArray {
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
            arr.push(util::convert_vector3d_to_gd(*value));
        }
        arr.into_shared()
    }

    #[export]
    pub fn get_texcoords(&self, _owner: &Resource) -> VariantArray {
        let arr = VariantArray::new();
        for value in self.inner.texcoords.iter() {
            arr.push(util::convert_vector2d_to_gd(*value));
        }
        arr.into_shared()
    }

    #[method]
    pub fn get_size(&self) -> f32 {
        self.inner.size
    }
);
