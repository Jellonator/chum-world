use gdnative::*;
use libchum;
use std::fs::File;

pub mod chumfile;
pub mod reader;
pub mod bytedata;

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct ChumArchive {
    pub archive: Option<libchum::ChumArchive>
}

#[methods]
impl ChumArchive {
    fn _init(_owner: Resource) -> Self {
        ChumArchive {
            archive: None
        }
    }

    #[export]
    fn load(&mut self, _owner: Resource, ngcpath: gdnative::GodotString, dgcpath: gdnative::GodotString) -> i64 {
        let mut ngcfile = match File::open(ngcpath.to_string()) {
            Ok(x) => x,
            Err(_) => return gdnative::GodotError::FileBadPath as i64
        };
        let mut dgcfile = match File::open(dgcpath.to_string()) {
            Ok(x) => x,
            Err(_) => return gdnative::GodotError::FileBadPath as i64
        };
        match libchum::ChumArchive::read_chum_archive(&mut ngcfile, &mut dgcfile) {
            Ok(x) => {
                self.archive = Some(x);
                0
            }
            Err(_) => gdnative::GodotError::FileCantOpen as i64
        }
    }

    #[export]
    fn get_file_list(&self, _owner: Resource) -> gdnative::VariantArray {
        let mut arr = gdnative::VariantArray::new();
        if let Some(archive) = &self.archive {
            for file in archive.get_files() {
                let f = Instance::<chumfile::ChumFile>::new();
                f.map_mut(|script, _res| {
                    script.read_from_chumfile(file);
                }).unwrap();
                arr.push(&Variant::from(f.base().new_ref()));
            }
        }
        return arr
    }

    #[export]
    fn do_thing(&self, _owner: Resource) {
        godot_print!("Hello, World!");
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<ChumArchive>();
    handle.add_class::<chumfile::ChumFile>();
    handle.add_class::<bytedata::ByteData>();
    handle.add_class::<reader::ChumReader>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();