use super::Entry;

pub struct File {
    name: String,
}

impl File {
    pub fn new(path: String) -> Self {
        Self { name: path }
    }
}

impl Entry for File {
    fn display(&self) {
        print!("{}  ", self.name);
    }
}
