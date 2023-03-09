use mlua::prelude::*;

pub mod document;

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    Ok(exports)
}
