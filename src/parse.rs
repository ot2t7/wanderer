use mlua::Lua;
use mlua::Error;

use std::io::Cursor;
use std::io::Read;

use super::instr::Function;

#[derive(Debug)]
pub enum ParserError {
    LuaError(Error),
    NoBytesLeft(String)
}

#[macro_export]
macro_rules! consume {
    ($x: expr, $y: expr) => {
        {
            let mut buff: [u8 ; $y] = [0 ; $y];
            match $x.read(&mut buff) {
                Ok(num_bytes_read) => {
                    if num_bytes_read < $y {
                        Err(ParserError::NoBytesLeft(format!("attempted to read {} bytes, only {} bytes left", $y, num_bytes_read)))
                    }
                    Ok(buff)
                }
            }
        }
    };
}

/// Take the original lua source code, and compile it into
/// Lua 5.1 bytecode.
fn to_bytecode(src: &String) -> Result<Cursor<Vec<u8>>, Error> {
    let state = Lua::new();
    let func = state.load(src).into_function()?;
    return Ok(Cursor::new(func.dump(true)));
}


/// Deserialize a chunk of lua bytecode
pub fn deserialize(src: &String) -> Result<Function, ParserError> {
    let mut bytecode = to_bytecode(src)
        .map_err(|e| ParserError::LuaError(e))?;
    
    // Read the lua header block

    // Header Signature, should be 0x1B4C7561
    loop {
        println!("{:?}", consume!(bytecode, 4));
        std::thread::sleep_ms(300);
    }

    todo!()
    
}