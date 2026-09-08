use crate::util;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::bitmap;
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct BitmapView {
    pub inner: bitmap::Bitmap,
}

impl_view_base!(
    BitmapView,
    bitmap::Bitmap,
    "BITMAP",
    (|_builder: &ClassBuilder<Self>| {}),

    fn new(_owner: &Resource) -> Self {
        BitmapView {
            inner: Default::default(),
        }
    }

    #[method]
    pub fn get_format(&self) -> i64 {
        self.inner.get_data().get_format() as i64
    }

    #[method]
    pub fn get_palette_format(&self) -> i64 {
        self.inner.get_data().get_format() as i64
    }

    #[method]
    fn export_to_png(&mut self, path: GodotString) -> () {
        let mut buffer = File::create(&path.to_string()).unwrap();
        self.inner.export_png(&mut buffer).unwrap();
    }
);
