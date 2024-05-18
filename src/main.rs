fn main() {
    let file = "image.ARW";
    let meta = rexiv2::Metadata::new_from_path(file);
    match meta {
        Ok(image) => {
            println!("Focal length: {:?}", image.get_focal_length().unwrap());
            println!(
                "Shutter speed: {:?}",
                image.get_exposure_time().unwrap().to_string()
            );
            println!("Aperture: {:?}", image.get_fnumber().unwrap());
            println!("ISO: {:?}", image.get_iso_speed().unwrap());
        }
        Err(_) => std::process::exit(1),
    }
}
