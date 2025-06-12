use mlua::Lua;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Thanks to https://github.com/benwiley4000/pico8-to-lua
pub fn preprocess_pico8_lua_bytes(
    input: &Vec<u8>,
    preprocessor_path: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let code = String::from_utf8_lossy(&input.clone()).into_owned();
    let p = Path::new(preprocessor_path);
    let preprocessor_code = match fs::read_to_string(p) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let preprocessor_lua = Lua::new();
    preprocessor_lua.load(&preprocessor_code).exec()?;

    let globals = preprocessor_lua.globals();

    let patch_fn: mlua::Function = globals.get("patch_lua")?;

    let result: String = patch_fn.call(code)?;

    Ok(result.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_pico8_lua_bytes() {
        let ori_code = fs::read("../examples/ppg-1.lua").unwrap();

        let result =
            preprocess_pico8_lua_bytes(&ori_code, "pico8_patcher/pico8-to-lua.lua").unwrap();

        let lua = Lua::new();
        lua.load(&result).exec().unwrap();
    }
}
