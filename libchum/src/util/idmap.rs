use crate::util;
use std::collections::hash_map;
use std::collections::HashMap;
use std::iter;

/// An element from an IdMap.
/// Contains the element's name and value.
/// Mutable references may only change the element's value, not their name.
#[derive(Clone, Debug)]
pub struct Element<T> {
    name: String,
    value: T,
}

impl<T> Element<T> {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_value_ref(&self) -> &T {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// A HashMap variation that relates IDs to arbitrary values.
/// An ID in this case is the same as Totemtech's ID system, where i32 are used
/// as keys, and values will store the unhashed version of the ID.
/// Although this structure operates similarly to a HashMap, the arguments/returns
/// of several functions are altered to reflect the ID system.
/// Functions that would normally return T now return Element<T>, which includes
/// the unhashed name.
/// Functions that accept keys take i32.
/// The 'keys' function has been split into 'keys_str' and 'keys_i32'.
/// Any function that performs some sort of insertion will require a String
/// value instead of i32.
#[derive(Clone, Default, Debug)]
pub struct IdMap<T> {
    data: HashMap<i32, Element<T>>,
}

impl<T> IdMap<T> {
    pub fn get_name_from_id(&self, id: i32) -> Option<&str> {
        self.data.get(&id).map(|x| x.get_name())
    }

    pub fn hash_id(key: &str) -> i32 {
        util::hash_name_i32(key)
    }

    // The following methods are all based on std::collections::HashMap
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn contains_key(&self, k: i32) -> bool {
        self.data.contains_key(&k)
    }

    pub fn drain(&mut self) -> hash_map::Drain<'_, i32, Element<T>> {
        self.data.drain()
    }

    pub fn entry(&mut self, key: i32) -> hash_map::Entry<'_, i32, Element<T>> {
        self.data.entry(key)
    }

    pub fn get(&self, k: i32) -> Option<&Element<T>> {
        self.data.get(&k)
    }

    pub fn get_key_value(&self, k: i32) -> Option<(&i32, &Element<T>)> {
        self.data.get_key_value(&k)
    }

    pub fn get_mut(&mut self, k: i32) -> Option<&mut Element<T>> {
        self.data.get_mut(&k)
    }

    pub fn insert(&mut self, k: String, v: T) -> Option<Element<T>> {
        let h = util::hash_name_i32(&k);
        self.data.insert(h, Element { name: k, value: v })
    }

    pub fn remove(&mut self, k: i32) -> Option<Element<T>> {
        self.data.remove(&k)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> hash_map::Iter<i32, Element<T>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> hash_map::IterMut<i32, Element<T>> {
        self.data.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn new() -> IdMap<T> {
        IdMap {
            data: HashMap::new(),
        }
    }

    pub fn remove_entry(&mut self, k: i32) -> Option<(i32, Element<T>)> {
        self.data.remove_entry(&k)
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&i32, &mut Element<T>) -> bool,
    {
        self.data.retain(f)
    }

    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit()
    }

    pub fn keys_i32(&self) -> hash_map::Keys<i32, Element<T>> {
        self.data.keys()
    }

    pub fn keys_str(
        &self,
    ) -> iter::Map<hash_map::Values<i32, Element<T>>, fn(&Element<T>) -> &str> {
        self.data.values().map(|x| &x.name.as_str())
    }

    pub fn values(&self) -> hash_map::Values<i32, Element<T>> {
        self.data.values()
    }

    pub fn values_mut(&mut self) -> hash_map::ValuesMut<i32, Element<T>> {
        self.data.values_mut()
    }

    pub fn with_capacity(capacity: usize) -> IdMap<T> {
        IdMap {
            data: HashMap::with_capacity(capacity),
        }
    }
}

impl<T> iter::Extend<(String, T)> for IdMap<T> {
    fn extend<U>(&mut self, iter: U)
    where
        U: iter::IntoIterator<Item = (String, T)>,
    {
        self.data.extend(
            iter.into_iter()
                .map(|(k, v)| (util::hash_name_i32(&k), Element { name: k, value: v })),
        )
    }
}
