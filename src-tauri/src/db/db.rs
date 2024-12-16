use std::fs;
use std::path::Path;

// Init db
pub fn init_db() {
    if !db_exist(){
        create_db_file();
    }
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_parent_dir = Path::new(&db_path).parent().unwrap();

    // Create parent dirs if they don't exist
    if !db_parent_dir.exists() {
        fs::create_dir_all(db_parent_dir).unwrap()
    }

    fs::File::create(db_path).unwrap();
}

fn db_exist() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/taipu/database.sqlite"
}