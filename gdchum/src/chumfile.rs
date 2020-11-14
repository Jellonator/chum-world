use crate::bytedata::ByteData;
use crate::util;
use crate::ChumArchive;
use gdnative::prelude::*;
use gdnative::api::Resource;
use libchum::{
    self, reader,
    scene::{self, collada},
    structure::ChumStruct,
};
use std::fs::File;
use std::io::{BufReader, Write};

pub fn get_filename(name: &str) -> &str {
    match name.rfind('>') {
        Some(pos) => &name[pos + 1..],
        None => name,
    }
}

pub fn get_basename(name: &str) -> &str {
    let name = get_filename(name);
    match name.find('.') {
        Some(pos) => &name[..pos],
        None => name,
    }
}

/// A File resource derived from a ChumArchive.
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
    parent: Option<Instance<ChumArchive,Shared>>,
    format: libchum::format::TotemFormat,
}

// Direct copy of res://Gui/EditorList.gd
const EXPORT_ID_BIN: i64 = 0;
const EXPORT_ID_TEXT: i64 = 1;
const EXPORT_ID_MODEL: i64 = 2;
const EXPORT_ID_TEXTURE: i64 = 3;
const EXPORT_ID_COLLADA: i64 = 4;

#[methods]
impl ChumFile {
    fn new(_owner: &Resource) -> Self {
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

    fn _register(builder: &ClassBuilder<Self>) {
        // builder.add_property("data").with_getter(Self::get_data).done();
        builder
            .add_property("data")
            .with_getter(Self::get_data)
            .done();
        builder
            .add_property("name")
            .with_getter(Self::get_name)
            .done();
        builder
            .add_property("type")
            .with_getter(Self::get_type)
            .done();
        builder
            .add_property("subtype")
            .with_getter(Self::get_subtype)
            .done();
    }

    #[export]
    pub fn get_archive(&self, _owner: &Resource) -> Instance<ChumArchive,Shared> {
        self.parent.clone().unwrap()
    }

    #[export]
    pub fn get_data(&self, _owner: TRef<Resource>) -> Instance<ByteData,Shared> {
        self.get_data_as_bytedata()
    }

    #[export]
    pub fn get_name(&self, _owner: TRef<Resource>) -> GodotString {
        self.nameid.new_ref()
    }

    #[export]
    pub fn get_type(&self, _owner: TRef<Resource>) -> GodotString {
        self.typeid.new_ref()
    }

    #[export]
    pub fn get_subtype(&self, _owner: TRef<Resource>) -> GodotString {
        self.subtypeid.new_ref()
    }

    /// Get the hash ID for this file
    #[export]
    pub fn get_hash_id(&self, _owner: &Resource) -> i32 {
        self.get_hash_id_ownerless()
    }

    /// Get the hadh ID for this file without an owner
    pub fn get_hash_id_ownerless(&self) -> i32 {
        libchum::hash_name(&self.namestr)
    }

    /// Get this file's name
    pub fn get_name_str(&self) -> &str {
        &self.namestr
    }

    /// Get this file's type
    pub fn get_type_str(&self) -> &str {
        &self.typestr
    }

    /// Get this file's subtype
    pub fn get_subtype_str(&self) -> &str {
        &self.subtypestr
    }

    /// Get an Instance of this file's containing ChumArchive
    pub fn get_archive_instance(&self) -> Instance<ChumArchive,Shared> {
        self.parent.clone().unwrap()
    }

    /// Read a file from the given ChumArchive
    pub fn read_from_chumfile(
        &mut self,
        file: &libchum::ChumFile,
        fmt: libchum::format::TotemFormat,
        parent: Instance<ChumArchive,Shared>,
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

    /// Get this file's format
    pub fn get_format(&self) -> libchum::format::TotemFormat {
        self.format
    }

    /// Read this file's data as a ByteData instance
    pub fn get_data_as_bytedata(&self) -> Instance<ByteData,Shared> {
        let archive_instance = self.get_archive_instance();
        unsafe {archive_instance.assume_safe()}.map(|archive,_| {
            let file = archive
                .archive
                .as_ref()
                .unwrap()
                .get_file_from_name(&self.namestr)
                .unwrap();
            let f = Instance::<ByteData, Unique>::new();
            f.map_mut(|script, _| {
                script.set_data(file.get_data().to_vec());
            })
            .unwrap();
            f.into_shared()
        })
        .unwrap()
    }

    /// Get this file's data as a Vec<u8>
    pub fn get_data_as_vec(&self) -> Vec<u8> {
        let archive_instance = self.get_archive_instance();
        unsafe {archive_instance.assume_safe()}.map(|archive, _| {
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

    /// Replace this file's data with the given Vec<u8>
    pub fn replace_data_with_vec(&mut self, data: Vec<u8>) {
        let archive_instance = self.get_archive_instance();
        unsafe {archive_instance.assume_safe()}.map_mut(|archive, _| {
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

    ///////////////////////////////////////////////////////////////////////////
    // EXPORT DATA                                                           //
    ///////////////////////////////////////////////////////////////////////////

    /// Export a BITMAP file as a .png
    fn export_bitmap_to_png(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        let bitmap =
            match reader::bitmap::Bitmap::read_data(&mut self.get_data_as_vec(), self.format) {
                Ok(x) => x,
                Err(err) => {
                    panic!("BITMAP file invalid: {}", err);
                }
            };
        bitmap.export_png(&mut buffer).unwrap();
    }

    /// Export this file's raw data
    fn export_to_binary(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        buffer.write_all(&mut self.get_data_as_vec()).unwrap();
    }

    /// Export a MESH file as a .obj
    fn export_mesh_to_obj(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        let tmesh = match reader::tmesh::TMesh::read_data(&mut self.get_data_as_vec(), self.format)
        {
            Ok(x) => x,
            Err(err) => {
                panic!("MESH file invalid: {}", err);
            }
        };
        tmesh.export_obj(&mut buffer).unwrap();
    }

    fn export_mesh_to_collada(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        let tmesh = match reader::tmesh::TMesh::read_data(&mut self.get_data_as_vec(), self.format)
        {
            Ok(x) => x,
            Err(err) => {
                panic!("MESH file invalid: {}", err);
            }
        };
        let mut scene = scene::Scene::new_empty();
        scene.add_trimesh(tmesh.create_scene_mesh(get_basename(&self.namestr).to_owned()));
        collada::scene_to_writer_dae(&scene, &mut buffer).unwrap();
    }

    fn export_skin_to_collada(&mut self, path: &str, merge_models: bool) {
        let mut buffer = File::create(path).unwrap();
        let skin = match reader::skin::Skin::read_data(&mut self.get_data_as_vec(), self.format) {
            Ok(x) => x,
            Err(err) => {
                panic!("MESH file invalid: {}", err);
            }
        };
        let mut scene = scene::Scene::new_empty();
        let archiveinstance = self.get_archive_instance();
        unsafe{ archiveinstance.assume_safe() }.map(|archive, res| {
            let names: Vec<String> = skin
                .vertex_groups
                .iter()
                .map(|group| archive.maybe_get_name_from_hash_str(group.group_id))
                .collect();
            for meshid in skin.meshes.iter() {
                if let Some(meshfile) = archive.get_file_from_hash(res, *meshid) {
                    unsafe { meshfile.assume_safe() }.map(|meshscript,_| match meshscript.typestr.as_str() {
                        "MESH" => {
                            let mesh = match reader::tmesh::TMesh::read_data(
                                &mut meshscript.get_data_as_vec(),
                                self.format,
                            ) {
                                Ok(x) => x,
                                Err(err) => {
                                    panic!("MESH file invalid: {}", err);
                                }
                            };
                            let mut trimesh = mesh.create_scene_mesh(
                                get_basename(&meshscript.namestr).to_owned(),
                            );
                            trimesh.skin = Some(skin.generate_scene_skin_for_mesh(
                                names.as_slice(),
                                *meshid,
                                mesh.vertices.len(),
                            ));
                            scene.add_trimesh(trimesh);
                        }
                        _ => {}
                    })
                    .unwrap();
                } else {
                    godot_warn!("Mesh {} does not exist!", meshid);
                }
            }
        })
        .unwrap();
        if merge_models {
            let mut data = Vec::new();
            data.append(&mut scene.trimeshes);
            if let Some(mut realmodel) = scene::merge_mesh_vec(data) {
                scene::try_determine_group_transforms(&mut realmodel);
                scene.add_trimesh(realmodel);
            }
        } else {
            for mesh in scene.trimeshes.iter_mut() {
                scene::try_determine_group_transforms(mesh);
            }
        }
        collada::scene_to_writer_dae(&scene, &mut buffer).unwrap();
    }

    /// Export a SURFACE file as a .obj
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
        .export_obj(&mut buffer)
        .unwrap();
    }

    /// Export a TXT file as a .obj
    fn export_txt_to_txt(&mut self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        buffer.write_all(&self.get_data_as_vec()[4..]).unwrap();
    }

    /// Export file data with the given export type to the given path
    #[export]
    pub fn export_to(&mut self, _owner: &Resource, export_type: i64, path: GodotString) {
        let pathstr: String = format!("{}", path);
        match export_type {
            EXPORT_ID_BIN => self.export_to_binary(&pathstr),
            EXPORT_ID_TEXT => self.export_txt_to_txt(&pathstr),
            EXPORT_ID_TEXTURE => self.export_bitmap_to_png(&pathstr),
            EXPORT_ID_MODEL => match &self.typestr.as_str() {
                &"MESH" => self.export_mesh_to_obj(&pathstr),
                &"SURFACE" => self.export_surface_to_obj(&pathstr),
                other => {
                    panic!("Unexpected type for OBJ export {}", other);
                }
            },
            EXPORT_ID_COLLADA => match &self.typestr.as_str() {
                &"MESH" => self.export_mesh_to_collada(&pathstr),
                &"SKIN" => self.export_skin_to_collada(&pathstr, true),
                other => {
                    panic!("Unexpected type for OBJ export {}", other);
                }
            },
            other => {
                panic!("Unexpected export type {}", other);
            }
        };
    }

    /// Replace a TXT file with the given string
    #[export]
    pub fn replace_txt_with_string(&mut self, _owner: &Resource, stringdata: GodotString) {
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

    ///////////////////////////////////////////////////////////////////////////
    // IMPORT DATA                                                           //
    ///////////////////////////////////////////////////////////////////////////

    /// Import BITMAP data from a file with the given format
    #[export]
    pub fn import_bitmap(
        &mut self,
        _owner: &Resource,
        path: GodotString,
        formattype: i64,
        palettetype: i64,
    ) {
        use libchum::reader::bitmap;
        let pathstr = path.to_string();
        let fh = File::open(&pathstr).unwrap();
        let image_format = bitmap::image::ImageFormat::from_path(&pathstr).unwrap();
        let mut buf_reader = BufReader::new(fh);
        let (bitmap, width, height) = bitmap::import_bitmap(&mut buf_reader, image_format).unwrap();
        let mut data =
            bitmap::BitmapFormat::new_empty(formattype as u8, palettetype as u8).unwrap();
        bitmap::compress_bitmap(&bitmap, &mut data, width, height);
        let bitmap =
            match reader::bitmap::Bitmap::read_data(&mut self.get_data_as_vec(), self.format) {
                Ok(x) => x,
                Err(err) => {
                    panic!("BITMAP file invalid: {}", err);
                }
            }
            .with_bitmap(data, width, height);
        let mut outdata = Vec::new();
        bitmap.write_to(&mut outdata, self.format).unwrap();
        self.replace_data_with_vec(outdata);
    }

    ///////////////////////////////////////////////////////////////////////////
    // STRUCTURE DATA                                                        //
    ///////////////////////////////////////////////////////////////////////////

    #[export]
    pub fn read_structure(&self, _owner: &Resource) -> Variant {
        match self.get_type_str() {
            "BITMAP" => {
                let bitmap = match reader::bitmap::Bitmap::read_data(
                    &mut self.get_data_as_vec(),
                    self.format,
                ) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("BITMAP file invalid: {}", err);
                    }
                };
                let data = bitmap.structure();
                util::struct_to_dict(&data).into_shared().to_variant()
            }
            "MATERIAL" => {
                let material = match reader::material::Material::read_data(
                    &self.get_data_as_vec(),
                    self.format,
                ) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("MATERIAL file invalid: {}", err);
                    }
                };
                let data = material.structure();
                util::struct_to_dict(&data).into_shared().to_variant()
            }
            _ => Variant::new(),
        }
    }

    #[export]
    pub fn import_structure(&mut self, _owner: &Resource, data: Dictionary) {
        let structure = util::dict_to_struct(&data);
        match self.get_type_str() {
            "BITMAP" => {
                let bitmap = match reader::bitmap::Bitmap::read_data(
                    &mut self.get_data_as_vec(),
                    self.format,
                ) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("BITMAP file invalid: {}", err);
                    }
                };
                let bitmapdata = reader::bitmap::Bitmap::destructure(structure)
                    .unwrap()
                    .with_bitmap(
                        bitmap.get_data().clone(),
                        bitmap.get_width(),
                        bitmap.get_height(),
                    );
                let mut outdata = Vec::new();
                bitmapdata.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "MATERIAL" => {
                let materialdata = reader::material::Material::destructure(structure).unwrap();
                let mut outdata = Vec::new();
                materialdata.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            _ => panic!("Could not import data"),
        }
    }
}
