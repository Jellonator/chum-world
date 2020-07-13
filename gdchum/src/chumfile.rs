use crate::bytedata::ByteData;
use crate::ChumArchive;
use gdnative::*;
use libchum::{self, export::ChumExport, reader};
use std::fs::File;
use std::io::Write;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct ChumFile {
    nameid: GodotString,
    typeid: GodotString,
    subtypeid: GodotString,
    namestr: String,
    typestr: String,
    subtypestr: String,
    parent: Option<Resource>,
    format: libchum::format::TotemFormat,
}

// Direct copy of res://Gui/EditorList.gd
const EXPORT_ID_BIN: i64 = 0;
const EXPORT_ID_TEXT: i64 = 1;
const EXPORT_ID_MODEL: i64 = 2;
const EXPORT_ID_TEXTURE: i64 = 3;

#[methods]
impl ChumFile {
    fn _register(builder: &init::ClassBuilder<Self>) {
        builder
            .add_property("data")
            .with_mut_getter(Self::get_data)
            .done();
        builder
            .add_property("name")
            .with_mut_getter(Self::get_name)
            .done();
        builder
            .add_property("type")
            .with_mut_getter(Self::get_type)
            .done();
        builder
            .add_property("subtype")
            .with_mut_getter(Self::get_subtype)
            .done();
    }

    #[export]
    pub fn get_hash_id(&mut self, _owner: Resource) -> i32 {
        libchum::hash_name(&self.namestr)
    }

    #[export]
    pub fn get_data(&mut self, _owner: Resource) -> Resource {
        self.get_data_as_bytedata().into_base()
    }

    #[export]
    pub fn get_name(&mut self, _owner: Resource) -> GodotString {
        self.nameid.new_ref()
    }

    #[export]
    pub fn get_type(&mut self, _owner: Resource) -> GodotString {
        self.typeid.new_ref()
    }

    #[export]
    pub fn get_subtype(&mut self, _owner: Resource) -> GodotString {
        self.subtypeid.new_ref()
    }

    fn export_to_bitmap(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        let bitmap =
            match reader::bitmap::Bitmap::read_data(&mut self.get_data_as_vec(), self.format) {
                Ok(x) => x,
                Err(err) => {
                    panic!("BITMAP file invalid: {}", err);
                }
            };
        bitmap.export(&mut buffer).unwrap();
    }

    fn export_to_binary(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        buffer.write_all(&mut self.get_data_as_vec()).unwrap();
    }

    fn export_mesh_to_obj(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        let tmesh = match reader::tmesh::TMesh::read_data(&mut self.get_data_as_vec(), self.format)
        {
            Ok(x) => x,
            Err(err) => {
                panic!("MESH file invalid: {}", err);
            }
        };
        tmesh.export(&mut buffer).unwrap();
    }

    fn export_surface_to_obj(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        let surf = match reader::surface::SurfaceObject::read_data(
            &mut self.get_data_as_vec(),
            self.format,
        ) {
            Ok(x) => x,
            Err(err) => {
                panic!("MESH file invalid: {}", err);
            }
        };
        surf.begin_export(reader::surface::SurfaceExportMode::Mesh(
            reader::surface::SurfaceGenMode::BezierInterp(10),
        ))
        .export(&mut buffer)
        .unwrap();
    }

    fn export_to_txt(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        buffer.write_all(&self.get_data_as_vec()[4..]).unwrap();
    }

    #[export]
    pub fn export_to(&mut self, _owner: Resource, export_type: i64, path: GodotString) {
        let pathstr: String = format!("{}", path);
        match export_type {
            EXPORT_ID_BIN => self.export_to_binary(&pathstr),
            EXPORT_ID_TEXT => self.export_to_txt(&pathstr),
            EXPORT_ID_TEXTURE => self.export_to_bitmap(&pathstr),
            EXPORT_ID_MODEL => match &self.typestr.as_str() {
                &"MESH" => self.export_mesh_to_obj(&pathstr),
                &"SURFACE" => self.export_surface_to_obj(&pathstr),
                other => {
                    panic!("Unexpected type for OBJ export {}", other);
                }
            },
            other => {
                panic!("Unexpected export type {}", other);
            }
        };
    }

