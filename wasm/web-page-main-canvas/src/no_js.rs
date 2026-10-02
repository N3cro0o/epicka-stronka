pub fn log(str: &str) {
    println!("{str}");
}

pub fn alert(str: &str) {
    eprintln!("{str}");
}

pub fn get_pixel_ratio() -> Result<f64, String> {
    Ok(1.0)
}
