use std::fs;

use crate::image_verification;

#[derive(Debug, Clone)]
pub struct Collection {
    pub collection: Vec<String>,
    pub errors: Vec<String>,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            collection: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn scan_collection(&mut self, absolute_path: &String) {
        tracing::info!("Scanning collection: {}", absolute_path);
        self.process(absolute_path);
        tracing::info!("Number of Wallpapers: {}", self.collection.len());
        tracing::info!("Scanned collection.");
    }

    pub fn process(&mut self, absolute_path: &String) {
        match fs::metadata(absolute_path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    self.process_directory(absolute_path);
                } else {
                    self.process_file(absolute_path);
                }
            }
            Err(error) => {
                tracing::warn!("Couldn't process: {}, Error: {}", absolute_path, error);
                self.errors.push(format!("Couldn't process: {}, Error: {}", absolute_path, error));
            }
        };
    }

    pub fn process_file(&mut self, absolute_path: &String) {
        if image_verification::is_supported_format(absolute_path) {
            self.collection.push(absolute_path.clone());
        } else {
            tracing::warn!("{}: Unsupported image format", absolute_path);
            self.errors.push(format!("{}: Unsupported image format", absolute_path));
        }
    }

    pub fn process_directory(&mut self, absolute_path: &String) {
        let paths = match fs::read_dir(absolute_path) {
            Ok(paths) => paths,
            Err(error) => {
                tracing::warn!("Couldn't read directory: {}, Error: {}", absolute_path, error);
                self.errors.push(format!("Couldn't read directory: {}, Error: {}", absolute_path, error));
                return;
            }
        };

        for path in paths {
            //let path = path.unwrap().path();

            let path = match path {
                Ok(path) => path.path(),
                Err(error) => {
                    tracing::warn!("Couldn't get path: {}", error);
                    self.errors.push(format!("Couldn't get path: {}", error));
                    continue;
                }
            };
            self.process(&path.display().to_string());
        }
    }
}