//! Lua sandbox and pipeline.

pub mod sandbox;
pub mod pipeline;

use mlua::Lua;

/// Closed sandbox: only TABLE, STRING, MATH, UTF8.
/// `require` is restricted to /etc/polaros.
pub fn setup_sandbox(lua: &Lua) -> mlua::Result<()> {
    lua.load_std_libs(
        mlua::StdLib::TABLE
            | mlua::StdLib::STRING
            | mlua::StdLib::MATH
            | mlua::StdLib::UTF8,
    )?;

    let require = lua.create_function(|lua, name: String| {
        let path = format!("/etc/polaros/{}.lua", name.replace('.', "/"));
        let src = std::fs::read_to_string(&path)
            .map_err(|e| mlua::Error::RuntimeError(format!("require {name}: {e}")))?;
        let chunk = lua.load(&src).set_name(&path);
        let module: mlua::Value = chunk.eval()?;
        lua.globals().set(name.as_str(), module.clone())?;
        Ok(module)
    })?;
    lua.globals().set("require", require)?;

    Ok(())
}

/// Call `robot.tick(ctx)` and return its JSON result.
pub fn call_tick(lua: &Lua, ctx: serde_json::Value) -> mlua::Result<serde_json::Value> {
    let robot: mlua::Table = lua.load("return require('robot')").eval()?;
    let ctx_table = lua.to_value(&ctx)?;
    let out: mlua::Value = robot.get::<mlua::Function>("tick")?.call(ctx_table)?;
    let json: serde_json::Value = lua.from_value(out)?;
    Ok(json)
}