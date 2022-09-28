use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    time::Instant,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tauri::regex::Regex;

use std::sync::Mutex;
pub struct InnerState {
    pub store: Mutex<RoaList>,
}

lazy_static! {
    static ref GENERAL_RE: Regex =
        Regex::new(r"(?m)(^\[general](?:\r?\n(?:[^\[\r\n].*)?)*)").unwrap();
    static ref KEYPAIR_RE: Regex = Regex::new(r#"(?m)(.+?)\s*=\s*"(.*)""#).unwrap();
}

macro_rules! GET_CONFIG {
    ($var: literal, $t:expr) => {
        ItemConfig::get_value_from_config($var, $t)
    };
}

macro_rules! GET_CONFIG_PARSED {
    ($var: literal, $t:expr) => {
        match GET_CONFIG!($var, $t).parse() {
            Ok(value) => value,
            Err(_) => 0,
        }
    };
}

macro_rules! GET_CONFIG_BOOL {
    ($var: literal, $t:expr) => {
        match GET_CONFIG!($var, $t).parse() {
            Ok(value) => value,
            Err(_) => false,
        }
    };
}

struct ItemConfig {}
impl ItemConfig {
    fn get_value_from_config<'a>(var: &'a str, text: &'a str) -> String {
        let first_match = GENERAL_RE.captures(text).unwrap().get(1).unwrap().as_str();
        //println!("{}", first_match);

        // result will be an iterator over tuples containing the start and end indices for each match in the string
        let result = KEYPAIR_RE.captures_iter(first_match);

        for mat in result {
            if var == mat.get(1).unwrap().as_str() {
                return mat.get(2).unwrap().as_str().to_string();
            }
        }

        "".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    icon: String,
    name: String,
    item_type: u8,
    url: u32,
    author: String,
    description: String,
    version: (u8, u8),
    finished: bool,
    bg_color: String,
    plural: bool,
    root: String,
}
impl Item {
    pub fn new() -> Item {
        Item {
            icon: String::new(),
            name: String::new(),
            item_type: 0,
            url: 0,
            author: String::new(),
            description: String::new(),
            version: (0, 0),
            finished: false,
            bg_color: String::new(),
            plural: false,
            root: String::new(),
        }
    }
    pub fn read(&mut self, path: &str) {
        let now = Instant::now();

        let _path = Path::new(path).join("config.ini");
        let file = match File::open(_path) {
            Ok(file) => file,
            Err(error) => {
                println!("Error opening file: {} in {}", error, path);
                return;
            }
        };
        let mut buffer = BufReader::new(file);
        let mut text = String::new();
        if !buffer.read_to_string(&mut text).is_ok() {
            return;
        }

        // Get every value from the config file
        self.icon = GET_CONFIG!("icon", &text);
        self.name = GET_CONFIG!("name", &text);
        self.item_type = GET_CONFIG_PARSED!("type", &text);
        self.url = GET_CONFIG_PARSED!("url", &text);
        self.author = GET_CONFIG!("author", &text);
        self.description = GET_CONFIG!("description", &text);

        let major_version: u8 = GET_CONFIG_PARSED!("major_version", &text);
        let minor_version: u8 = GET_CONFIG_PARSED!("minor_version", &text);
        self.version = (major_version, minor_version);

        self.finished = GET_CONFIG_BOOL!("finished", &text);
        self.bg_color = GET_CONFIG!("bg_color", &text);
        self.plural = GET_CONFIG_BOOL!("plural", &text);
        self.root = GET_CONFIG!("root", &text);

        let elapsed = now.elapsed();
        println!("Read config file in {} ms", elapsed.as_millis());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    name: String,
    items: Vec<Item>,
}
impl Category {
    pub fn new() -> Category {
        Category {
            name: String::new(),
            items: Vec::new(),
        }
    }
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharactersTree {
    item_type: u8,
    categories: Vec<Category>,
}
impl CharactersTree {
    pub fn new() -> CharactersTree {
        CharactersTree {
            item_type: 0,
            categories: Vec::new(),
        }
    }
    pub fn add_category(&mut self, category: Category) {
        self.categories.push(category);
    }
    pub fn get_category(&self, index: usize) -> &Category {
        &self.categories[index]
    }
    /// Populate the [`CharactersTree`] given two arrays.
    /// One array contains each character's file path and the other contains a tuple with the category name and its offset.
    pub fn set_by_paths(&mut self, paths: &[String], categories: &Vec<(String, u16)>) {
        let mut category = Category::new();
        let mut i = 0;
        let mut j = 1; //Look ahead

        //Include all characters that are not in a category
        if categories[0].1 != 0 {
            category.name = "Free".to_string();
            for i in 0..categories[0].1 {
                let mut item = Item::new();
                item.read(paths[i as usize].as_str());
                category.add_item(item);
            }
            self.add_category(category);
        }

        //Include all characters that are in a category
        for cat_info in categories {
            let next_cat_pos = if j < categories.len() {
                categories[j].1
            } else {
                paths.len() as u16
            };

            category = Category::new();
            category.name = cat_info.0.clone();
            for cha_i in &paths[i as usize..next_cat_pos as usize] {
                let mut item = Item::new();
                item.read(cha_i.as_str());
                category.add_item(item);
            }
            self.add_category(category);

            i = next_cat_pos;
            j += 1;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericTree {
    item_type: u8,
    items: Vec<Item>,
}
impl GenericTree {
    /// Creates a new [`GenericTree`].
    pub fn new(item_type: u8) -> GenericTree {
        GenericTree {
            item_type,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
    pub fn set_from_path(&mut self, paths: &[String]) {
        for path in paths {
            let mut item = Item::new();
            item.read(path.as_str());
            self.add_item(item);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoaList {
    char_tree: CharactersTree,
    buddy_tree: GenericTree,
    skin_tree: GenericTree,
    stage_tree: GenericTree,
}
impl RoaList {
    pub fn new() -> RoaList {
        RoaList {
            char_tree: CharactersTree::new(),
            buddy_tree: GenericTree::new(1),
            skin_tree: GenericTree::new(0),
            stage_tree: GenericTree::new(2),
        }
    }

    /// Populate the [`CharactersTree`] given two arrays.
    /// One array contains each character's file path and the other contains a tuple with the category name and its offset.
    pub fn add_char_tree_from_paths(&mut self, paths: &[String], categories: &Vec<(String, u16)>) {
        self.char_tree.set_by_paths(paths, categories);
    }
    pub fn add_buddy_tree_from_paths(&mut self, paths: &[String]) {
        self.buddy_tree.set_from_path(paths);
    }
    pub fn add_skin_tree_from_paths(&mut self, paths: &[String]) {
        self.skin_tree.set_from_path(paths);
    }
    pub fn add_stage_tree_from_paths(&mut self, paths: &[String]) {
        self.stage_tree.set_from_path(paths);
    }

    pub fn get_char_tree(&self) -> CharactersTree {
        self.char_tree.clone()
    }
    pub fn cat_mut(&mut self) -> &mut Vec<Category> {
        self.char_tree.categories.as_mut()
    }
    pub fn get_buddy_tree(&self) -> GenericTree {
        self.buddy_tree.clone()
    }
    pub fn get_skin_tree(&self) -> GenericTree {
        self.skin_tree.clone()
    }
    pub fn get_stage_tree(&self) -> GenericTree {
        self.stage_tree.clone()
    }
    pub fn get(&self) -> &RoaList {
        self
    }
    pub fn get_len(&self) -> usize {
        self.char_tree.categories.len()
            + self.buddy_tree.items.len()
            + self.skin_tree.items.len()
            + self.stage_tree.items.len()
    }

    /// Move elements
    pub fn move_categories(&mut self, from: Vec<usize>, to: Vec<usize>) {
        let cats = self.cat_mut();
        let mut removed_els: Vec<Category> = Vec::new();

        let mut j:usize = 0;
        for i in from {
            removed_els.push(cats.remove(i - j));
            j += 1;
        }

        j = 0;
        for element in removed_els {
            cats.insert(to[j], element);
            j += 1;
        }
        
    }
}
