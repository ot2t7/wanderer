use std::{fs::{read_dir, read}, path::Path, collections::HashMap, ffi::OsStr};
use std::io::Error;

#[derive(Debug)]
pub enum VmStringSeekError {
    IoError(Error)
}

const STR_DIR: &str = "lua";

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
                println!("{}", entry_path.display())
            }
        }
    }

    return Ok(());
}


pub fn load_vm_strings() -> Result<HashMap<String, String>, VmStringSeekError> {
    let path = Path::new(STR_DIR);
    let mut vm_strings: HashMap<String, String> = HashMap::new();

    scan_dir(path, &mut vm_strings)?;

    return Ok(vm_strings);
}