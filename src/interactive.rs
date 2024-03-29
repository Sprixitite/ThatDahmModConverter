use std::{collections::HashMap, path::{Path, PathBuf}};

use colored::Colorize;
use dialoguer::{Confirm, MultiSelect};

use crate::{app, messages};

fn get_yesno<'a>() -> Confirm<'a> {
    Confirm::new().report(false)
}

fn get_multi<'a>() -> MultiSelect<'a> {
    MultiSelect::new().report(false).clear(true)
}

fn find_local_basemod() -> Option<PathBuf> {

    let base_mod_definition = Path::new("base.lua");
    match base_mod_definition.exists() {
        true => {
            // base.lua in current working directory
            messages::getmsg("legacy_basemod_nearby").print();
            Some(base_mod_definition.to_owned())
        },
        false => {
            // no base.lua in cwd
            messages::getmsg("legacy_basemod_not_nearby").print();
            None
        }
    }

}

fn get_subdir_mods(cwd: &Path) -> HashMap<String, PathBuf> {

    let mut found = HashMap::new();
    let yesno = get_yesno();
    yesno.with_prompt(messages::getmsg("legacy_has_modtxt").msg());

    // Get iterator here so we can handle errors with fancy colours :)
    let dir_iterator = match cwd.read_dir() {
        Ok(i) => i,
        Err(e) => app::error("cwd_read_failure", Some(&e))
    };

    for entry_result in dir_iterator {

        if entry_result.is_err() {
            messages::getmsg("fail_read_direntry").print_args(&[&entry_result.unwrap_err().to_string()]);
            continue;
        }

        let entry_path = entry_result.unwrap().path();

        if !entry_path.is_dir() {
            continue;
        }

        let mut basemod_path = entry_path.clone();
        let mut modtxt_path = entry_path.clone();
        basemod_path.push("base.lua");
        modtxt_path.push("mod.txt");

        if !basemod_path.exists() {
            continue;
        }

        // Do this so the prompt is themed
        // Yes this is stupid
        found.insert(basemod_path.to_str().unwrap().italic().to_string(), basemod_path);

    }

    return found;

}

fn prompt_filter_mods(unfiltered: &HashMap<String, PathBuf>) -> Vec<PathBuf> {

    let multiselect = get_multi();

    let keys_vec: Vec<String> = unfiltered.keys().map(|x| x.to_owned()).collect();
    let keys_slice: &[String] = keys_vec.as_slice();

    let selected = multiselect.items(keys_slice)
               .with_prompt(messages::getmsg("prompt_derive_multiselect").msg())
               .interact();

    messages::getmsg("postprompt_derive_multiselect").print();

    let selected = selected.unwrap();

    let mut filtered = vec![];
    for idx in selected {
        let key = keys_slice.get(idx).unwrap();
        let value = unfiltered.get(key).unwrap().to_owned();
        filtered.push(value);

        // Re-print selection ourselves for theming
        println!("      {}", key.dimmed().italic());
    }

    filtered

}

pub fn get_wanted_mods() -> Vec<PathBuf> {

    match find_local_basemod() {
        Some(path) => {
            return vec![path];
        },
        None => ()
    }

    let cwd_result = std::env::current_dir();
    let cwd = match cwd_result {
        Ok(v) => v,
        Err(e) => app::error("cwd_access_failure", Some(&e))
    };

    let unfiltered_paths = get_subdir_mods(&cwd);

    return prompt_filter_mods(&unfiltered_paths)
    
   //Some(mod_paths)

}