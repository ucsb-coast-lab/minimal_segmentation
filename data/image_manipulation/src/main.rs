#![allow(unused_code)]
use image::imageops::{grayscale, resize, FilterType};
use image::*;
use std::error::Error;
use std::fs::*;
use std::path::PathBuf;

// Set the general components here
const BASE_PATH: &str = "../bluebirds";
const ORIGINAL_IMAGES_DIR: &str = "resized_labels";
const NEW_IMAGES_DIR: &str = "resized_labels";
const WIDTH: u32 = 300;
const HEIGHT: u32 = 200;

fn main() -> Result<(), Box<dyn Error>> {
    // Get information on the original files here
    let original_images_dir_path = PathBuf::from([BASE_PATH, ORIGINAL_IMAGES_DIR].join("/"));
    let image_paths =
        read_dir(&original_images_dir_path).expect(&format!("Couldn't open paths at {:?}", &original_images_dir_path));

    // Create information on the intended output directory; if it doesn't exist, recursively create it
    let new_images_dir_path = PathBuf::from([BASE_PATH, NEW_IMAGES_DIR].join("/"));
    if !&new_images_dir_path.exists() {
        create_dir_all(&new_images_dir_path).expect(&format!(
            "{:?} didn't exist, and couldn't create for new images",
            &new_images_dir_path
        ));
    }

    // Specify the actual operation function here
    // resize_image_color(image_paths, new_images_dir_path)?;
    // resize_image_grayscale(image_paths, new_images_dir_path)?;
    shift_image_information(image_paths, new_images_dir_path)?;
    
    Ok(())
}


/// This function can be used to 
fn shift_image_information(image_paths: ReadDir, new_images_dir_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut shifted: RgbaImage = ImageBuffer::new(WIDTH, HEIGHT);
    for path in image_paths {
        let img_path = path?.path();
        {
            let mut img = image::open(img_path.clone()).expect(&format!("Couldn't open {:?}", &img_path));
            let (w,h) = img.dimensions();
            for y in 0..h {
                for x in 0..w {
                    let pixel = img.get_pixel(x,y);
                    // Adjust the mask here to be assigned with a zero base. 
                    // Make sure that 
                    let class = match pixel {
                        Rgba([0,0,0,255]) => {
                            //0
                            Rgba([0,0,0,255])
                        },
                        Rgba([2,2,2,255]) => {
                            // 1
                            Rgba([1,1,1,255])
                        },
                        Rgba([1,1,1,255]) => {
                            // 2
                            Rgba([2,2,2,255])
                        }
                        _ => panic!(format!("Unexpected class at {:?}",(x,y)))
                    };
                    shifted.put_pixel(x,y,class);
                    // print!("{}",class);
                }
                //println!("");
            }

        }
        println!("- Shifted image info for image at: {:?}", &img_path.display());
        let shifted_path = img_path.to_str().unwrap().split("/").last().unwrap();
        let shifted_save_path = [new_images_dir_path.to_str().unwrap(), shifted_path].join("/");
        println!("- Saving image to {:?}", &shifted_save_path);
        shifted.save(shifted_save_path)?;
    }
    Ok(())
}


/// Resize and save images to dimensions (WIDTH,HEIGHT), with all colors channels intact 
fn resize_image_color(image_paths: ReadDir, new_images_dir_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut scaled = ImageBuffer::new(WIDTH, HEIGHT);
    for path in image_paths {
        let img_path = path?.path();
        {
            let mut img = image::open(img_path.clone()).expect(&format!("Couldn't open {:?}", &img_path));
            scaled = resize(&img, WIDTH, HEIGHT, FilterType::Nearest);
        }
        println!("- Resized image at: {:?}", &img_path.display());
        let scaled_path = img_path.to_str().unwrap().split("/").last().unwrap();
        let scaled_save_path = [new_images_dir_path.to_str().unwrap(), scaled_path].join("/");
        println!("- Saving image to {:?}", &scaled_save_path);
        scaled.save(scaled_save_path)?;
    }
    Ok(())
}


/// Resize and save images to dimensions (WIDTH,HEIGHT) as a grayscale image
fn resize_image_grayscale(image_paths: ReadDir, new_images_dir_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut scaled = ImageBuffer::new(WIDTH, HEIGHT);
    for path in image_paths {
        let img_path = path?.path();
        {
            let mut img = image::open(img_path.clone()).expect(&format!("Couldn't open {:?}", &img_path));
            scaled = grayscale(&resize(&img, WIDTH, HEIGHT, FilterType::Gaussian));
        }
        println!("- Resized image at: {:?}", &img_path.display());
        let scaled_path = img_path.to_str().unwrap().split("/").last().unwrap();
        let scaled_save_path = [new_images_dir_path.to_str().unwrap(), scaled_path].join("/");
        println!("- Saving image to {:?}", &scaled_save_path);
        scaled.save(scaled_save_path)?;
    }
    Ok(())
}
