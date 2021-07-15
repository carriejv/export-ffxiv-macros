use directories::UserDirs;
use libxivdat::xiv_macro::{read_macro_content, Macro};
use serde::{Deserialize, Serialize};
use toml::to_string_pretty;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    #[serde(rename = "macro")]
    macro_vec: Vec<MacroOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MacroOutput {
    title: String,
    icon: String,
    lines: Vec<String>,
    text: String,
}

impl From<&Macro> for MacroOutput {
    fn from(x: &Macro) -> MacroOutput {
        let filtered_lines = x
            .lines
            .iter()
            .cloned()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();
        MacroOutput {
            title: String::from(&x.title),
            icon: format!("{:?}", x.get_icon().unwrap()),
            lines: filtered_lines.clone(),
            text: filtered_lines.join(" "),
        }
    }
}

fn main() {
    // Check if we got passed a path. If not, get all macros in default dir.
    let args = env::args().collect::<Vec<String>>();
    // Input and output provided.
    let targets = if args.len() == 3 {
        vec![(PathBuf::from(&args[1]), PathBuf::from(&args[2]))]
    } else if args.len() == 2 {
        let path = PathBuf::from(&args[1]);
        let mut out_path = PathBuf::from(match path.file_name() {
            Some(out_path) => out_path,
            None => {
                println!("Could not determine file name.");
                process::exit(1);
            }
        });
        out_path.set_extension(".toml");
        vec![(path, out_path)]
    } else {
        // Get user dir
        let usr_dir = match UserDirs::new() {
            Some(usr_dir) => usr_dir,
            None => {
                println!("Could not locate user dirs.");
                process::exit(1);
            }
        };
        let usr_doc_dir = match usr_dir.document_dir() {
            Some(usr_doc_dir) => usr_doc_dir,
            None => {
                println!("Could not locate user documents dir.");
                process::exit(1);
            }
        };
        let usr_doc_dir_str = match usr_doc_dir.to_str() {
            Some(usr_doc_dir_str) => usr_doc_dir_str,
            None => {
                println!("Path contains nonstandard characters.");
                process::exit(1);
            }
        };
        let game_cfg_dir = [
            usr_doc_dir_str,
            "My Games",
            "FINAL FANTASY XIV - A Realm Reborn",
        ]
        .iter()
        .collect::<PathBuf>();
        let mut targets = match fs::read_dir(&game_cfg_dir) {
            Ok(dir_items) => {
                let mut char_targets = Vec::<(PathBuf, PathBuf)>::new();
                for item in dir_items {
                    match item {
                        Ok(item) => {
                            let item_path = item.path();
                            let item_path_str = item_path.to_string_lossy();
                            if item_path.is_dir() && item_path_str.contains("FFXIV_CHR") {
                                // Get read path
                                let mut path_buf = PathBuf::from(&item_path);
                                path_buf.push("MACRO.DAT");
                                // Get write path
                                let parent_name = match item_path.file_name() {
                                    Some(parent_name) => parent_name,
                                    None => {
                                        println!("Could not determine parent folder for character macro file.");
                                        process::exit(1);
                                    }
                                };
                                let out_path = PathBuf::from(format!(
                                    "{}_MACRO.toml",
                                    parent_name.to_string_lossy()
                                ));
                                char_targets.push((path_buf, out_path));
                            }
                        }
                        Err(err) => {
                            println!("Filesystem error: {}.", err);
                            process::exit(1);
                        }
                    }
                }
                char_targets
            }
            Err(err) => {
                println!("Filesystem error: {}.", err);
                process::exit(1);
            }
        };
        let mut macrosys_path = game_cfg_dir;
        macrosys_path.push("MACROSYS.dat");
        targets.push((macrosys_path, PathBuf::from("MACROSYS.toml")));
        targets
    };

    // Export
    for target in targets.iter() {
        println!("Exporting {}...", target.0.to_string_lossy());
        println!("\t to {}.", target.1.to_string_lossy());
        // Read macros
        let macro_vec = match read_macro_content(&target.0) {
            Ok(macro_vec) => macro_vec,
            Err(err) => {
                println!("Error extracting macros: {}.", err);
                process::exit(1);
            }
        };
        // Convert to output struct
        let mut output = Output {
            macro_vec: Vec::<MacroOutput>::new(),
        };
        for macro_item in macro_vec.iter() {
            let output_fmt_macro = MacroOutput::from(macro_item);
            if !output_fmt_macro.title.is_empty() && !output_fmt_macro.lines.is_empty() {
                output.macro_vec.push(output_fmt_macro);
            }
        }
        // Write
        let toml_out = to_string_pretty(&output).unwrap();
        match fs::write(&target.1, &toml_out) {
            Ok(_) => (),
            Err(err) => {
                println!("Error writing output: {}.", err);
                process::exit(1);
            }
        };
    }
}
