extern crate image;

use image::GenericImageView;
use clap::App;


fn scale_image(image_path: &str, scale: u32, sample_size: u32) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {

    // Open image and get dimensions
    let orginal_image = image::open(image_path).unwrap();
    let (orginal_width, orginal_height) = orginal_image.dimensions();

    // Resize image and apply filter
    let scaled_image = orginal_image.resize(orginal_width*scale, orginal_height*scale, image::imageops::Nearest);
    let smoothed_image = imageproc::filter::median_filter(&scaled_image.into_rgb8(), sample_size, sample_size);

    return smoothed_image;

}


fn get_file_name(file_path: &str) -> &str {
    // Remove path from name
    let break_postion = file_path.rfind("/");
    let file_name;
    if break_postion != None {
        file_name = &file_path[break_postion.unwrap()+1..file_path.chars().count()];
    } else {
        file_name = file_path;
    }
    
    // Remove extension from name
    let extension_break_postion = file_name.rfind(".");
    let extensionless_file_name;
    if break_postion != None {
        extensionless_file_name = &file_name[0..extension_break_postion.unwrap()];
    } else {
        extensionless_file_name = file_name;
    }

    return extensionless_file_name;
}



fn main() {

    // Load parameter info
    let yaml = clap::load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    // Get sub command matches
    if let Some(scale_matches) = matches.subcommand_matches("cut") {
            
        // Get parameter values
        let image_path = scale_matches.value_of("input").unwrap();
        let output = scale_matches.value_of("output").unwrap();
        let size = scale_matches.value_of("size").unwrap().parse::<u32>().unwrap();

        // Open Image
        let orginal_image = image::open(image_path).unwrap();

        // Get image name
        let image_name = get_file_name(image_path);

        // Get dimensions
        let (orginal_width, orginal_height) = orginal_image.dimensions();

        // Get images
        let images_wide = orginal_width / size;
        let images_tall = orginal_height / size;

        // Loop though images
        for x in 0..images_wide {
            for y in 0..images_tall {
                let cropped = orginal_image.crop_imm(x*size, y*size, size, size);
                cropped.save(output.to_string() + image_name + "." + &x.to_string() + "." + &y.to_string() + ".png");
            }
        }
    }

    // Get sub command matches
    if let Some(scale_matches) = matches.subcommand_matches("scale") {
            
        // Get parameter values
        let scale_factor = scale_matches.value_of("scale").unwrap().parse::<u32>().unwrap();
        let sample_size = scale_matches.value_of("median").unwrap().parse::<u32>().unwrap();
        let image_paths = scale_matches.values_of("input").unwrap();
        let output = scale_matches.value_of("output").unwrap();


        // Scale images
        for image_path in image_paths {
           
            // Scale image
            let large_image = scale_image(image_path, scale_factor, sample_size);

            // Get image name
            let image_name = get_file_name(image_path);

            // Save Image
            large_image.save(output.to_string() + &image_name + ".png");
            println!("Saved image {}", image_name);
        }
    }
}
