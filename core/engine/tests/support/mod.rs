use std::fmt::format;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use zen_engine::loader::{FilesystemLoader, FilesystemLoaderOptions};
use zen_engine::model::DecisionContent;

#[allow(dead_code)]
pub fn test_data_root() -> String {
    let cargo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    cargo_root
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("test-data")
        .to_string_lossy()
        .to_string()
}

#[allow(dead_code)]
pub fn load_test_data(key: &str) -> DecisionContent {
    let file_root = test_data_root();
    let file = File::open(Path::new(&test_data_root()).join(key)).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|e| {
        panic!(
            "Was unable to create condent from reader, with error {:#?}",
            e
        );
    })
}

#[allow(dead_code)]
pub fn create_fs_loader() -> FilesystemLoader {
    FilesystemLoader::new(FilesystemLoaderOptions {
        keep_in_memory: false,
        root: test_data_root(),
    })
}
