extern crate image;

use image::GenericImageView;
use clap::App;


fn scale_image(image_path: String, scale: u32, sample_size: u32) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {

    // Open image
    let orginal_image = image::open(image_path).unwrap();
    
    // Get dimensions
    let (orginal_width, orginal_height) = orginal_image.dimensions();

    // resize image
    let scaled_image = orginal_image.resize(orginal_width*scale, orginal_height*scale, image::imageops::Nearest);
   
    // Apply filter
    let smoothed_image = imageproc::filter::median_filter(&scaled_image.into_rgb8(), sample_size, sample_size);

    // Return image
    return smoothed_image;

}


fn main() {

    // Load parameter info
    let yaml = clap::load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    // Get sub command matches
    if let Some(scale_matches) = matches.subcommand_matches("scale") {
            
        // Get parameter values
        let scale_factor = scale_matches.value_of("scale").unwrap().parse::<u32>().unwrap();
        let sample_size = scale_matches.value_of("median").unwrap().parse::<u32>().unwrap();
        let orginal_image = scale_matches.value_of("image").unwrap().to_string();
        
        // Scale image
        let large_image = scale_image(orginal_image, scale_factor, sample_size);
                
        // Save Image
        large_image.save("output.png");
    }
}
