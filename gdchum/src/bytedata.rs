use gdnative::*;

/// More or less a combination of Godot's PoolByteArray and
/// Array types. Note that modifying the data will modify the
/// original data as well.
#[derive(NativeClass)]
#[inherit(Resource)]
pub struct ByteData {
    data: Vec<u8>
}

#[methods]
impl ByteData {
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    #[export]
    pub fn get(&self, _owner: Resource, mut index: i64) -> i64 {
        if index < 0 {
            index = self.data.len() as i64 + index;
        }
        self.data[index as usize] as i64
    }

    #[export]
    pub fn set(&mut self, _owner: Resource, index: i64, value: u8) {
        self.data[index as usize] = value as u8;
    }

    #[export]
    pub fn append(&mut self, _owner: Resource, value: u8) {
        self.data.push(value);
    }

    #[export]
    pub fn size(&self, _owner: Resource) -> i64 {
        self.data.len() as i64
    }

    #[export]
    pub fn back(&self, _owner: Resource) -> Variant {
        match self.data.len() {
            0 => Variant::new(),
            x => (self.data[x-1] as i64).into()
        }
    }

    #[export]
    pub fn clear(&mut self, _owner: Resource) {
        self.data.clear()
    }

    #[export]
    pub fn count(&self, _owner: Resource, value: u8) -> i64 {
        self.data.iter()
                 .filter(|x| **x == value)
                 .count() as i64
    }

    #[export]
    pub fn duplicate(&self, _owner: Resource) -> Resource {
        let f = Instance::<ByteData>::new();
        f.map_mut(|script, res| {
            script.set_data(self.data.clone());
            res
        }).unwrap()
    }

    #[export]
    pub fn empty(&self, _owner: Resource) -> bool {
        self.data.is_empty()
    }

    #[export]
    pub fn erase(&mut self, _owner: Resource, value: u8) {
        if let Some(index) = self.data.iter().position(|x| *x == value) {
            self.data.remove(index);
        }
    }

    #[export]
    pub fn find(&self, _owner: Resource, value: u8) -> i64 {
        if let Some(index) = self.data.iter().position(|x| *x == value) {
            (index as i64).into()
        } else {
            -1
        }
    }

    #[export]
    pub fn find_from(&self, _owner: Resource, value: u8, mut from: i64) -> i64 {
        if from < 0 {
            from = self.data.len() as i64 + from;
        }
        if let Some(index) = self.data[from as usize..].iter().position(|x| *x == value) {
            (index as i64).into()
        } else {
            -1
        }
    }

    #[export]
    pub fn find_last(&self, _owner: Resource, value: u8) -> i64 {
        if let Some(index) = self.data.iter().rposition(|x| *x == value) {
            (index as i64).into()
        } else {
            -1
        }
    }

    #[export]
    pub fn find_last_from(&self, _owner: Resource, value: u8, mut from: i64) -> i64 {
        if from < 0 {
            from = self.data.len() as i64 + from;
        }
        if let Some(index) = self.data[..=from as usize].iter().rposition(|x| *x == value) {
            (index as i64).into()
        } else {
            -1
        }
    }

    #[export]
    pub fn front(&self, _owner: Resource) -> Variant {
        match self.data.len() {
            0 => Variant::new(),
            _x => (self.data[0] as i64).into()
        }
    }

    #[export]
    pub fn has(&self, _owner: Resource, value: u8) -> bool {
        if let Some(_index) = self.data.iter().position(|x| *x == value) {
            true
        } else {
            false
        }
    }

    #[export]
    pub fn insert(&mut self, _owner: Resource, index: i64, value: u8) {
        self.data.insert(index as usize, value);
    }

    #[export]
    pub fn slice(&self, _owner: Resource, from: i64, to: i64) -> Resource {
        let f = Instance::<ByteData>::new();
        f.map_mut(|script, res| {
            script.set_data(self.data[from as usize..(to+1) as usize].to_owned());
            res
        }).unwrap()
    }

    #[export]
    pub fn pop_back(&mut self, _owner: Resource) -> Variant {
        match self.data.len() {
            0 => Variant::new(),
            _x => (self.data.pop().unwrap() as i64).into()
        }
    }

    #[export]
    pub fn pop_front(&mut self, _owner: Resource) -> Variant {
        match self.data.len() {
            0 => Variant::new(),
            _x => (self.data.remove(0) as i64).into()
        }
    }

    #[export]
    pub fn push_front(&mut self, _owner: Resource, value: u8) {
        self.data.insert(0, value);
    }

    #[export]
    pub fn push_back(&mut self, _owner: Resource, value: u8) {
        self.data.push(value);
    }

    #[export]
    pub fn remove(&mut self, _owner: Resource, index: i64) {
        self.data.remove(index as usize);
    }

    #[export]
    pub fn resize(&mut self, _owner: Resource, size: i64) {
        self.data.resize(size as usize, 0);
    }

    fn _init(_owner: Resource) -> Self {
        ByteData {
            data: Vec::new()
        }
    }
}