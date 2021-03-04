use crate::chumfile::ChumFile;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::bitmap;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct BitmapView {
    pub inner: Option<bitmap::Bitmap>,
}

#[methods]
impl BitmapView {
    fn new(_owner: &Resource) -> Self {
        BitmapView { inner: None }
    }

    fn _register(_builder: &ClassBuilder<Self>) {}

    pub fn set_data(&mut self, data: bitmap::Bitmap) {
        self.inner = Some(data);
    }

    #[export]
    pub fn load(&mut self, _owner: &Resource, data: Instance<ChumFile, Shared>) {
        if let Err(e) = self.load_from(data) {
            display_err!("Error while loading BITMAP into view: {}", e);
        }
    }

    #[export]
    pub fn save(&self, _owner: &Resource, data: Instance<ChumFile, Shared>) {
        // use libchum::binary::ChumBinary;
        if let Some(bitmap) = self.inner.as_ref() {
            let mut v: Vec<u8> = Vec::new();
            unsafe { data.assume_safe() }
                .map_mut(|chumfile, _| {
                    bitmap.write_to(&mut v, chumfile.get_format()).unwrap();
                    chumfile.replace_data_with_vec(v);
                })
                .unwrap();
        }
    }

    pub fn load_from(&mut self, data: Instance<ChumFile, Shared>) -> Result<(), Box<dyn Error>> {
        // use libchum::binary::ChumBinary;
        unsafe {
            let data = data.assume_safe();
            self.inner = Some(data.map(|cfile, _| {
                cfile.borrow_data(|mut inner_data| {
                    bitmap::Bitmap::read_from(&mut inner_data, cfile.get_format())
                })
            })??);
        }
        Ok(())
    }

    #[export]
    pub fn get_format(&self, _owner: &Resource) -> i64 {
        self.inner.as_ref().unwrap().get_data().get_format() as i64
    }

    #[export]
    pub fn get_palette_format(&self, _owner: &Resource) -> i64 {
        self.inner.as_ref().unwrap().get_data().get_format() as i64
    }
    
    #[export]
    fn export_to_png(&mut self, _owner: &Resource, path: String) {
        let mut buffer = File::create(&path).unwrap();
        self.inner.as_ref().unwrap().export_png(&mut buffer).unwrap();
    }

    /// Import BITMAP data from a file with the given format
    #[export]
    pub fn import_bitmap(
        &mut self,
        _owner: &Resource,
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
        self.inner =
            Some(self.inner.as_ref().unwrap().with_bitmap(data, width, height));
    }
}
