use async_compat::CompatExt;
use mlua::{ExternalError, ExternalResult, Lua, Result, Table};

async fn lookup_host(_: &Lua, host: String) -> Result<String> {
    tokio::net::lookup_host(host)
        .compat()
        .await
        .to_lua_err()?
        .next()
        .map(|addr| addr.to_string())
        .ok_or_else(|| "failed to lookup host".to_lua_err())
}

#[mlua::lua_module]
fn module(lua: &Lua) -> Result<Table> {
    let lookup_host = lua.create_async_function(lookup_host)?;
    lua.create_table_from([("lookup_host", lookup_host)])
}
