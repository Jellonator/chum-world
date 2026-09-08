
macro_rules! impl_view_base {
    (
        $name:ty,
        $type:ty,
        $typename:literal,
        $registerblock:expr,
        $($fndef:tt)*
    ) => {
        #[methods]
        impl $name
        {
            $($fndef)*

            fn _register(builder: &ClassBuilder<$name>) {
                builder
                    .signal("modified")
                    .done();
                ($registerblock)(builder)
            }
    
            #[method]
            pub fn get_type(&self) -> &str {
                $typename
            }
    
            pub fn set_data(&mut self, data: $type) {
                self.inner = data;
            }
    
            #[method]
            pub fn load(&mut self, #[base] owner: &Resource, data: Instance<$crate::chumfile::ChumFile, Shared>) {
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
        save $custom_save:expr,
        $($fndef:tt)*
    ) => {
        impl_view_base!(
            $name, $type, $typename,
            |builder: &ClassBuilder<$name>| {
                ($block)(builder)
            },
            $($fndef)*
            #[export]
            pub fn save(&self, owner: &Resource, data: Instance<$crate::chumfile::ChumFile, Shared>) -> () {
                ($custom_save)(self, owner, data)
            }
        );
    };
    (
        $name:ty,
        $type:ty,
        $typename:literal,
        $block:expr,
        $($fndef:tt)*
    ) => {
        impl_view_base!(
            $name, $type, $typename,
            |builder: &ClassBuilder<$name>| {
                ($block)(builder)
            },
            $($fndef)*
            #[method]
            #[allow(unused_imports)]
            pub fn save(&self, data: Instance<$crate::chumfile::ChumFile, Shared>) -> () {
                use libchum::binary::ChumBinary;
                let mut v: Vec<u8> = Vec::new();
                unsafe { data.assume_safe() }
                    .map_mut(|chumfile, _| {
                        self.inner.write_to(&mut v, chumfile.get_format()).unwrap();
                        chumfile.replace_data_with_vec(v);
                    })
                    .unwrap();
            }
        );

    }
}

#[macro_export]
macro_rules! impl_view_node_resource {
    (
        $name:ty,
        $type:ty,
        $typename:literal,
        $block:expr,
        save $custom_save:expr,
        $($fndef:tt)*
    ) => {
        impl_view!(
            $name, $type, $typename,
            |builder: &ClassBuilder<$name>| {
                builder.property("flags")
                    .with_getter(Self::get_flags)
                    .with_setter(Self::set_flags)
                    .done();
                builder.property("header_transform")
                    .with_getter(Self::get_header_transform)
                    .with_setter(Self::set_header_transform)
                    .done();
                builder.property("header_floats")
                    .with_getter(Self::get_header_floats)
                    .with_setter(Self::set_header_floats)
                    .done();
                ($block)(builder);
            },

            save $custom_save,
            
            $($fndef)*

            #[export]
            pub fn get_flags(&self, _owner: TRef<Resource>) -> i64 {
                self.inner.item_flags as i64
            }
    
            #[export]
            pub fn set_flags(&mut self, owner: TRef<Resource>, value: i64) -> () {
                self.inner.item_flags = value as u16;
                owner.emit_signal("modified", &[]);
            }
    
            #[export]
            pub fn get_header_transform(&self, _owner: TRef<Resource>) -> Transform {
                util::transform3d_to_godot(&self.inner.header.transform)
            }
    
            #[export]
            pub fn set_header_transform(&mut self, owner: TRef<Resource>, value: Transform) -> () {
                self.inner.header.transform = util::godot_to_transform3d(&value);
                owner.emit_signal("modified", &[]);
            }
    
            #[export]
            pub fn get_header_floats(&self, _owner: TRef<Resource>) -> VariantArray<Shared> {
                let arr = VariantArray::new();
                arr.push(self.inner.header.floats[0]);
                arr.push(self.inner.header.floats[1]);
                arr.push(self.inner.header.floats[2]);
                arr.push(self.inner.header.floats[3]);
                arr.into_shared()
            }
    
            #[export]
            pub fn set_header_floats(&mut self, owner: TRef<Resource>, value: VariantArray<Shared>) -> () {
                for i in 0..4 {
                    self.inner.header.floats[i] = value.get(i as i32).to_variant().to::<f32>().unwrap();
                }
                owner.emit_signal("modified", &[]);
            }
        );
    };
    (
        $name:ty,
        $type:ty,
        $typename:literal,
        $block:expr,
        $($fndef:tt)*
    ) => {
        impl_view!(
            $name, $type, $typename,
            |builder: &ClassBuilder<$name>| {
                builder.property("flags")
                    .with_getter(Self::get_flags)
                    .with_setter(Self::set_flags)
                    .done();
                builder.property("header_transform")
                    .with_getter(Self::get_header_transform)
                    .with_setter(Self::set_header_transform)
                    .done();
                builder.property("header_floats")
                    .with_getter(Self::get_header_floats)
                    .with_setter(Self::set_header_floats)
                    .done();
                ($block)(builder);
            },

            #[export]
            pub fn get_flags(&self, _owner: TRef<Resource>) -> i64 {
                self.inner.item_flags as i64
            }
    
            #[export]
            pub fn set_flags(&mut self, owner: TRef<Resource>, value: i64) -> () {
                self.inner.item_flags = value as u16;
                owner.emit_signal("modified", &[]);
            }
    
            #[export]
            pub fn get_header_transform(&self, _owner: TRef<Resource>) -> Transform {
                util::transform3d_to_godot(&self.inner.header.transform)
            }
    
            #[export]
            pub fn set_header_transform(&mut self, owner: TRef<Resource>, value: Transform) -> () {
                self.inner.header.transform = util::godot_to_transform3d(&value);
                owner.emit_signal("modified", &[]);
            }
    
            #[export]
            pub fn get_header_floats(&self, _owner: TRef<Resource>) -> VariantArray<Shared> {
                let arr = VariantArray::new();
                arr.push(self.inner.header.floats[0]);
                arr.push(self.inner.header.floats[1]);
                arr.push(self.inner.header.floats[2]);
                arr.push(self.inner.header.floats[3]);
                arr.into_shared()
            }
    
            #[export]
            pub fn set_header_floats(&mut self, owner: TRef<Resource>, value: VariantArray<Shared>) -> () {
                for i in 0..4 {
                    self.inner.header.floats[i] = value.get(i as i32).to_variant().to::<f32>().unwrap();
                }
                owner.emit_signal("modified", &[]);
            }
            
            $($fndef)*
        );
    };
}
