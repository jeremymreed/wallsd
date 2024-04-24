use std::fs;

use crate::image_verification;

#[derive(Debug, Clone)]
pub struct Collection {
    pub collection: Vec<String>,
}

impl Collection {
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
            }
        };
    }

    pub fn process_file(&mut self, absolute_path: &String) {
        if image_verification::is_supported_format(absolute_path) {
            self.collection.push(absolute_path.clone());
        } else {
            tracing::warn!("{}: Unsupported image format", absolute_path);
        }
    }

    pub fn process_directory(&mut self, absolute_path: &String) {
        let paths = fs::read_dir(absolute_path).unwrap();

        for path in paths {
            let path = path.unwrap().path();

            self.process(&path.display().to_string());
        }

    }
}