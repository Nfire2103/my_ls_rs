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

    fn get_name(&self) -> &String {
        return &self.name;
    }
}
