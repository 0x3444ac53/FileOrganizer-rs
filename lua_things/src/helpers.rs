use mlua::prelude::*;

pub fn register_api_function<F, A, R>(
    lua: &Lua,
    table: &LuaTable,
    name: &str,
    func: F,
) -> mlua::Result<()>
where
    F: 'static + Send + Fn(&LuaTable, A) -> mlua::Result<R>,
    R: IntoLuaMulti,
    A: FromLuaMulti,
{
    let lua_func = lua.create_function(func)?;
    table.set(name, lua_func)
}
