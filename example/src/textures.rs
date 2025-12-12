use lemon_2d::prelude::TextureId;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Texture {
    TilesetFloor,
}

impl TextureId for Texture {
    fn data(&self) -> &[u8] {
        match self {
            Texture::TilesetFloor => include_bytes!("../assets/TilesetFloor.png"),
        }
    }

    fn all() -> impl Iterator<Item = Self> {
        Texture::iter()
    }
}
