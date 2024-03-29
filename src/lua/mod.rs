use std::fs::{self};
use std::path::Path;

use mlua::prelude::*;
use mlua::Lua;

use crate::app;
use crate::messages;

mod fake_dahm_context;
mod no_op_table;

pub fn create_mod_txts(context: &Lua, basemod_paths: &[&Path]) {

    for path in basemod_paths {
        
        let basemod_exec_result = context.load( *path ).eval::<LuaTable>();
        
        let mod_tbl = match basemod_exec_result {
            Ok(t) => t,
            Err(e) => app::error("exec_basemod_failure", Some(&e))
        };

        let json = match serde_json::to_string_pretty(&mod_tbl).map_err(mlua::Error::external) {
            Ok(s) => s,
            Err(e) => app::error("serde_serialization_failure", Some(&e))
        };

        let mut mod_txt_path = (*path).clone().to_owned();
        mod_txt_path.pop();
        mod_txt_path.push("mod.txt");

        _ = fs::write(mod_txt_path, json);

    }

}

pub fn init_context() -> Lua {

    messages::getmsg("init_lua_context").print();

    let lua_context: Lua;
    unsafe {
        lua_context = mlua::Lua::unsafe_new();

        let context_load_result = lua_context.load_from_std_lib(
            LuaStdLib::DEBUG |
            LuaStdLib::MATH |
            LuaStdLib::STRING |
            LuaStdLib::TABLE
        );

        match context_load_result {
            Ok(_) => (),
            Err(e) => {
                app::error(
                    "create_context_fail",
                    Some(&e)
                );
            }
        };
    }

    lua_context

}

pub fn init_dorhud_context(context: &Lua) {
    messages::getmsg("init_dahm_context").print();

    let no_op_table = no_op_table::get_code(&context);
    let fake_dahm_context = fake_dahm_context::get_code(&context);

    let no_op_exec_result = no_op_table.exec();
    match no_op_exec_result {
        Ok(_) => (),
        Err(e) => {
            app::error(
                "builtin_lua_fail",
                Some(&e)
            );
        }
    };

    let fake_dahm_exec_result = fake_dahm_context.exec();
    match fake_dahm_exec_result {
        Ok(_) => (),
        Err(e) => {
            app::error(
                "builtin_lua_fail",
                Some(&e)
            );
        }
    };
}