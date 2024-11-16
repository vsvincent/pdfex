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
    save_dir: String,
    file_path: String,
}

fn main() -> Result<(), PDF2ImageError> {
    let opt = Opt::from_args();
    println!("Loading PDF... {}", opt.file_path);

    if !fs::metadata(&opt.file_path).is_ok() {
        eprintln!("Error: File not found at path: {}", opt.file_path);
        return Err(PDF2ImageError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found `{}`", opt.file_path),
        )));
    } else {
        println!("File found at path: {}", opt.file_path);
    }
    utils::print::print_paths_in_current_dir().unwrap();
    let pdf = match PDF::from_file(opt.file_path) {
        Ok(pdf) => pdf,
        Err(e) => {
            eprintln!("Failed to load PDF: {}", e);
            return Err(e);
        }
    };
    let pages = pdf
        .render(
            pdf2image::Pages::Range(1..=8),
            RenderOptionsBuilder::default().build()?,
        )
        .unwrap();
    println!("{:?}", pages.len());
    let pages_enumerable = pages.iter().enumerate();
    for (i, page) in pages_enumerable {
        let file_name = format!("{}page_{}.jpg", opt.save_dir, i + 1);
        match page.save(&file_name) {
            Ok(_) => {
                println!("Saved {}", file_name);
                utils::text::extract_text_from_image(&file_name).unwrap();
            }
            Err(e) => eprintln!("Failed to save {}: {}", file_name, e),
        }
    }
    Ok(())
}
