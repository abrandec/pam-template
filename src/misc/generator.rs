// external
use clap::Parser;
use fancy_regex::Regex;
use rust_embed::RustEmbed;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    str::from_utf8,
};

#[derive(RustEmbed)]
#[folder = "src/template/"]
pub struct Template;

// pam name validation.
// results in a panic if name is in the incorrect format
pub fn validate_pam_name(pam_name: String) -> Vec<String> {
    // pam modules require(?) exactly one "-" in-between two words in order to work.
    // check if formatted correctly
    if Regex::new(r"(?<=[a-zA-Z])-(?=[a-zA-Z])")
        .expect("regex broken?")
        .is_match(pam_name.clone().as_str())
        .expect("cannot unwrap pam_name_check_re")
        == false
        // check for special characters
        || Regex::new(r"([\/\\{\}\[\]+_*$~`!@#^()|';:.,<>])")
            .expect("regex broken?")
            .is_match(pam_name.clone().as_str())
            .expect("cannot unwrap regex")
            == true
        || pam_name.clone().matches("-").count() > 1
    {
        panic!("Incorrect name format for template");
    }

    // Split Strings
    let split = pam_name.as_str().split("-");
    let name_split: Vec<&str> = split.collect();
    name_split
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>()
}

// initialize directory for new pam project
fn init_dir(name_og: String) {
    if Path::new(&name_og.clone()).exists() == true {
        panic!("path already exists");
    }

    // create directory structure
    create_dir_all(&name_og.clone());

    // client dir
    create_dir_all(&format!("{}/client", name_og.clone()));
    create_dir_all(&format!("{}/client/src", name_og.clone()));

    // module dir
    create_dir_all(&format!("{}/module", name_og.clone()));
    create_dir_all(&format!("{}/module/src", name_og.clone()));
    create_dir_all(&format!("{}/module/conf", name_og.clone()));
}

pub fn gen_template(pam_name: String) {
    /// string check ///
    let mut name_split = validate_pam_name(pam_name);

    /// file loading zone ///
    // root files //
    let just_file = Template::get("Justfile").unwrap();
    let root_toml = Template::get("Cargo.toml").expect("client/Cargo.toml missing");

    // client files //
    let cli_toml = Template::get("client/Cargo.toml").expect("client/Cargo.toml missing");
    let cli_main = Template::get("client/src/main.rs").expect("client/src/main.rs missing");

    // module files //
    let mod_toml = Template::get("module/Cargo.toml").expect("module/Cargo.toml missing");

    let conf = Template::get("module/conf/template-pam").expect("module/conf/template-pam missing");
    let mod_lib = Template::get("module/src/lib.rs").expect("module/src/lib.rs missing");

    // name initialization
    let name_og = name_split.clone().join("_");

    // vector switcheroo
    let temp_name_val = name_split[0].clone();
    name_split[0] = name_split[1].clone();
    name_split[1] = temp_name_val;

    // pam.d service name gets reversed
    let name_pamd = name_split.clone().join("-");
    init_dir(name_og.clone());

    // for struct in module lib
    let struct_name = format!(
        "{}{}",
        name_split[1][0..1].to_uppercase() + &name_split[1][1..],
        name_split[0][0..1].to_uppercase() + &name_split[0][1..]
    );

    /// regex time ///
    /////////////////
    // dir: 'root' //
    /////////////////

    // Cargo.toml
    let new_root_toml = from_utf8(root_toml.data.as_ref()).unwrap();

    // Justfile
    let mut new_just_file = Regex::new(r"template-pam")
        .expect("regex broken?")
        .replace_all(&from_utf8(just_file.data.as_ref()).unwrap(), &name_pamd);

    let just_file_ptr = &Regex::new(r"pam_template")
        .expect("regex broken?")
        .replace_all(new_just_file.as_ref(), &name_og);

    /////////////////
    // dir: client //
    /////////////////

    // Cargo.toml
    let new_cli_toml = from_utf8(cli_toml.data.as_ref()).unwrap();

    // src/main.rs
    let new_cli_main = Regex::new(r"template-pam")
        .expect("regex broken?")
        .replace_all(&from_utf8(cli_main.data.as_ref()).unwrap(), &name_pamd);

    /////////////////
    // dir: module //
    /////////////////

    // Cargo.toml
    let mut new_mod_toml = Regex::new(r"pam-module-template|pam_template")
        .expect("regex broken?")
        .replace_all(&from_utf8(mod_toml.data.as_ref()).unwrap(), &name_og);
    // src/conf/
    let new_conf = Regex::new(r"pam_template")
        .expect("regex broken?")
        .replace_all(&from_utf8(conf.data.as_ref()).unwrap(), &name_og);
    // src/lib.rs
    let new_mod_lib = Regex::new(r"PamTemplate")
        .expect("regex broken?")
        .replace_all(&from_utf8(mod_lib.data.as_ref()).unwrap(), &struct_name);

    /// writin' time ///
    // root
    let mut root_toml_file =
        File::create(&format!("{}/Cargo.toml", &name_og)).expect("cannot write Justfile");
    root_toml_file
        .write_all(new_root_toml.as_bytes())
        .expect("cannot write to Justfile");

    let mut newer_just_file =
        File::create(&format!("{}/Justfile", &name_og)).expect("cannot write Justfile");
    newer_just_file
        .write_all(just_file_ptr.as_ref().as_bytes())
        .expect("cannot write to Justfile");

    // client
    let mut cli_toml_file =
        File::create(&format!("{}/client/Cargo.toml", &name_og)).expect("cannot write Justfile");
    cli_toml_file
        .write_all(new_cli_toml.as_bytes())
        .expect("cannot write to Justfile");
    let mut cli_main_file = File::create(&format!("{}/client/src/main.rs", &name_og))
        .expect("cannot create /cli/src/main.rs");
    cli_main_file
        .write_all(new_cli_main.as_bytes())
        .expect("cannot write to cli/src/main.rs");

    // module
    let mut mod_toml_file =
        File::create(&format!("{}/module/Cargo.toml", &name_og)).expect("cannot write Justfile");
    mod_toml_file
        .write_all(new_mod_toml.as_bytes())
        .expect("cannot write to Justfile");
    let mut conf_file = File::create(&format!("{}/module/conf/{}", &name_og, &name_pamd))
        .expect("cannot write Justfile");
    conf_file
        .write_all(new_conf.as_bytes())
        .expect("cannot write to Justfile");
    let mut mod_lib_file =
        File::create(&format!("{}/module/src/lib.rs", &name_og)).expect("cannot write Justfile");
    mod_lib_file
        .write_all(new_mod_lib.as_bytes())
        .expect("cannot write to Justfile");
}
