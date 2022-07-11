use std::{fs::{read_dir, read}, path::Path, collections::HashMap, ffi::OsStr};
use std::io::Error;

use lazy_static::lazy_static;
use regex::Regex;

const STR_DIR: &str = "lua";

lazy_static! {
    static ref SEARCH: Regex = Regex::new("-- @BEGIN (.+)@").unwrap();
}

#[derive(Debug)]
pub enum VmStringSeekError {
    IoError(Error)
}

/// Crawls a directory, logging the hashmap of vm strings.
fn scan_dir(path: &Path, log: &mut HashMap<String, String>) -> Result<(), VmStringSeekError> {
    for entry in read_dir(path)
        .map_err(|e| VmStringSeekError::IoError(e))?
    {
        let entry = entry
            .map_err(|e| VmStringSeekError::IoError(e))?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            // A subdirectory that needs to be scanned
            scan_dir(&entry_path, log)?;
        } else {
            if entry_path.extension() == Some(OsStr::new("lua")) {
                // We've found a valid lua file 
                let contents = read(entry_path)
                    .map_err(|e| VmStringSeekError::IoError(e))?;
                let source = String::from_utf8_lossy(&contents);
                for cap in SEARCH.captures_iter(&source) {
                    let section_name = &cap[1];
                    let section_regex = Regex::new(format!(
                        "-- @BEGIN {}@([\\S\\n\\t\\v ]+)-- @END {}@",
                        section_name,
                        section_name
                    ).as_str()).unwrap();
                    match section_regex.captures_iter(&source).next() {
                        Some(section_data) => {
                            let section_content = &section_data[1];
                            log.insert(section_name.to_string(), section_content.to_string());
                        }
                        None => {} // TODO: Log this somewhere in the future to show a broken vm string
                    };
                }
            }
        }
    }
    
    return Ok(());
}

/// Return a hashmap where the index is the section name of a vm string and the return
/// is the actual content of the vm string
pub fn load_vm_strings() -> Result<HashMap<String, String>, VmStringSeekError> {
    let path = Path::new(STR_DIR);
    let mut vm_strings: HashMap<String, String> = HashMap::new();

    scan_dir(path, &mut vm_strings)?;

    return Ok(vm_strings);
}