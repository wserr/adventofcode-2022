// Use BTreeMap<&str, TreeValue>
// Where &str is the full path
// Directories end with a trailing slash

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Tree {
    pub items: BTreeMap<String, TreeValue>,
}

#[derive(Debug)]
pub enum TreeValue {
    Folder,
    File(usize),
}

impl TreeQuerier for Tree {
    fn calculate_file_size_in_folder(&self, folder_path: &str) -> Result<usize, &'static str> {
        let files = self.get_all_files_from_folder(folder_path)?;

        let result = files.into_iter().fold(0, |mut acc, v| {
            if let &TreeValue::File(value) = v {
                acc += value;
            }
            acc
        });
        Ok(result)
    }

    fn get_all_files_from_folder(
        &self,
        folder_path: &str,
    ) -> Result<Vec<&TreeValue>, &'static str> {
        match self.items.get(folder_path) {
            None => Err("Folder not found"),
            Some(_) => {
                let result: Vec<&TreeValue> = self.items.iter().fold(Vec::new(), |mut acc, v| {
                    if v.0.starts_with(folder_path) && !v.0.ends_with("/") {
                        acc.push(&v.1);
                    }
                    acc
                });
                Ok(result)
            }
        }
    }
}

impl TreeCommander for Tree {
    fn get_or_create_folder(&mut self, folder_path: String) -> Result<String, &'static str> {
        match self.items.get(folder_path.as_str()) {
            None => {
                self.items.insert(folder_path.clone(), TreeValue::Folder);
                Ok(String::from(folder_path))
            }
            Some(_) => Ok(String::from(folder_path)),
        }
    }

    fn create_file(
        &mut self,
        folder_path: &str,
        file_name: &str,
        file_size: usize,
    ) -> Result<String, &'static str> {
        let mut full_path_name = folder_path.to_string();
        full_path_name.push_str(file_name);
        match self.items.get(full_path_name.as_str()) {
            None => {
                self.items
                    .insert(full_path_name.clone(), TreeValue::File(file_size));
                Ok(full_path_name)
            }
            Some(_x) => Err("File already exists"),
        }
    }
}

pub trait TreeQuerier {
    // Fetches all files within a folder
    // Returns an error if folder was not found
    fn get_all_files_from_folder(&self, folder_path: &str)
        -> Result<Vec<&TreeValue>, &'static str>;

    fn calculate_file_size_in_folder(&self, folder_path: &str) -> Result<usize, &'static str>;
}

pub trait TreeCommander {
    // Gets or creates a new folder with specified path. The name of the new folder is returned.
    fn get_or_create_folder(&mut self, folder_path: String) -> Result<String, &'static str>;

    // Creates a new file if it does not exist already.
    // Returns the path of the newly created file.
    fn create_file(
        &mut self,
        folder_path: &str,
        file_name: &str,
        file_size: usize,
    ) -> Result<String, &'static str>;
}

#[test]
fn should_find_item_in_tree() {
    let items = BTreeMap::from([
        ("/a/b/c".to_string(), TreeValue::File(25)),
        ("/a/b/".to_string(), TreeValue::Folder),
        ("/a/b/d".to_string(), TreeValue::File(10)),
        ("/a/b/e".to_string(), TreeValue::File(5)),
    ]);

    let tree = Tree { items };

    let count = tree.get_all_files_from_folder("/a/b/").unwrap().len();
    assert_eq!(3, count);
}

#[test]
fn should_create_new_file() {
    let items = BTreeMap::from([
        ("/a/b/c".to_string(), TreeValue::File(25)),
        ("/a/b/".to_string(), TreeValue::Folder),
        ("/a/b/d".to_string(), TreeValue::File(10)),
        ("/a/b/e".to_string(), TreeValue::File(5)),
    ]);
    let new_file_name = "test.txt";
    let new_file_folder = "/a/b/";

    let mut tree = Tree { items };
    let created_file = tree
        .create_file(new_file_folder, new_file_name, 100)
        .unwrap();

    let count = tree.get_all_files_from_folder("/a/b/").unwrap().len();

    assert_eq!("/a/b/test.txt".to_string(), created_file);
    assert_eq!(4, count);
    assert_eq!(140, tree.calculate_file_size_in_folder("/a/b/").unwrap());
}
