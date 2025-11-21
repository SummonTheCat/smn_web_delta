use std::{
    collections::HashMap,
    fs,
    sync::OnceLock,
    path::Path,
};

static FILE_CACHE: OnceLock<FileCache> = OnceLock::new();

#[derive(Debug)]
pub struct FileCache {
    files: HashMap<String, String>,
}

impl FileCache {
    pub fn init(root: &str) {
        let mut map = HashMap::new();
        Self::load_dir(Path::new(root), &mut map, "");

        FILE_CACHE.set(FileCache { files: map })
            .expect("FileCache already initialized");
    }

    fn load_dir(base: &Path, map: &mut HashMap<String, String>, prefix: &str) {
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                let key = format!("{}/{}", prefix, name).replace("//", "/");

                if path.is_dir() {
                    Self::load_dir(&path, map, &key);
                } else if path.is_file() {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        map.insert(key, contents);
                    }
                }
            }
        }
    }

    pub fn get(path: &str) -> Option<&'static str> {
        FILE_CACHE.get()?.files.get(path).map(|v| v.as_str())
    }
}
