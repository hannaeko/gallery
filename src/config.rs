pub struct ThumbnailConfig {
    pub size: u32,
    pub square: bool,
}

pub struct Config {
    pub gallery_name: &'static str,

    pub storage_path: &'static str,
    pub cache_path: &'static str,

    pub small_thumbnail: ThumbnailConfig,
}

impl Config {
    pub fn load() -> Self {
        Config {
            gallery_name: "gallery",

            storage_path: "/home/zorg/documents/projets/gallery/storage",
            cache_path: "/home/zorg/documents/projets/gallery/cache",

            small_thumbnail: ThumbnailConfig {
                size: 250,
                square: true
            },
        }
    }
}
