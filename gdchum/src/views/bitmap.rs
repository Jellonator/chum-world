use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::bitmap;
use std::fs::File;
use std::io::BufReader;
use crate::util;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct BitmapView {
    pub inner: bitmap::Bitmap,
}

#[methods]
impl BitmapView {
    fn new(_owner: &Resource) -> Self {
        BitmapView { inner:  Default::default() }
    }

    impl_view!(BitmapView, bitmap::Bitmap, "BITMAP",
        |_builder: &ClassBuilder<Self>| {}
    );

    #[export]
    pub fn get_format(&self, _owner: &Resource) -> i64 {
        self.inner.get_data().get_format() as i64
    }

    #[export]
    pub fn get_palette_format(&self, _owner: &Resource) -> i64 {
        self.inner.get_data().get_format() as i64
    }

    #[export]
    fn export_to_png(&mut self, _owner: &Resource, path: String) {
        let mut buffer = File::create(&path).unwrap();
        self.inner.export_png(&mut buffer).unwrap();
    }

    /// Import BITMAP data from a file with the given format
    #[export]
    pub fn import_bitmap(
        &mut self,
        owner: &Resource,
        path: GodotString,
        formattype: i64,
        palettetype: i64,
    ) {
        let pathstr = path.to_string();
        let fh = File::open(&pathstr).unwrap();
        let image_format = bitmap::image::ImageFormat::from_path(&pathstr).unwrap();
        let mut buf_reader = BufReader::new(fh);
        let (bitmap, width, height) = bitmap::import_bitmap(&mut buf_reader, image_format).unwrap();
        let mut data =
            bitmap::BitmapFormat::new_empty(formattype as u8, palettetype as u8).unwrap();
        bitmap::compress_bitmap(&bitmap, &mut data, width, height).unwrap();
        self.inner = self.inner.with_bitmap(data, width, height);
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_structure(&self, _owner: &Resource) -> Variant {
        use libchum::structure::ChumStruct;
        let data = self.inner.get_struct().structure();
        util::struct_to_dict(&data).into_shared().to_variant()
    }

    #[export]
    pub fn import_structure(&mut self, owner: &Resource, data: Dictionary) {
        use libchum::structure::ChumStruct;
        let structure = util::dict_to_struct(&data);
        self.inner.import_struct(&bitmap::BitmapStruct::destructure(&structure).unwrap());
        owner.emit_signal("modified", &[]);
    }
}
