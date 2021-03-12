use libchum::reader::spline::*;
use gdnative::prelude::*;
use gdnative::api::Resource;
use crate::util;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct SplineView {
    pub inner: Spline,
}

#[methods]
impl SplineView {
    fn new(_owner: &Resource) -> Self {
        SplineView { inner: Default::default() }
    }

    impl_view_node_resource!(SplineView, Spline, "SPLINE",
        |_builder: &ClassBuilder<Self>| {
        }
    );

    #[export]
    pub fn get_structure(&self, _owner: &Resource) -> Variant {
        return Variant::new();
    }

    #[export]
    pub fn import_structure(&mut self, _owner: &Resource, _data: Dictionary) {
        // do nothing
    }

    #[export]
    pub fn get_precomputed_vertices(&self, _owner: &Resource) -> Vector3Array {
        Vector3Array::from_vec(self.inner.get_vertices_as_vec())
    }

    #[export]
    pub fn get_stops(&self, _owner: &Resource) -> Vector3Array {
        Vector3Array::from_vec(self.inner.get_section_stops_as_vec())
    }
}
