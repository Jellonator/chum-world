use crate::bytedata::ByteData;
use crate::scenedata;
use crate::util;
use crate::views::*;
use crate::ChumArchive;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::binary::ChumBinary;
use libchum::{archive, common, reader, scene, structure::ChumStruct};
use std::fs::File;
use std::io::{BufReader, Write};

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
    parent: Option<Instance<ChumArchive, Shared>>,
    format: libchum::format::TotemFormat,
}

// Direct copy of res://Gui/EditorList.gd
const EXPORT_ID_BIN: i64 = 0;
const EXPORT_ID_TEXT: i64 = 1;
const EXPORT_ID_MODEL: i64 = 2;
const EXPORT_ID_TEXTURE: i64 = 3;
const EXPORT_ID_COLLADA: i64 = 4;
const EXPORT_ID_WAV: i64 = 5;

macro_rules! get_view {
    ($viewtype:ty, $chumfile:expr) => {{
        let instance = Instance::<$viewtype, Unique>::new();
        match instance.map_mut(|nodeview, _| nodeview.load_from($chumfile)) {
            Ok(value) => match value {
                Ok(_inner) => Ok(instance.into_shared().to_variant()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e.into()),
        }
    }};
}

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
    pub fn get_archive(&self, _owner: &Resource) -> Instance<ChumArchive, Shared> {
        self.parent.clone().unwrap()
    }

    #[export]
    pub fn get_data(&self, _owner: TRef<Resource>) -> Instance<ByteData, Shared> {
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
        libchum::util::hash_name_i32(&self.namestr)
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
    pub fn get_archive_instance(&self) -> &Instance<ChumArchive, Shared> {
        self.parent.as_ref().unwrap()
    }

    /// Read a file from the given ChumArchive
    pub fn read_from_chumfile(
        &mut self,
        file: &archive::ChumFile,
        fmt: libchum::format::TotemFormat,
        parent: Instance<ChumArchive, Shared>,
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
    pub fn get_data_as_bytedata(&self) -> Instance<ByteData, Shared> {
        let archive_instance = self.get_archive_instance();
        unsafe { archive_instance.assume_safe() }
            .map(|archive, _| {
                let file = archive.archive.get_file_from_name(&self.namestr).unwrap();
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
        unsafe { archive_instance.assume_safe() }
            .map(|archive, _| {
                let file = archive.archive.get_file_from_name(&self.namestr).unwrap();
                file.get_data().to_owned()
            })
            .unwrap()
    }

    pub fn borrow_data<F, G>(&self, func: F) -> G
    where
        F: Fn(&[u8]) -> G,
    {
        let archive_instance = self.get_archive_instance();
        unsafe { archive_instance.assume_safe() }
            .map(|archive, _| {
                let file: &archive::ChumFile =
                    archive.archive.get_file_from_name(&self.namestr).unwrap();
                func(file.get_data())
            })
            .unwrap()
    }

    pub fn borrow_data_mut<F, G>(&mut self, func: F) -> G
    where
        F: Fn(&mut Vec<u8>) -> G,
    {
        let archive_instance = self.get_archive_instance();
        unsafe { archive_instance.assume_safe() }
            .map_mut(|archive, _| {
                let file: &mut archive::ChumFile = archive
                    .archive
                    .get_file_from_name_mut(&self.namestr)
                    .unwrap();
                func(file.get_data_mut())
            })
            .unwrap()
    }

    /// Replace this file's data with the given Vec<u8>
    pub fn replace_data_with_vec(&mut self, data: Vec<u8>) {
        let archive_instance = self.get_archive_instance();
        unsafe { archive_instance.assume_safe() }
            .map_mut(|archive, _| {
                archive
                    .archive
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
        let mesh = match reader::mesh::Mesh::read_from(
            &mut self.get_data_as_vec().as_slice(),
            self.format,
        ) {
            Ok(x) => x,
            Err(err) => {
                panic!("MESH file invalid: {}", err);
            }
        };
        mesh.export_obj(&mut buffer).unwrap();
    }

    fn export_skin_to_gltf(&self, path: &str) {
        use libchum::binary::ChumBinary;
        let skin = match reader::skin::Skin::read_from(
            &mut self.get_data_as_vec().as_slice(),
            self.format,
        ) {
            Ok(x) => x,
            Err(err) => {
                panic!("MESH file invalid: {}", err);
            }
        };
        let mut scene = scene::Scene::new_empty();
        let archiveinstance = self.get_archive_instance();
        let mut skin_meshes = Vec::new();
        unsafe { archiveinstance.assume_safe() }
            .map(|archive, res| {
                for meshid in skin.meshes.iter() {
                    if let Some(meshfile) = archive.get_file_from_hash(&res, *meshid) {
                        unsafe { meshfile.assume_safe() }
                            .map(|meshscript, _| match meshscript.typestr.as_str() {
                                "MESH" => {
                                    let mesh = match reader::mesh::Mesh::read_from(
                                        &mut meshscript.get_data_as_vec().as_slice(),
                                        self.format,
                                    ) {
                                        Ok(x) => x,
                                        Err(err) => {
                                            panic!("MESH file invalid: {}", err);
                                        }
                                    };
                                    // add mesh if not exists
                                    let mut trimesh = mesh.create_scene_mesh();
                                    trimesh.skin =
                                        mesh.generate_mesh_skin(reader::skin::SkinInfo {
                                            names: archive.get_name_map(),
                                            skin: &skin,
                                            skin_id: self.get_hash_id_ownerless(),
                                            mesh_id: *meshid,
                                        });
                                    scene
                                        .meshes
                                        .insert(meshscript.get_name_str().to_string(), trimesh);
                                    skin_meshes.push(meshscript.get_name_str().to_string());
                                }
                                _ => {}
                            })
                            .unwrap();
                    } else {
                        godot_warn!("Mesh {} does not exist!", meshid);
                    }
                }
                let mut scene_skin = scene::Skin {
                    joints: skin.generate_scene_skin_joints(archive.get_name_map()),
                };
                scene_skin
                    .auto_set_joint_transforms(scene.meshes.values().map(|x| x.get_value_ref()));
                scene.root.graphic = scene::NodeGraphic::Skin {
                    skin: scene_skin,
                    meshes: skin_meshes,
                };
            })
            .unwrap();
        scenedata::add_required_materials(&mut scene, self.get_archive_instance().clone());
        scene.export_to(path).expect("Serialization Error");
    }

    fn export_mesh_to_gltf(&self, path: &str) {
        let mesh = match reader::mesh::Mesh::read_from(
            &mut self.get_data_as_vec().as_slice(),
            self.format,
        ) {
            Ok(x) => x,
            Err(err) => {
                panic!("MESH file invalid: {}", err);
            }
        };
        let mut scene = scene::Scene::new_empty();
        scene
            .meshes
            .insert(self.namestr.to_string(), mesh.create_scene_mesh());
        scene.root = scene::SNode {
            tree: std::collections::HashMap::new(),
            transform: common::Transform3D::identity(),
            graphic: scene::NodeGraphic::Mesh {
                mesh: self.namestr.to_string(),
            },
        };
        scenedata::add_required_materials(&mut scene, self.get_archive_instance().clone());
        scene.export_to(path).expect("Serialization Error");
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
                panic!("SURFACE file invalid: {}", err);
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

    /// Export to .wav
    fn export_to_wav(&self, path: &str) {
        use libchum::binary::ChumBinary;
        let snd = match reader::sound::SoundGcn::read_from(
            &mut self.get_data_as_vec().as_slice(),
            self.format,
        ) {
            Ok(x) => x,
            Err(err) => {
                panic!("SOUND file invalid: {}", err);
            }
        };
        let sample = snd.gen_samples();
        use hound;
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: snd.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(path, spec).unwrap();
        for value in sample {
            writer.write_sample(value as i16).unwrap();
        }
        writer.finalize().unwrap();
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
                &"MESH" => self.export_mesh_to_gltf(&pathstr),
                &"SKIN" => self.export_skin_to_gltf(&pathstr),
                other => {
                    panic!("Unexpected type for OBJ export {}", other);
                }
            },
            EXPORT_ID_WAV => self.export_to_wav(&pathstr),
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
        // Set first four bytes to the data's size
        // Fortunately, rust lets you write to slices
        let size = data.len() - 4;
        self.format
            .write_u32(&mut &mut data[0..4], size as u32)
            .unwrap();
        // Actually set the data
        self.replace_data_with_vec(data);
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
        bitmap::compress_bitmap(&bitmap, &mut data, width, height).unwrap();
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
        use libchum::binary::ChumBinary;
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
                let data = bitmap.get_struct().structure();
                util::struct_to_dict(&data).into_shared().to_variant()
            }
            "MATERIAL" => {
                let matdata: Vec<u8> = self.get_data_as_vec();
                let material = match reader::material::Material::read_from(
                    &mut matdata.as_slice(),
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
            "COLLISIONVOL" => {
                let voldata: Vec<u8> = self.get_data_as_vec();
                let vol = match reader::collisionvol::CollisionVol::read_from(
                    &mut voldata.as_slice(),
                    self.format,
                ) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("COLLISIONVOL file invalid: {}", err);
                    }
                };
                let data = vol.structure();
                util::struct_to_dict(&data).into_shared().to_variant()
            }
            "WARP" => {
                let warpdata: Vec<u8> = self.get_data_as_vec();
                let warp =
                    match reader::warp::Warp::read_from(&mut warpdata.as_slice(), self.format) {
                        Ok(x) => x,
                        Err(err) => {
                            panic!("WARP file invalid: {}", err);
                        }
                    };
                let data = warp.structure();
                util::struct_to_dict(&data).into_shared().to_variant()
            }
            "ROTSHAPE" => {
                let vecdata: Vec<u8> = self.get_data_as_vec();
                let data = match reader::rotshape::RotShape::read_from(
                    &mut vecdata.as_slice(),
                    self.format,
                ) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("ROTSHAPE file invalid: {}", err);
                    }
                };
                let structure = data.structure();
                util::struct_to_dict(&structure).into_shared().to_variant()
            }
            "OMNI" => {
                let vecdata: Vec<u8> = self.get_data_as_vec();
                let data = match reader::omni::Omni::read_from(&mut vecdata.as_slice(), self.format)
                {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("OMNI file invalid: {}", err);
                    }
                };
                let structure = data.structure();
                util::struct_to_dict(&structure).into_shared().to_variant()
            }
            "LIGHT" => {
                let vecdata: Vec<u8> = self.get_data_as_vec();
                let data =
                    match reader::light::Light::read_from(&mut vecdata.as_slice(), self.format) {
                        Ok(x) => x,
                        Err(err) => {
                            panic!("LIGHT file invalid: {}", err);
                        }
                    };
                let structure = data.structure();
                util::struct_to_dict(&structure).into_shared().to_variant()
            }
            "LOD" => {
                let vecdata: Vec<u8> = self.get_data_as_vec();
                let data = match reader::lod::Lod::read_from(&mut vecdata.as_slice(), self.format) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("LOD file invalid: {}", err);
                    }
                };
                let structure = data.structure();
                util::struct_to_dict(&structure).into_shared().to_variant()
            }
            "MATERIALANIM" => {
                let vecdata: Vec<u8> = self.get_data_as_vec();
                let data = match reader::materialanim::MaterialAnimation::read_from(
                    &mut vecdata.as_slice(),
                    self.format,
                ) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("MATERIALANIM file invalid: {}", err);
                    }
                };
                let structure = data.structure();
                util::struct_to_dict(&structure).into_shared().to_variant()
            }
            "NODE" => {
                let vecdata: Vec<u8> = self.get_data_as_vec();
                let data = match reader::node::Node::read_from(&mut vecdata.as_slice(), self.format)
                {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("NODE file invalid: {}", err);
                    }
                };
                let structure = data.structure();
                util::struct_to_dict(&structure).into_shared().to_variant()
            }
            _ => Variant::new(),
        }
    }

    pub fn priv_get_view(&self, owner: TRef<Resource>) -> anyhow::Result<Variant> {
        let instance = Instance::from_base(owner.claim()).unwrap();
        Ok(match self.get_type_str() {
            "BITMAP" => get_view!(BitmapView, instance)?,
            "CAMERA" => get_view!(CameraView, instance)?,
            "COLLISIONVOL" => get_view!(CollisionVolView, instance)?,
            "GAMEOBJ" => get_view!(GameObjView, instance)?,
            "HFOG" => get_view!(HFogView, instance)?,
            "LIGHT" => get_view!(LightView, instance)?,
            "LOD" => get_view!(LodView, instance)?,
            "MATERIAL" => get_view!(MaterialView, instance)?,
            "MATERIALANIM" => get_view!(MaterialAnimView, instance)?,
            "MATERIALOBJ" => get_view!(MaterialObjView, instance)?,
            "MESH" => get_view!(MeshView, instance)?,
            "NODE" => get_view!(NodeView, instance)?,
            "OMNI" => get_view!(OmniView, instance)?,
            "ROTSHAPE" => get_view!(RotShapeView, instance)?,
            "SKIN" => get_view!(SkinView, instance)?,
            "SOUND" => get_view!(SoundView, instance)?,
            "SPLINE" => get_view!(SplineView, instance)?,
            "SURFACE" => get_view!(SurfaceView, instance)?,
            "WARP" => get_view!(WarpView, instance)?,
            other => anyhow::bail!("No view for files of type {} yet", other),
        })
    }

    #[export]
    pub fn get_view(&self, owner: TRef<Resource>) -> Variant {
        match self.priv_get_view(owner) {
            Ok(v) => v,
            Err(e) => {
                display_err!("Error while loading {} into view: {}", self.namestr, e);
                Variant::new()
            }
        }
    }

    #[export]
    pub fn import_structure(&mut self, _owner: &Resource, data: Dictionary) {
        use libchum::binary::ChumBinary;
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
                let bitmapstruct = reader::bitmap::BitmapStruct::destructure(&structure).unwrap();
                let bitmapdata = reader::bitmap::Bitmap::from_struct(&bitmapstruct).with_bitmap(
                    bitmap.get_data().clone(),
                    bitmap.get_width(),
                    bitmap.get_height(),
                );
                let mut outdata = Vec::new();
                bitmapdata.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "MATERIAL" => {
                let materialdata = reader::material::Material::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                materialdata.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "COLLISIONVOL" => {
                let voldata = reader::collisionvol::CollisionVol::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                voldata.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "WARP" => {
                let warpdata = reader::warp::Warp::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                warpdata.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "ROTSHAPE" => {
                let data = reader::rotshape::RotShape::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                data.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "OMNI" => {
                let data = reader::omni::Omni::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                data.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "LIGHT" => {
                let data = reader::light::Light::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                data.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "LOD" => {
                let data = reader::lod::Lod::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                data.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "MATERIALANIM" => {
                let data =
                    reader::materialanim::MaterialAnimation::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                data.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            "NODE" => {
                let data = reader::node::Node::destructure(&structure).unwrap();
                let mut outdata = Vec::new();
                data.write_to(&mut outdata, self.format).unwrap();
                self.replace_data_with_vec(outdata);
            }
            _ => panic!("Could not import data"),
        }
    }
}
