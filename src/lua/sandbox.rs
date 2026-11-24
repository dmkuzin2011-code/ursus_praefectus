//! Closed Lua sandbox helpers.

use mlua::Lua;

/// Register a Rust hook into the Lua state.
pub fn register_hook<F>(lua: &Lua, name: &str, f: F) -> mlua::Result<()>
where
    F: Fn(&Lua, mlua::Value) -> mlua::Result<mlua::Value> + 'static,
{
    let func = lua.create_function(f)?;
    lua.globals().set(name, func)?;
    Ok(())
}