use std::path::{Path, PathBuf};

mod app;
mod lua;
mod messages;
mod interactive;

fn aloha() {
    messages::getmsg("title_msg").print();
    messages::getmsg("madeby_msg").print_newlines(2);
}

fn find_wanted_mods() -> Vec<PathBuf> {

    match app::PROGRAM_SETTINGS.is_interactive {
        true => interactive::get_wanted_mods(),
        false => unimplemented!()
    }

}

fn main() {

    // I am sorry if this is bad rust code, I don't write a lot of rust
    // This was originally gonna be written in C#, but I couldn't find an
    // Up-to-date Lua5.1 binding to C# (only 5.4)
    // I did write a quick prototype in C++ but I have no faith in that version
    // At this rate might as well learn Go

    aloha();
    
    // Init the lazy static PROGRAM_SETTINGS
    let _ = app::PROGRAM_SETTINGS.is_interactive;

    let context = lua::init_context();

    lua::init_dorhud_context(&context);

    let got_mods = find_wanted_mods();
    let mods_ref: Vec<&Path> = got_mods.iter().map(|p| p.as_ref()).collect();

    lua::create_mod_txts(&context, &mods_ref);

}
