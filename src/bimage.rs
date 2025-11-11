use sevenz_rust2::ArchiveReader;
use std::collections::HashSet;
use std::fs::File;

const DEFAULT_LOCATION: &str = "./lib/modules";


pub struct Bimage {
    image: ArchiveReader<File>,
    modules: Vec<String>,
}

impl Default for Bimage {
    fn default() -> Self {
        let reader = ArchiveReader::open(DEFAULT_LOCATION, Default::default()).expect("No image location given, and unable to open/locate default image");


        let mut modules = reader.archive().files.iter().filter(|e|{
            e.is_directory &&
                e.name.split("/").count() == 1
        }).map(|e| { e.name.clone() }).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
        modules.sort();
        Self {
            image: reader,
            modules,
        }
    }
}


impl Bimage {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        let reader = ArchiveReader::open(path, Default::default()).expect("Unable to find specified bimage.");
        Self {
            image: reader,
            ..Default::default()
        }
    }


    fn resolve_path(module: &str, class: &str) -> String {
        let module = if module.is_empty() { "java.base" } else { module };
        let class = Self::d2s(class);
        format!("{module}/classes/{class}.class")
    }
    fn d2s( dots: &str) -> String {
        dots.replace(".", "/")
    }
    fn f2s ( slashes: &str) -> String {
        slashes.replace("/", ".")
    }


    pub fn get_class(&mut self, module: &str, class: &str) -> Vec<u8> {
        let path = Self::resolve_path(module, class);
        self.image.read_file(&path).unwrap()
    }
}