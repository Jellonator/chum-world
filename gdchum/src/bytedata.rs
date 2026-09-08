use gdnative::api::Resource;
use gdnative::prelude::*;

/// More or less a combination of Godot's PoolByteArray and
/// Array types. Note that modifying the data will modify the
/// original data as well.
#[derive(NativeClass)]
#[inherit(Resource)]
pub struct ByteData {
    data: Vec<u8>,
}

#[methods]
impl ByteData {
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    #[method]
    pub fn set_from_array(&mut self, data: PoolArray<u8>)
    {
        self.data = data.to_vec();
    }

    #[method]
    pub fn get(&self, mut index: i64) -> i64 {
        if index < 0 {
            index = self.data.len() as i64 + index;
        }
        self.data[index as usize] as i64
    }

    #[method]
    pub fn set(&mut self, index: i64, value: u8) {
        self.data[index as usize] = value as u8;
    }

    #[method]
    pub fn append(&mut self, value: u8) {
        self.data.push(value);
    }

    #[method]
    pub fn size(&self) -> i64 {
        self.data.len() as i64
    }

    #[method]
    pub fn back(&self) -> Variant {
        match self.data.len() {
            0 => Variant::nil(),
            x => self.data[x - 1].to_variant(),
        }
    }

    #[method]
    pub fn clear(&mut self) {
        self.data.clear()
    }

    #[method]
    pub fn count(&self, value: u8) -> i64 {
        self.data.iter().filter(|x| **x == value).count() as i64
    }

    #[method]
    pub fn duplicate(&self) -> Instance<ByteData, Unique> {
        let f = Instance::<ByteData, Unique>::new();
        f.map_mut(|script, _| {
            script.set_data(self.data.clone());
        })
        .unwrap();
        f
    }

    #[method]
    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    #[method]
    pub fn erase(&mut self, value: u8) {
        if let Some(index) = self.data.iter().position(|x| *x == value) {
            self.data.remove(index);
        }
    }

    #[method]
    pub fn find(&self, value: u8) -> i64 {
        if let Some(index) = self.data.iter().position(|x| *x == value) {
            (index as i64).into()
        } else {
            -1
        }
    }

    #[method]
    pub fn find_from(&self, value: u8, mut from: i64) -> i64 {
        if from < 0 {
            from = self.data.len() as i64 + from;
        }
        if let Some(index) = self.data[from as usize..].iter().position(|x| *x == value) {
            (index as i64).into()
        } else {
            -1
        }
    }

    #[method]
    pub fn find_last(&self, value: u8) -> i64 {
        if let Some(index) = self.data.iter().rposition(|x| *x == value) {
            (index as i64).into()
        } else {
            -1
        }
    }

    #[method]
    pub fn find_last_from(&self, value: u8, mut from: i64) -> i64 {
        if from < 0 {
            from = self.data.len() as i64 + from;
        }
        if let Some(index) = self.data[..=from as usize]
            .iter()
            .rposition(|x| *x == value)
        {
            (index as i64).into()
        } else {
            -1
        }
    }

    #[method]
    pub fn front(&self) -> Variant {
        match self.data.len() {
            0 => Variant::nil(),
            _x => self.data[0].to_variant(),
        }
    }

    #[method]
    pub fn has(&self, value: u8) -> bool {
        if let Some(_index) = self.data.iter().position(|x| *x == value) {
            true
        } else {
            false
        }
    }

    #[method]
    pub fn insert(&mut self, index: i64, value: u8) {
        self.data.insert(index as usize, value);
    }

    #[method]
    pub fn slice(&self, from: i64, to: i64) -> Instance<ByteData, Unique> {
        let f = Instance::<ByteData, Unique>::new();
        f.map_mut(|script, _| {
            script.set_data(self.data[from as usize..(to + 1) as usize].to_owned());
        })
        .unwrap();
        f
    }

    #[method]
    pub fn pop_back(&mut self) -> Variant {
        match self.data.len() {
            0 => Variant::nil(),
            _x => self.data.pop().unwrap().to_variant(),
        }
    }

    #[method]
    pub fn pop_front(&mut self) -> Variant {
        match self.data.len() {
            0 => Variant::nil(),
            _x => self.data.remove(0).to_variant(),
        }
    }

    #[method]
    pub fn push_front(&mut self, value: u8) {
        self.data.insert(0, value);
    }

    #[method]
    pub fn push_back(&mut self, value: u8) {
        self.data.push(value);
    }

    #[method]
    pub fn remove(&mut self, index: i64) {
        self.data.remove(index as usize);
    }

    #[method]
    pub fn resize(&mut self, size: i64) {
        self.data.resize(size as usize, 0);
    }

    fn new(_owner: &Resource) -> Self {
        ByteData { data: Vec::new() }
    }
}
