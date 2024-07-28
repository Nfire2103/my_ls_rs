use super::Entry;

pub struct File {
    name: String,
}

impl File {
    pub fn new(path: &str) -> Self {
        Self {
            name: path.to_string(),
        }
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
