pub struct ThumbnailConfig {
    pub size: u32,
    pub square: bool,
    pub extension: &'static str,
}

pub struct Config {
    pub gallery_name: &'static str,

    pub storage_path: &'static str,
    pub cache_path: &'static str,

    pub small_thumbnail: ThumbnailConfig,
    pub medium_thumbnail: ThumbnailConfig,
}

impl Config {
    pub fn load() -> Self {
        Config {
            gallery_name: "gallery",

            storage_path: "/home/zorg/documents/projets/gallery/storage",
            cache_path: "/home/zorg/documents/projets/gallery/cache",

            small_thumbnail: ThumbnailConfig {
                size: 200,
                square: true,
                extension: "small.jpeg",
            },
            medium_thumbnail: ThumbnailConfig {
                size: 1000,
                square: false,
                extension: "medium.jpeg",
            },
        }
    }
}
