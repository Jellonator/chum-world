use crate::bytedata::ByteData;
use crate::ChumArchive;
use gdnative::*;
use libchum;

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
