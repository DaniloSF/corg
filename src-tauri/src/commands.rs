use std::{fs::File, io::BufReader};

use steam_workshop_api::Workshop;
use tauri::State;

use crate::{
    parser::{parse_categories_section, parse_order_section},
    r_lock,
    types::{CharactersTree, InnerState, RoaList},
};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
pub async fn get_characters_tree(state: State<'_, InnerState>) -> Result<CharactersTree, String> {
    let rl = state.store.lock().unwrap();

    Ok(rl.get_char_tree())
}

#[tauri::command]
pub fn get_roa_list(state: State<InnerState>) -> Result<String, &str> {
    let rl: RoaList = r_lock!(state).clone();
    Ok(serde_json::to_string(&rl).unwrap())
}

#[tauri::command]
pub fn move_categories(
    from_c: Vec<usize>,
    to_c: Vec<usize>,
    state: State<'_, InnerState>,
) -> Result<String, &str> {
    let mut rl = r_lock!(state);
    rl.move_categories(from_c, to_c);
    Ok(serde_json::to_string(&rl.clone()).unwrap())
}

#[tauri::command]
pub async fn read_roa_file(
    order_path: &str,
    categories_path: &str,
    state: State<'_, InnerState>,
) -> Result<&'static str, &'static str> {
    let mut roa_list = r_lock!(state);

    if roa_list.get_len() > 0 {
        return Err("Already loaded");
    }
    println!("Reading file: {}", order_path);

    let order_file = match File::open(order_path) {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {}", error),
    };
    let categories_file = match File::open(categories_path) {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {}", error),
    };
    let mut order_buffer: BufReader<&File> = BufReader::new(&order_file);

    println!("Parsing order file and categories file");
    let characters_path = parse_order_section(&mut order_buffer);
    let categories = parse_categories_section(&categories_file);
    println!("Parsing Buddies");
    let buddies_path = parse_order_section(&mut order_buffer);
    println!("Parsing Stages");
    let stages_path = parse_order_section(&mut order_buffer);
    println!("Parsing Skins");
    let skins_path = parse_order_section(&mut order_buffer);

    roa_list.add_char_tree_from_paths(&characters_path, &categories);
    roa_list.add_buddy_tree_from_paths(&buddies_path);
    roa_list.add_stage_tree_from_paths(&stages_path);
    roa_list.add_skin_tree_from_paths(&skins_path);

    println!("Done parsing");
    Ok("All good!")
}
