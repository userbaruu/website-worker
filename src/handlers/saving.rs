use crate::models::berita::Berita;
use std::fs::File;

pub fn save_berita(array_berita: Vec<Vec<Berita>>, path: &str) {
    let mut collections: Vec<&Berita> = Vec::new();

    for berita in array_berita.iter() {
        for b in berita {
            collections.push(b);
        }
    }

    serde_json::to_writer_pretty(File::create(path).unwrap(), &collections).unwrap();
}