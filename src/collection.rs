use std::fs;

pub struct Collection {
    pub collection: Vec<String>,
}

impl Collection {

    pub fn scan_collection(&mut self, absolute_path: &String) {
        self.process(absolute_path);
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
                println!("Couldn't process: {}, Error: {}", absolute_path, error);
            }
        };
    }

    pub fn process_file(&mut self, absolute_path: &String) {
        self.collection.push(absolute_path.clone());
    }

    pub fn process_directory(&mut self, absolute_path: &String) {
        let paths = fs::read_dir(absolute_path).unwrap();

        for path in paths {
            let path = path.unwrap().path();

            self.process(&path.display().to_string());
        }

    }
}