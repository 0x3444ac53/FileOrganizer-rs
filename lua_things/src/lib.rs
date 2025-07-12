use mlua::prelude::*;

pub fn register_api_function<F, A, R>(
    lua: &Lua,
    table: LuaTable,
    name: &str,
    func: F,
) -> mlua::Result<()>
where
    F: 'static + Send + Fn(&Lua, A) -> mlua::Result<R>,
    R: IntoLuaMulti,
    A: FromLuaMulti,
{
    let lua_func = lua.create_function(func)?;
    table.set(name, lua_func)
}

pub unsafe fn compile_fennel(fnl: String) -> Result<String, LuaError> {
    let fennel_env = Lua::unsafe_new();
    let require: LuaFunction = fennel_env.globals().get("require")?;
    fennel_env
        .load(include_str!("../../assets/lua/fennel.lua"))
        .set_name("fennel.lua")
        .exec()?;
    let fennel: LuaTable = require.call("fennel")?;
    let fennel_compiler: LuaFunction = fennel.get("compileString")?;
    let compiled_to_lua: String = fennel_compiler.call(fnl)?;
    Ok(compiled_to_lua)
}
