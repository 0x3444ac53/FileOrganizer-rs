use lua_things::*;
use mlua::prelude::*;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

fn main() -> Result<(), ()> {
    todo!()
}

fn find_config(path: &Path) -> Result<PathBuf, std::io::Error> {
    path.read_dir()?
        .filter_map(|entry| match entry {
            Ok(file) => match file.file_name().to_str() {
                Some("config.lua") | Some("config.fnl") => Some(file.path()),
                _ => None,
            },
            Err(_) => None,
        })
        .next()
        .ok_or_else(|| std::io::Error::other("Cunts"))
}

fn load_lua(path: &Path) -> mlua::Result<(Lua, LuaTable)> {
    let config = find_config(path)?;
    match config.file_name().unwrap().to_str() {
        Some("config.fnl") => {
            let compiled_to_lua = unsafe { compile_fennel(fs::read_to_string(config)?)? };
            println!("{}", compiled_to_lua);
            let lua = Lua::new();
            let module: LuaTable = lua.load(&compiled_to_lua).eval()?;
            Ok((lua, module))
        }
        Some("config.lua") => {
            let lua = Lua::new();
            let module = lua.load(config).eval()?;
            Ok((lua, module))
        }
        None => Err(LuaError::RuntimeError(format!(
            "config.fnl not found in {}",
            path.to_str().unwrap_or("bad bad bad"),
        ))),
        _ => Err(LuaError::RuntimeError(
            "Somethign broke bad. like really bad".into(),
        )),
    }
}

#[test]
fn test_loading_lua() -> mlua::Result<()> {
    let path = Path::new("/Users/Nora/repos/FileOrganizer-rs/test/basic loading/");
    let (lua, module) = load_lua(path)?;
    let fun: LuaFunction = module.get("main")?;
    let val = fun.call::<bool>(())?;
    assert_eq!(true, val);
    Ok(())
}

#[test]
fn testing_lua_ffi() -> mlua::Result<()> {
    let path = Path::new("/Users/Nora/repos/FileOrganizer-rs/ffi/");
    let (lua, module) = load_lua(path)?;
    let api_module = lua.create_table()?;
    register_api_function(&lua, api_module, "mv", |_, (from, to): (String, String)| {
        std::fs::rename(&from, &to).map_err(mlua::ExternalError::into_lua_err)
    })
}
