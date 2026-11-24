//! Lua sandbox and pipeline.

pub mod sandbox;
pub mod pipeline;

use mlua::Lua;
use serde_json::Value;

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

/// Call `robot.tick(ctx)` and return its JSON result (backward compatible).
pub fn call_tick(lua: &Lua, ctx: Value) -> mlua::Result<Value> {
    let robot: mlua::Table = lua.load("return require('robot')").eval()?;
    let ctx_table = lua.to_value(&ctx)?;
    let out: mlua::Value = robot.get::<mlua::Function>("tick")?.call(ctx_table)?;
    let json: Value = lua.from_value(out)?;
    Ok(json)
}

/// Call a robot script by name and return its JSON result.
/// This is the new style: call_robot(lua, script_name, ctx)
pub fn call_robot(lua: &Lua, script_name: &str, ctx: Value) -> mlua::Result<Value> {
    let script: mlua::Table = lua.load(&format!("return require('{}')", script_name)).eval()?;
    let ctx_table = lua.to_value(&ctx)?;
    let out: mlua::Value = script.get::<mlua::Function>("tick")?.call(ctx_table)?;
    let json: Value = lua.from_value(out)?;
    Ok(json)
}

/// Call a robot script with a specific action and parameters.
/// This is the new style: call_robot_with_action(lua, script_name, action, params)
pub fn call_robot_with_action(
    lua: &Lua,
    script_name: &str,
    action: &str,
    params: Value,
) -> mlua::Result<Value> {
    let script: mlua::Table = lua.load(&format!("return require('{}')", script_name)).eval()?;

    // Call the action function with params
    let out: mlua::Value = script
        .get::<mlua::Function>(action)?
        .call(params)?;

    let json: Value = lua.from_value(out)?;
    Ok(json)
}

/// Safe system utilities module for Lua scripts.
/// Provides read-only access to system state and limited control actions.
pub fn register_sys_module(lua: &Lua) -> mlua::Result<()> {
    // sys.info - system information (read-only)
    let sys_info = lua.create_function(|lua, _params: ()| -> mlua::Result<Value> {
        // Return basic system info - this is populated by the host
        Ok(Value::Object(serde_json::json!({
            "polaros_version": env!("CARGO_PKG_VERSION"),
            "hostname": "ursus-host",
            "mode": "robot"
        })))
    })?;
    lua.globals().set("sys", lua.create_table()?)?;
    lua.globals().get::<mlua::Table>("sys")?.set("info", sys_info)?;

    // sys.service_status - check service status
    let sys_service_status = lua.create_function(|lua, service_name: String| -> mlua::Result<Value> {
        Ok(Value::Object(serde_json::json!({
            "status": "stopped",
            "service": service_name
        })))
    })?;
    lua.globals()
        .get::<mlua::Table>("sys")?
        .set("service_status", sys_service_status)?;

    // sys.service_restart - request service restart (limited capability)
    let sys_service_restart = lua.create_function(|lua, service_name: String| -> mlua::Result<Value> {
        // This is a limited operation - in production, this would
        // communicate with the Rust host to request a restart
        Ok(Value::Object(serde_json::json!({
            "requested": true,
            "service": service_name,
            "note": "restart requested via sys.service_restart"
        })))
    })?;
    lua.globals()
        .get::<mlua::Table>("sys")?
        .set("service_restart", sys_service_restart)?;

    Ok(())
}