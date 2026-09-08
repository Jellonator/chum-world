use crate::util;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::spline::*;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct SplineView {
    pub inner: Spline,
}

impl_view_node_resource!(
    SplineView,
    Spline,
    "SPLINE",
    (|_builder: &ClassBuilder<Self>| {}),

    fn new(_owner: &Resource) -> Self {
        SplineView {
            inner: Default::default(),
        }
    }


    #[method]
    pub fn get_structure(&self) -> Variant {
        return Variant::nil();
    }

    #[method]
    pub fn import_structure(&mut self, _data: Dictionary) -> () {
        // do nothing
    }

    #[method]
    pub fn get_precomputed_vertices(&self) -> Vector3Array {
        Vector3Array::from_vec(
            self.inner
            .get_vertices_as_vec()
            .iter()
            .map(|x| util::convert_vector3d_to_gd(*x))
            .collect())
    }

    #[method]
    pub fn get_stops(&self) -> Vector3Array {
        Vector3Array::from_vec(
            self.inner
            .get_section_stops_as_vec()
            .iter()
            .map(|x| util::convert_vector3d_to_gd(*x))
            .collect())
    }
);
