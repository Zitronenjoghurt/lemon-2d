use std::fmt::Debug;
use std::hash::Hash;

pub trait TextureId: Debug + Copy + Hash + Eq + 'static {
    fn data(&self) -> &[u8];
    fn all() -> impl Iterator<Item = Self>;
}
