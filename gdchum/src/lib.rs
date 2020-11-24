use gdnative::prelude::*;
use gdnative::api::Resource;
use libchum;
use std::fs::File;
use std::collections::HashMap;

#[macro_use]
pub mod util;
pub mod bytedata;
pub mod chumfile;
pub mod names;
pub mod reader;
pub mod views;

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct ChumArchive {
    pub archive: libchum::ChumArchive,
    pub files: HashMap<i32, Instance<chumfile::ChumFile, Shared>>
}

#[methods]
impl ChumArchive {
    fn new(_owner: &Resource) -> Self {
        ChumArchive {
            archive: libchum::ChumArchive::default(),
            files: HashMap::new()
        }
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
        match self.archive.write_chum_archive(&mut ngcfile, &mut dgcfile)
        {
            Ok(_) => 0,
            Err(_) => GodotError::FileCantWrite as i64,
        }
    }

    #[export]
    fn load(
        &mut self,
        owner: TRef<Resource, Shared>,
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
                self.files.clear();
                for file in x.get_files() {
                    let f = Instance::<chumfile::ChumFile,Unique>::new();
                    let id = f.map_mut(|script, _res| {
                        let instance = Instance::from_base(owner.claim()).unwrap();
                        script.read_from_chumfile(file, x.get_format(), instance);
                        script.get_hash_id_ownerless()
                    })
                    .unwrap();
                    self.files.insert(id, f.into_shared());
                }
                self.archive = x;
                0
            }
            Err(e) => {
                display_err!("Error loading archive: {}, {}\n{}", ngcpath, dgcpath, e);
                GodotError::FileCantOpen as i64
            }
        }
    }

    #[export]
    fn get_file_list(&self, _owner: &Resource) -> VariantArray<Unique> {
        let arr = VariantArray::<Unique>::new();
        for (_id, file) in self.files.iter() {
            arr.push(file);
        }
        return arr;
    }

    #[export]
    pub fn get_file_from_hash(
        &self,
        _owner: &Resource,
        id: i32,
    ) -> Option<Instance<chumfile::ChumFile, Shared>> {
        self.files.get(&id).map(|x| x.clone())
    }

    #[export]
    fn do_thing(&self, _owner: &Resource) {
        godot_print!("Hello, World!");
    }

    #[export]
    pub fn maybe_get_name_from_hash(&self, _owner: &Resource, id: i32) -> GodotString {
        if let Some(name) = self.archive.get_name_from_id(id) {
            GodotString::from_str(&name)
        } else {
            if let Some(name) = names::NAMES.get(&id) {
                GodotString::from_str(&name)
            } else {
                GodotString::from_str(&id.to_string())
            }
        }
    }

    pub fn maybe_get_name_from_hash_str(&self, id: i32) -> String {
        if let Some(name) = self.archive.get_name_from_id(id) {
            name.to_owned()
        } else {
            if let Some(name) = names::NAMES.get(&id) {
                name.to_string()
            } else {
                // id.to_string()
                format!("0x{:08X}", id)
            }
        }
    }

    #[export]
    pub fn register_name(&mut self, _owner: &Resource, name: GodotString) -> bool {
        let utf8 = name.to_utf8();
        let name_str = utf8.as_str();
        match self.archive.add_name(name_str) {
            Ok(did_insert) => {
                if did_insert {
                    display_info!("String \"{}\" has been registered in the name database.", name);
                }
                true
            },
            Err(_) => {
                display_err!(
                    "Could not insert the string \"{}\" into the name database due to a name collision with \"{}\".",
                    name, self.maybe_get_name_from_hash_str(libchum::hash_name(name_str))
                );
                false
            }
        }
    }

    #[export]
    pub fn get_hash_of_name(&self, _owner: &Resource, name: GodotString) -> i32 {
        libchum::hash_name(name.to_utf8().as_str())
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<ChumArchive>();
    handle.add_class::<chumfile::ChumFile>();
    handle.add_class::<bytedata::ByteData>();
    handle.add_class::<reader::ChumReader>();
    handle.add_class::<util::StructGenerator>();
    views::init(handle);
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
