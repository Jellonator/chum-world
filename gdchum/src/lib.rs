use gdnative::prelude::*;
use gdnative::api::Resource;
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
    fn new(_owner: &Resource) -> Self {
        ChumArchive { archive: None }
    }

    #[export]
    fn save(
        &mut self,
        _owner: &Resource,
        ngcpath: GodotString,
        dgcpath: GodotString,
    ) -> i64 {
        let mut ngcfile = match File::create(ngcpath.to_string()) {
            Ok(x) => x,
            Err(_) => return GodotError::FileCantOpen as i64,
        };
        let mut dgcfile = match File::create(dgcpath.to_string()) {
            Ok(x) => x,
            Err(_) => return GodotError::FileCantOpen as i64,
        };
        match self
            .archive
            .as_ref()
            .unwrap()
            .write_chum_archive(&mut ngcfile, &mut dgcfile)
        {
            Ok(_) => 0,
            Err(_) => GodotError::FileCantWrite as i64,
        }
    }

    #[export]
    fn load(
        &mut self,
        _owner: &Resource,
        ngcpath: GodotString,
        dgcpath: GodotString,
        fmt: GodotString,
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
                return GodotError::InvalidParameter as i64;
            }
        };
        let mut ngcfile = match File::open(ngcpath.to_string()) {
            Ok(x) => x,
            Err(e) => {
                display_err!("Error loading archive: {}, {}\n{}", ngcpath, dgcpath, e);
                return GodotError::FileBadPath as i64;
            }
        };
        let mut dgcfile = match File::open(dgcpath.to_string()) {
            Ok(x) => x,
            Err(e) => {
                display_err!("Error loading archive: {}, {}\n{}", ngcpath, dgcpath, e);
                return GodotError::FileBadPath as i64;
            }
        };
        match libchum::ChumArchive::read_chum_archive(&mut ngcfile, &mut dgcfile, format) {
            Ok(x) => {
                self.archive = Some(x);
                0
            }
            Err(e) => {
                display_err!("Error loading archive: {}, {}\n{}", ngcpath, dgcpath, e);
                GodotError::FileCantOpen as i64
            }
        }
    }

    #[export]
    fn get_file_list(&self, owner: TRef<Resource, Shared>) -> VariantArray<Unique> {
        let instance = Instance::from_base(owner.claim()).unwrap();
        let arr = VariantArray::<Unique>::new();
        if let Some(archive) = &self.archive {
            for file in archive.get_files() {
                let f = Instance::<chumfile::ChumFile,Unique>::new();
                f.map_mut(|script, _res| {
                    script.read_from_chumfile(file, archive.get_format(), instance.clone());
                })
                .unwrap();
                arr.push(f);
            }
        }
        return arr;
    }

    #[export]
    pub fn get_file_from_hash(
        &self,
        owner: TRef<Resource, Shared>,
        id: i32,
    ) -> Option<Instance<chumfile::ChumFile, Shared>> {
        let instance = Instance::from_base(owner.claim()).unwrap();
        if let Some(archive) = &self.archive {
            if let Some(file) = archive.get_file_from_hash(id) {
                let f = Instance::<chumfile::ChumFile, Unique>::new();
                f.map_mut(|script, _res| {
                    script.read_from_chumfile(file, archive.get_format(), instance.clone());
                })
                .unwrap();
                Some(f.into_shared())
            } else {
                None
            }
        } else {
            None
        }
    }

    #[export]
    fn do_thing(&self, _owner: &Resource) {
        godot_print!("Hello, World!");
    }

    #[export]
    pub fn maybe_get_name_from_hash(&self, _owner: &Resource, id: i32) -> GodotString {
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

fn init(handle: InitHandle) {
    handle.add_class::<ChumArchive>();
    handle.add_class::<chumfile::ChumFile>();
    handle.add_class::<bytedata::ByteData>();
    handle.add_class::<reader::ChumReader>();
    handle.add_class::<util::StructGenerator>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
