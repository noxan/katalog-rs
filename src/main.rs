use epub::doc::EpubDoc as EpubDocument;
use std::path::PathBuf;

struct Library {
    name: String,
    path: PathBuf,
}

impl Library {
    fn new(name: String, path: PathBuf) -> Library {
        Library { name, path }
    }

    fn init(&self) {
        if !self.path.exists() {
            std::fs::create_dir(&self.path).unwrap();
        }

        let files = std::fs::read_dir(&self.path).unwrap();
        for file in files {
            let path = file.unwrap().path();
            println!("{:?}", path.display());
        }

        let files = std::fs::read_dir(&self.path).unwrap();
        let file = files.into_iter().next().unwrap().unwrap().path();
        println!("{:?}", file);
        let mut epub = EpubDocument::new(file).unwrap();
        println!("{:?}", epub.metadata);
        println!("{:?}", epub.get_cover_id().unwrap())
    }
}

fn main() {
    let root = home::home_dir().unwrap().join("Books");
    let library = Library::new(String::from("Default"), root);

    println!("Library '{}' at {}", library.name, library.path.display());
    library.init();
}
