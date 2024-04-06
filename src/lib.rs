use std::fs;
use std::collections::HashMap;

pub struct Scrounger {
    search_directory: String,
    files: Vec<String>,
    dead_methods: Vec<String>,
}


impl Scrounger {
    pub fn new(search_directory: String) -> Scrounger {
        Scrounger {
            search_directory,
            files: Vec::new(),
            dead_methods: Vec::new(),
        }
    }

    pub fn search(&mut self) {
        let paths = fs::read_dir(&self.search_directory).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let path_str = path.to_str().unwrap().to_owned();
            if path_str.ends_with(".py") {
                self.files.push(path_str);
            }
        }
    }

    fn read_file(&self, file: &str) -> String {
        fs::read_to_string(file).unwrap()
    }

    fn find_methods_in_file(&self, file: &str) -> Vec<String> {
        let file_contents = self.read_file(file);
        let mut methods = Vec::new();
        for line in file_contents.lines() {
            if line.contains("def ") {
                methods.push(self.get_method_name(line));
            }
        }
        methods
    }

    fn get_method_name(&self, line: &str) -> String {
        let almost_method_name = line.trim_start().replace("def ", "");
        let method_name_end = almost_method_name.find(" ").unwrap();
        println!("{:?}", almost_method_name);
        let method_name = almost_method_name[..method_name_end].to_owned();
        return method_name;
    }

    pub fn search_files(&mut self) {
        let mut files_to_methods = HashMap::new();
        for file in &self.files {
            let methods = self.find_methods_in_file(file);
            files_to_methods.insert(file, methods);
        }
        print!("{:?}", files_to_methods);
    }

    pub fn find_dead_methods(&mut self) {
        // Find dead methods in the files
        // and store them in the dead_methods vector
    }

    pub fn print_dead_methods(&self) {
        for method in &self.dead_methods {
            println!("{}", method);
        }
    }
}