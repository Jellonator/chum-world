macro_rules! impl_view_base {
    (
        $name:ty,
        $type:ty,
        $typename:literal,
        $block:expr
    ) => {
        fn _register(builder: &ClassBuilder<$name>) {
            builder.add_method("get_type", gdnative::godot_wrap_method!($name,
                fn get_type(&self, _owner: &Resource) -> &str
            ));
            builder.add_method("load", gdnative::godot_wrap_method!($name,
                fn load(&mut self, _owner: &Resource, data: Instance<$crate::chumfile::ChumFile, Shared>)
            ));
            builder.add_signal(Signal {
                name: "modified",
                args: &[]
            });
            $block(builder)
        }

        pub fn get_type(&self, _owner: &Resource) -> &str {
            $typename
        }

        pub fn set_data(&mut self, data: $type) {
            self.inner = data;
        }

        pub fn load(&mut self, owner: &Resource, data: Instance<$crate::chumfile::ChumFile, Shared>) {
            if let Err(e) = self.load_from(data) {
                display_err!("Error while loading {} into view: {}", $typename, e);
            } else {
                owner.emit_signal("modified", &[]);
            }
        }

        #[allow(unused_imports)]
        pub fn load_from(&mut self, data: Instance<$crate::chumfile::ChumFile, Shared>) -> $crate::anyhow::Result<()> {
            use libchum::binary::ChumBinary;
            unsafe {
                let data = data.assume_safe();
                self.inner = data.map(|cfile, _| {
                    cfile.borrow_data(|mut inner_data| {
                        <$type>::read_from(&mut inner_data, cfile.get_format())
                    })
                })??;
            }
            Ok(())
        }
    }
}

/// Generate a few methods and signals that all View types must have.
/// This includes:
/// func load(ChumFile) - loads data into this view from a ChumFile instance
/// func save(ChumFile) - saves data from this view into a ChumFile instance
/// func get_type(): String - returns this view's type
/// signal modified() - Called when this view's data is modified
#[macro_export]
macro_rules! impl_view {
    (
        $name:ty,
        $type:ty,
        $typename:literal,
        $block:expr,
        $custom_save:expr
    ) => {
        impl_view_base!(
            $name, $type, $typename,
            |builder: &ClassBuilder<$name>| {
                builder.add_method("save", gdnative::godot_wrap_method!($name,
                    fn save(&self, _owner: &Resource, _data: Instance<$crate::chumfile::ChumFile, Shared>)
                ));
                $block(builder)
                
            }
        );

        pub fn save(&self, owner: &Resource, data: Instance<$crate::chumfile::ChumFile, Shared>) {
            $custom_save(self, owner, data)
        }
    };
    (
        $name:ty,
        $type:ty,
        $typename:literal,
        $block:expr
    ) => {
        impl_view_base!(
            $name, $type, $typename,
            |builder: &ClassBuilder<$name>| {
                builder.add_method("save", gdnative::godot_wrap_method!($name,
                    fn save(&self, _owner: &Resource, data: Instance<$crate::chumfile::ChumFile, Shared>)
                ));
                $block(builder)
                
            }
        );

        #[allow(unused_imports)]
        pub fn save(&self, _owner: &Resource, data: Instance<$crate::chumfile::ChumFile, Shared>) {
            use libchum::binary::ChumBinary;
            let mut v: Vec<u8> = Vec::new();
            unsafe { data.assume_safe() }
                .map_mut(|chumfile, _| {
                    self.inner.write_to(&mut v, chumfile.get_format()).unwrap();
                    chumfile.replace_data_with_vec(v);
                })
                .unwrap();
        }
    }
}

#[macro_export]
macro_rules! impl_view_node_resource {
    (
        $name:ty,
        $type:ty,
        $typename:literal,
        $block:expr
        $(,$custom_save:expr)?
    ) => {
        impl_view!(
            $name, $type, $typename,
            |builder: &ClassBuilder<$name>| {
                builder.add_method("get_flags", gdnative::godot_wrap_method!($name,
                    fn get_flags(&self, _owner: &Resource) -> i64
                ));
                builder.add_method("set_flags", gdnative::godot_wrap_method!($name,
                    fn set_flags(&mut self, owner: &Resource, value: i64)
                ));
                builder.add_method("get_header_transform", gdnative::godot_wrap_method!($name,
                    fn get_header_transform(&self, _owner: TRef<Resource>) -> Transform
                ));
                builder.add_method("set_header_transform", gdnative::godot_wrap_method!($name,
                    fn set_header_transform(&mut self, _owner: TRef<Resource>, value: Transform)
                ));
                builder.add_method("get_header_floats", gdnative::godot_wrap_method!($name,
                    fn get_header_floats(&self, _owner: TRef<Resource>) -> VariantArray<Shared>
                ));
                builder.add_method("set_header_floats", gdnative::godot_wrap_method!($name,
                    fn set_header_floats(&mut self, _owner: TRef<Resource>, value: VariantArray<Shared>)
                ));
                builder.add_property("flags")
                    .with_getter(Self::get_flags)
                    .with_setter(Self::set_flags)
                    .done();
                builder.add_property("header_transform")
                    .with_getter(Self::get_header_transform)
                    .with_setter(Self::set_header_transform)
                    .done();
                builder.add_property("header_floats")
                    .with_getter(Self::get_header_floats)
                    .with_setter(Self::set_header_floats)
                    .done();
                $block(builder);
            }
            $(,$custom_save)?
        );

        pub fn get_flags(&self, _owner: TRef<Resource>) -> i64 {
            self.inner.item_flags as i64
        }

        pub fn set_flags(&mut self, owner: TRef<Resource>, value: i64) {
            self.inner.item_flags = value as u16;
            owner.emit_signal("modified", &[]);
        }

        pub fn get_header_transform(&self, _owner: TRef<Resource>) -> Transform {
            util::transform3d_to_godot(&self.inner.header.transform)
        }

        pub fn set_header_transform(&mut self, owner: TRef<Resource>, value: Transform) {
            self.inner.header.transform = util::godot_to_transform3d(&value);
            owner.emit_signal("modified", &[]);
        }

        pub fn get_header_floats(&self, _owner: TRef<Resource>) -> VariantArray<Shared> {
            let arr = VariantArray::new();
            arr.push(self.inner.header.floats[0]);
            arr.push(self.inner.header.floats[1]);
            arr.push(self.inner.header.floats[2]);
            arr.push(self.inner.header.floats[3]);
            arr.into_shared()
        }

        pub fn set_header_floats(&mut self, owner: TRef<Resource>, value: VariantArray<Shared>) {
            for i in 0..4 {
                self.inner.header.floats[i] = value.get(i as i32).to_f64() as f32;
            }
            owner.emit_signal("modified", &[]);
        }
    };
}
