use gdnative::*;
use libchum;
use std::fs::File;

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
    fn load(&mut self, _owner: Resource, ngcpath: gdnative::GodotString, dgcpath: gdnative::GodotString) -> Result<(), i64> {
        let mut ngcfile = match File::open(ngcpath.to_string()) {
            Ok(x) => x,
            Err(_) => return Err(gdnative::GodotError::FileBadPath as i64)
        };
        let mut dgcfile = match File::open(dgcpath.to_string()) {
            Ok(x) => x,
            Err(_) => return Err(gdnative::GodotError::FileBadPath as i64)
        };
        match libchum::ChumArchive::read_chum_archive(&mut ngcfile, &mut dgcfile) {
            Ok(x) => {
                self.archive = Some(x);
                Ok(())
            }
            Err(_) => Err(gdnative::GodotError::FileCantOpen as i64)
        }
    }

    #[export]
    fn get_file_list(&self, _owner: Resource) -> gdnative::VariantArray {
        let mut arr = gdnative::VariantArray::new();
        if let Some(archive) = &self.archive {
            for file in archive.get_files() {
                let gdvariant = gdnative::Variant::from(file.get_name_id());
                arr.push(&gdvariant);
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
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();