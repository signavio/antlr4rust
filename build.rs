use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

const ANTLR4_JAR: &str = "antlr-4.13.1-complete.jar";

fn main() {
    let grammars = vec![
        "CSV",
        "ReferenceToATN",
        "XMLLexer",
        "SimpleLR",
        "Labels",
        "FHIRPath",
    ];
    let additional_args = vec![Some("-visitor"), None, None, None, None];
    let antlr_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tools")
        .join(ANTLR4_JAR);

    if !antlr_path.exists() {
        eprintln!(
            "#####\nPlease download antlr:\n  curl 'https://raw.githubusercontent.com/antlr/website-antlr4/gh-pages/download/{}' -o {}\n#####",
            ANTLR4_JAR,
            antlr_path.display()
        );
        std::process::exit(-1);
    }

    for (grammar, arg) in grammars.into_iter().zip(additional_args) {
        //ignoring error because we do not need to run anything when deploying to crates.io
        let _ = gen_for_grammar(grammar, &antlr_path, arg);
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", antlr_path.display());
}

fn gen_for_grammar(
    grammar_file_name: &str, antlr_path: &Path, additional_arg: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let dest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("gen");
    let input = env::current_dir().unwrap().join("grammars");
    let file_name = grammar_file_name.to_owned() + ".g4";

    Command::new("java")
        .current_dir(input)
        .arg("-cp")
        .arg(antlr_path)
        .arg("org.antlr.v4.Tool")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg(dest_path)
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;
    // .unwrap()
    // .stdout;
    // eprintln!("xx{}",String::from_utf8(x).unwrap());

    println!("cargo:rerun-if-changed=grammars/{}", file_name);
    Ok(())
}
