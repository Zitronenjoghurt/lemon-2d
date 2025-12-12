use crate::assets::texture::TextureId;
use crate::game::config::GameConfig;
use crate::prelude::AppleGraphicsBackend;
use ahash::{AHasher, HashMap, HashMapExt};
use macroquad::prelude::build_textures_atlas;
use macroquad::texture::Texture2D;
use std::hash::{Hash, Hasher};

pub mod texture;

pub struct AssetServer {
    textures: HashMap<u64, Texture2D>,
}

impl AssetServer {
    pub fn load<T: TextureId>(#[allow(unused_variables)] config: &GameConfig) -> Self {
        let mut textures = HashMap::new();
        for id in T::all() {
            let key = Self::hash_key(&id);
            let texture = Texture2D::from_file_with_format(id.data(), None);
            tracing::debug!("Successfully loaded texture: {id:?}");
            textures.insert(key, texture);
        }

        #[cfg(target_os = "macos")]
        {
            if config.apple_graphics_backend != AppleGraphicsBackend::Metal {
                build_textures_atlas();
                tracing::debug!("Successfully built texture atlas");
            } else {
                build_textures_atlas();
                // miniquad does not support it
                tracing::debug!("Skipping texture atlas build since it is not supported on Metal");
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            build_textures_atlas();
            tracing::debug!("Successfully built texture atlas");
        }

        Self { textures }
    }

    fn hash_key(id: &impl TextureId) -> u64 {
        let mut hasher = AHasher::default();
        id.hash(&mut hasher);
        std::any::TypeId::of::<Self>().hash(&mut hasher);
        hasher.finish()
    }
}
