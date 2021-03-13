use crate::util;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::lod;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct LodView {
    pub inner: lod::Lod,
}

#[methods]
impl LodView {
    fn new(_owner: &Resource) -> Self {
        LodView {
            inner: Default::default(),
        }
    }

    impl_view_node_resource!(LodView, lod::Lod, "LOD", |_builder: &ClassBuilder<Self>| {});

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

    #[export]
    pub fn get_resources(&self, _owner: TRef<Resource>) -> VariantArray<Shared> {
        let arr = VariantArray::new();
        for value in self.inner.skin_ids.iter() {
            arr.push(value.to_variant());
        }
        arr.into_shared()
    }

    #[export]
    pub fn set_resources(&mut self, owner: TRef<Resource>, value: VariantArray<Shared>) {
        let value = unsafe { value.assume_unique() };
        self.inner.skin_ids = value
            .iter()
            .map(|x| i32::from_variant(&x).unwrap())
            .collect();
        owner.emit_signal("modified", &[]);
    }

    /*#[export]
    pub fn get_animations(&self, _owner: TRef<Resource>) -> VariantArray<Shared> {
        let arr = VariantArray::new();
        for value in self.inner.anims.iter() {
            arr.push(value.to_variant());
        }
        arr.into_shared()
    }

    #[export]
    pub fn set_animations(&mut self, owner: TRef<Resource>, value: VariantArray<Shared>) {
        let value = unsafe { value.assume_safe() };
        self.inner.anims = value.iter().map(|x| i32::from_variant(x).unwrap()).collect();
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_sounds(&self, _owner: TRef<Resource>) -> VariantArray<Shared> {
        let arr = VariantArray::new();
        for value in self.inner.sounds.iter() {
            arr.push(value.to_variant());
        }
        arr.into_shared()
    }

    #[export]
    pub fn set_sounds(&mut self, owner: TRef<Resource>, value: VariantArray<Shared>) {
        let value = unsafe { value.assume_safe() };
        self.inner.sounds = value.iter().map(|x| i32::from_variant(x).unwrap()).collect();
        owner.emit_signal("modified", &[]);
    }*/
}
