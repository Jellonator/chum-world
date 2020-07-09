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
    data: Resource,
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
            .with_setter(Self::set_data)
            .done();
        builder
            .add_property("name")
            .with_mut_getter(Self::get_name)
            .with_setter(Self::set_name)
            .done();
        builder
            .add_property("type")
            .with_mut_getter(Self::get_type)
            .with_setter(Self::set_type)
            .done();
        builder
            .add_property("subtype")
            .with_mut_getter(Self::get_subtype)
            .with_setter(Self::set_subtype)
            .done();
    }

    #[export]
    pub fn get_data(&mut self, _owner: Resource) -> Resource {
        self.data.new_ref()
    }

    #[export]
    pub fn set_data(&mut self, _owner: Resource, data: Resource) {
        self.data = data;
    }

    #[export]
    pub fn get_name(&mut self, _owner: Resource) -> GodotString {
        self.nameid.new_ref()
    }

    #[export]
    pub fn set_name(&mut self, _owner: Resource, value: GodotString) {
        self.nameid = value;
    }

    #[export]
    pub fn get_type(&mut self, _owner: Resource) -> GodotString {
        self.typeid.new_ref()
    }

    #[export]
    pub fn set_type(&mut self, _owner: Resource, value: GodotString) {
        self.typeid = value;
    }

    #[export]
    pub fn get_subtype(&mut self, _owner: Resource) -> GodotString {
        self.subtypeid.new_ref()
    }

    #[export]
    pub fn set_subtype(&mut self, _owner: Resource, value: GodotString) {
        self.subtypeid = value;
    }

    fn export_to_bitmap(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        self.get_bytedata()
            .map(|bytedata, _| {
                let bitmap =
                    match reader::bitmap::Bitmap::read_data(bytedata.get_data(), self.format) {
                        Ok(x) => x,
                        Err(err) => {
                            panic!("BITMAP file invalid: {}", err);
                        }
                    };
                bitmap.export(&mut buffer).unwrap();
            })
            .unwrap();
    }

    fn export_to_binary(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        self.get_bytedata()
            .map(|bytedata, _| {
                buffer.write_all(bytedata.get_data()).unwrap();
            })
            .unwrap();
    }

    fn export_mesh_to_obj(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        self.get_bytedata()
            .map(|bytedata, _| {
                let tmesh = match reader::tmesh::TMesh::read_data(bytedata.get_data(), self.format)
                {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("MESH file invalid: {}", err);
                    }
                };
                tmesh.export(&mut buffer).unwrap();
            })
            .unwrap();
    }

    fn export_surface_to_obj(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        self.get_bytedata()
            .map(|bytedata, _| {
                let surf = match reader::surface::SurfaceObject::read_data(
                    bytedata.get_data(),
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
            })
            .unwrap();
    }

    fn export_to_txt(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        self.get_bytedata()
            .map(|bytedata, _| {
                buffer.write_all(&bytedata.get_data()[4..]).unwrap();
            })
            .unwrap();
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
        let f = Instance::<ByteData>::new();
        self.format = fmt;
        self.parent = Some(parent);
        self.data = f
            .map_mut(|script, res| {
                script.set_data(file.get_data().to_vec());
                res
            })
            .unwrap();
    }

    fn _init(_owner: Resource) -> Self {
        let f = Instance::<ByteData>::new();
        let data = f.into_base();
        ChumFile {
            data: data,
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

    pub fn get_bytedata<'a>(&'a self) -> Instance<ByteData> {
        Instance::<ByteData>::try_from_base(self.data.new_ref()).unwrap()
    }
}
