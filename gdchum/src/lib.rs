use gdnative::*;
use libchum;
use std::fs::File;

#[macro_use]
pub mod util;
pub mod bytedata;
pub mod chumfile;
pub mod names;
pub mod reader;

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct ChumArchive {
    pub archive: Option<libchum::ChumArchive>,
}

#[methods]
impl ChumArchive {
    fn _init(_owner: Resource) -> Self {
        ChumArchive { archive: None }
    }

    #[export]
    fn save(
        &mut self,
        _owner: Resource,
        ngcpath: gdnative::GodotString,
        dgcpath: gdnative::GodotString,
    ) -> i64 {
        let mut ngcfile = match File::create(ngcpath.to_string()) {
            Ok(x) => x,
            Err(_) => return gdnative::GodotError::FileCantOpen as i64,
        };
        let mut dgcfile = match File::create(dgcpath.to_string()) {
            Ok(x) => x,
            Err(_) => return gdnative::GodotError::FileCantOpen as i64,
        };
        match self
            .archive
            .as_ref()
            .unwrap()
            .write_chum_archive(&mut ngcfile, &mut dgcfile)
        {
            Ok(_) => 0,
            Err(_) => gdnative::GodotError::FileCantWrite as i64,
        }
    }

    #[export]
    fn load(
        &mut self,
        _owner: Resource,
        ngcpath: gdnative::GodotString,
        dgcpath: gdnative::GodotString,
        fmt: gdnative::GodotString,
    ) -> i64 {
        let format = match fmt.to_string().as_ref() {
            "PS2" => libchum::format::TotemFormat::PS2,
            "NGC" => libchum::format::TotemFormat::NGC,
            a => {
                display_err!(
                    "Error loading archive: {}, {}\nInvalid format input {}",
                    ngcpath,
                    dgcpath,
                    a
                );
                return gdnative::GodotError::InvalidParameter as i64;
            }
        };
        let mut ngcfile = match File::open(ngcpath.to_string()) {
            Ok(x) => x,
            Err(e) => {
                display_err!("Error loading archive: {}, {}\n{}", ngcpath, dgcpath, e);
                return gdnative::GodotError::FileBadPath as i64;
            }
        };
        let mut dgcfile = match File::open(dgcpath.to_string()) {
            Ok(x) => x,
            Err(e) => {
                display_err!("Error loading archive: {}, {}\n{}", ngcpath, dgcpath, e);
                return gdnative::GodotError::FileBadPath as i64;
            }
        };
        match libchum::ChumArchive::read_chum_archive(&mut ngcfile, &mut dgcfile, format) {
            Ok(x) => {
                self.archive = Some(x);
                0
            }
            Err(e) => {
                display_err!("Error loading archive: {}, {}\n{}", ngcpath, dgcpath, e);
                gdnative::GodotError::FileCantOpen as i64
            }
        }
    }

    #[export]
    fn get_file_list(&self, owner: Resource) -> gdnative::VariantArray {
        let mut arr = gdnative::VariantArray::new();
        if let Some(archive) = &self.archive {
            for file in archive.get_files() {
                let f = Instance::<chumfile::ChumFile>::new();
                f.map_mut(|script, _res| {
                    script.read_from_chumfile(file, archive.get_format(), owner.new_ref());
                })
                .unwrap();
                arr.push(&Variant::from(f.base().new_ref()));
            }
        }
        return arr;
    }

    #[export]
    pub fn get_file_from_hash(
        &self,
        owner: Resource,
        id: i32,
    ) -> Option<Instance<chumfile::ChumFile>> {
        if let Some(archive) = &self.archive {
            if let Some(file) = archive.get_file_from_hash(id) {
                let f = Instance::<chumfile::ChumFile>::new();
                f.map_mut(|script, _res| {
                    script.read_from_chumfile(file, archive.get_format(), owner.new_ref());
                })
                .unwrap();
                Some(f)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[export]
    fn do_thing(&self, _owner: Resource) {
        godot_print!("Hello, World!");
    }

    #[export]
    pub fn maybe_get_name_from_hash(&self, _owner: Resource, id: i32) -> GodotString {
        if let Some(archive) = &self.archive {
            if let Some(name) = archive.get_name_from_id(id) {
                GodotString::from_str(&name)
            } else {
                if let Some(name) = names::NAMES.get(&id) {
                    GodotString::from_str(&name)
                } else {
                    GodotString::from_str(&id.to_string())
                }
            }
        } else {
            if let Some(name) = names::NAMES.get(&id) {
                GodotString::from_str(&name)
            } else {
                GodotString::from_str(&id.to_string())
            }
        }
    }

    pub fn maybe_get_name_from_hash_str(&self, id: i32) -> String {
        if let Some(archive) = &self.archive {
            if let Some(name) = archive.get_name_from_id(id) {
                name.to_owned()
            } else {
                if let Some(name) = names::NAMES.get(&id) {
                    name.to_string()
                } else {
                    // id.to_string()
                    format!("0x{:08X}", id)
                }
            }
        } else {
            if let Some(name) = names::NAMES.get(&id) {
                name.to_string()
            } else {
                // id.to_string()
                format!("0x{:08X}", id)
            }
        }
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
