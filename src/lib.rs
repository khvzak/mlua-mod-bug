#[cfg(test)]
mod tests {
    use mlua::{chunk, Lua, Result};

    #[test]
    fn it_works() -> Result<()> {
        let lua = make_lua()?;
        lua.load(chunk! {
            local module = require("module")
            local co = coroutine.create(module.lookup_host)

            while true do
                local ok, res = coroutine.resume(co, "fb.com:443")
                if not ok or type(res) ~= "userdata" then
                    print(res)
                    break
                end
            end
        })
        .exec()
    }

    #[test]
    fn it_fails() -> Result<()> {
        let lua = make_lua()?;
        lua.load(chunk! {
            local module = require("module")
            local co = coroutine.create(module.lookup_host)

            while true do
                local ok, res = coroutine.resume(co, "wrong addr")
                if not ok or type(res) ~= "userdata" then
                    print(res)
                    break
                end
            end
        })
        .exec()
    }

    fn make_lua() -> Result<Lua> {
        let (dylib_path, dylib_ext, separator);
        if cfg!(target_os = "macos") {
            dylib_path = std::env::var("DYLD_FALLBACK_LIBRARY_PATH").unwrap();
            dylib_ext = "dylib";
            separator = ":";
        } else if cfg!(target_os = "linux") {
            dylib_path = std::env::var("LD_LIBRARY_PATH").unwrap();
            dylib_ext = "so";
            separator = ":";
        } else {
            panic!("unknown target os");
        };

        let cpath = dylib_path
            .split(separator)
            .take(3)
            .map(|p| {
                let mut path = std::path::PathBuf::from(p);
                path.push(format!("lib?.{}", dylib_ext));
                path.to_str().unwrap().to_owned()
            })
            .collect::<Vec<_>>()
            .join(";");

        let lua = unsafe { Lua::unsafe_new() }; // To be able to load C modules
        lua.load(&format!("package.cpath = \"{}\"", cpath)).exec()?;
        Ok(lua)
    }
}
