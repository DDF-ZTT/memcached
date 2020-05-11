use std::{
    collections::hash_map::DefaultHasher,
    convert::TryInto,
    hash::{Hash, Hasher},
};

#[allow(clippy::result_unwrap_used)]
pub(crate) fn default_hash_function(key: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish().try_into().unwrap()
}
