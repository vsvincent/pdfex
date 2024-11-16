pub fn extract_text_from_image(image_path: &str) -> Result<String, PDF2ImageError> {
    match tesseract::ocr(&file_name, "eng") {
        Ok(text) => {
            let txt_file_name = format!("{}page_{}.txt", opt.save_dir, i + 1);
            match fs::write(&txt_file_name, text) {
                Ok(_) => println!("Saved {}", txt_file_name),
                Err(e) => eprintln!("Failed to save {}: {}", txt_file_name, e),
            }
        }
        Err(e) => eprintln!("Failed to extract text from {}: {}", file_name, e),
    }
}