    #[export]
    pub fn replace_with_string(&mut self, _owner: Resource, stringdata: GodotString) {
        let mut data = vec![0; 4];
        let realstr = format!("{}", stringdata);
        godot_print!("A:");
        // TotemTech uses \r\n for newlines, but Godot uses \n.
        // Godot also converts all \r\n to \n when setting text.
        // So, all \n must be replaced with \r\n.
        // The easiest way to accomplish this is just to push '\r\n' when encountering '\n',
        // and ignoring '\r' just in case.
        for c in realstr.bytes() {
            match c {
                0x0A => {
                    data.push(0x0A);
                    data.push(0x0D);
                }
                0x0D => {}
                other => data.push(other),
            }
        }
        godot_print!("B:");
        // Set first four bytes to the data's size
        // Fortunately, rust lets you write to slices
        let size = data.len() - 4;
        self.format
            .write_u32(&mut &mut data[0..4], size as u32)
            .unwrap();
        godot_print!("C:");
        // Actually set the data
        self.replace_data_with_vec(data);
        godot_print!("D:");
    }

    pub fn get_name_str(&self) -> &str {
        &self.namestr
    }

    pub fn get_type_str(&self) -> &str {
        &self.typestr
    }

    pub fn get_subtype_str(&self) -> &str {
        &self.subtypestr
    }

    pub fn get_name_hash(&self) -> i32 {
        libchum::hash_name(&self.namestr)
    }

    pub fn get_archive_instance(&self) -> Instance<ChumArchive> {
        Instance::try_from_base(self.parent.as_ref().unwrap().new_ref()).unwrap()
    }

    pub fn read_from_chumfile(
        &mut self,
        file: &libchum::ChumFile,
        fmt: libchum::format::TotemFormat,
        parent: Resource,
    ) {
        self.nameid = GodotString::from_str(file.get_name_id());
        self.typeid = GodotString::from_str(file.get_type_id());
        self.subtypeid = GodotString::from_str(file.get_subtype_id());
        self.namestr = file.get_name_id().into();
        self.typestr = file.get_type_id().into();
        self.subtypestr = file.get_subtype_id().into();
        self.format = fmt;
        self.parent = Some(parent);
    }

    fn _init(_owner: Resource) -> Self {
        ChumFile {
            nameid: GodotString::new(),
            typeid: GodotString::new(),
            subtypeid: GodotString::new(),
            namestr: String::new(),
            typestr: String::new(),
            subtypestr: String::new(),
            format: libchum::format::TotemFormat::NGC,
            parent: None,
        }
    }

    pub fn get_format(&self) -> libchum::format::TotemFormat {
        self.format
    }

    pub fn get_data_as_bytedata(&self) -> Instance<ByteData> {
        self.get_archive_instance()
            .map(|archive, _| {
                let file = archive
                    .archive
                    .as_ref()
                    .unwrap()
                    .get_file_from_name(&self.namestr)
                    .unwrap();
                let f = Instance::<ByteData>::new();
                f.map_mut(|script, res| {
                    script.set_data(file.get_data().to_vec());
                    res
                })
                .unwrap();
                f
            })
            .unwrap()
    }

    pub fn get_data_as_vec(&self) -> Vec<u8> {
        self.get_archive_instance()
            .map(|archive, _| {
                let file = archive
                    .archive
                    .as_ref()
                    .unwrap()
                    .get_file_from_name(&self.namestr)
                    .unwrap();
                file.get_data().to_owned()
            })
            .unwrap()
    }

    pub fn replace_data_with_vec(&mut self, data: Vec<u8>) {
        self.get_archive_instance()
            .map_mut(|archive, _| {
                archive
                    .archive
                    .as_mut()
                    .unwrap()
                    .get_file_from_name_mut(&self.namestr)
                    .unwrap()
                    .replace_data(data);
                godot_print!("WROTE TO ARCHIVE");
            })
            .unwrap();
    }
}
