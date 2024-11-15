use std::fs;
mod utils;

use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    rename_all = "kebab-case",
    about = "If this very complex tool is confounding you, use `-h`"
)]
struct Opt {
    path: String,
}

fn main() -> Result<(), PDF2ImageError> {
    let opt = Opt::from_args();
    println!("Loading PDF... {}", opt.path);

    if !fs::metadata(&opt.path).is_ok() {
        eprintln!("Error: File not found at path: {}", opt.path);
        return Err(PDF2ImageError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found `{}`", opt.path),
        )));
    } else {
        println!("File found at path: {}", opt.path);
    }
    utils::print::print_paths_in_current_dir().unwrap();
    let pdf = match PDF::from_file(opt.path) {
        Ok(pdf) => pdf,
        Err(e) => {
            eprintln!("Failed to load PDF: {}", e);
            return Err(e);
        }
    };
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=8),
        RenderOptionsBuilder::default().build()?,
    );
    println!("{:?}", pages.unwrap().len());

    Ok(())
}
