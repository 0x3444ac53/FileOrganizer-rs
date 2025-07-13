use crate::helpers::register_api_function;
use mlua::prelude::*;
use std::fs::{copy, rename};
use std::io::copy;
use std::path::Path;

pub fn build_fs_module(lua: Lua) -> LuaTable {}

fn api_copy(_: LuaTable, from: String, to: String) -> Result<u64> {
    copy
}
