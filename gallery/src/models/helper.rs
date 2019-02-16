use std::io;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

pub trait ExifExtractor {
    const TAG_LIST: &'static [exif::Tag];

    fn extract_exif_map(path: &PathBuf) -> io::Result<HashMap<exif::Tag, String>> {
        let mut tag_map: HashMap<_,_> = Self::TAG_LIST.iter().map(|tag| (*tag, String::from(""))).collect();

        let file = fs::File::open(path)?;
        let res_reader = exif::Reader::new(&mut io::BufReader::new(&file));
        if res_reader.is_err() {
            return Ok(tag_map)
        }
        let reader = res_reader.unwrap();
        for (tag, value) in tag_map.iter_mut() {
            if let Some(field) = reader.get_field(*tag, false) {
                *value = field.value.display_as(*tag).to_string();
            }
        }
        Ok(tag_map)
    }

    fn extract_exif(&mut self, path: &PathBuf) -> io::Result<()>;
}
