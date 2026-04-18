use directories::ProjectDirs;
use std::path::Path;
// check if database has been created
//
// if not create database and all necessary tables
// a way to check if the database file already exists

// check if database file exists

pub fn database_exists() -> Option<bool> {
    let fname = "/ppmc.sqlite3";
    if let Some(proj_dirs) = ProjectDirs::from("","","ppmc"){
        let data_dir = proj_dirs.config_dir().to_str().unwrap();
        let path = Path::new(&(data_dir.to_owned() + fname)).exists();
        println!("{path}");

        if path {
            return Some(true);
        } else{
            // this could mean two things
            // either file does not exist, or we may not have the rights to complete this operation
            // if the latter, don't attempt to create fname at path
            return Some(false);
        }
    }

    // we failed to retrieve users project directory
    None
}
