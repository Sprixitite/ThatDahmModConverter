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

    aloha();
    
    // Init the lazy static PROGRAM_SETTINGS
    let _ = app::PROGRAM_SETTINGS.is_interactive;

    let context = lua::init_context();

    lua::init_dorhud_context(&context);

    let got_mods = find_wanted_mods();
    let mods_ref: Vec<&Path> = got_mods.iter().map(|p| p.as_ref()).collect();

    lua::create_mod_txts(&context, &mods_ref);

}
