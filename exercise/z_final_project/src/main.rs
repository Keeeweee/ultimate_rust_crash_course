// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use image::DynamicImage;

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let mut subcommand = args.remove(0);
    if subcommand == "generate" {
        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!
        if args.len() != 4 {
            print_usage_and_exit();
        }
        let outfile = args.remove(0);
        let red: u8 = args.remove(0).parse().expect("Failed to parse a number");
        let green: u8 = args.remove(0).parse().expect("Failed to parse a number");
        let blue: u8 = args.remove(0).parse().expect("Failed to parse a number");


        generate(outfile, red, green, blue);
    }
    else if subcommand == "fractal" {
        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)

        if args.len() != 1 {
            print_usage_and_exit();
        }
        let outfile = args.remove(0);
        fractal(outfile);
    }
    else {
        if args.len() < 2 {
            print_usage_and_exit();
        }

        let infile = subcommand;
        let outfile = args.remove(0);
        let mut img = image::open(infile).expect("Failed to open INFILE.");

        while args.len() > 0 {
            subcommand = args.remove(0);
            match subcommand.as_str() {
                // EXAMPLE FOR CONVERSION OPERATIONS
                "blur" => {
                    if args.len() < 1 {
                        print_usage_and_exit();
                    }
                    else {
                        println!("Blurring Image.");
                    }

                    let blur_ammunt: f32 = args.remove(0).parse().expect("Failed to parse a number");
                    // **OPTION**
                    // Improve the blur implementation -- see the blur() function below
                    img = blur(&img, blur_ammunt);
                }

                // **OPTION**
                // Brighten -- see the brighten() function below
                "brighten" => {
                    if args.len() < 1 {
                        print_usage_and_exit();
                    }
                    else {
                        println!("Brightening Image.");
                    }

                    let brighten_ammount: i32 = args.remove(0).parse().expect("Failed to parse a number");

                    img = brighten(&img, brighten_ammount);
                }

                // **OPTION**
                // Crop -- see the crop() function below
                "crop" => {
                    if args.len() < 4 {
                        print_usage_and_exit();
                    }
                    else {
                        println!("Cropping Image.");
                    }

                    let x: u32 = args.remove(0).parse().expect("Failed to parse a number");
                    let y: u32 = args.remove(0).parse().expect("Failed to parse a number");
                    let width: u32 = args.remove(0).parse().expect("Failed to parse a number");
                    let height: u32 = args.remove(0).parse().expect("Failed to parse a number");

                    img = crop(&mut img, x, y, width, height);
                }

                // **OPTION**
                // Rotate -- see the rotate() function below
                "rotate" => {
                    if args.len() < 1 {
                        print_usage_and_exit();
                    }
                    else {
                        println!("Rotating Image.");
                    }

                    let degrees: i32 = args.remove(0).parse().expect("Failed to parse a number");

                    img = rotate(&img, degrees);
                }

                // **OPTION**
                // Invert -- see the invert() function below
                "invert" => {
                    if args.len() < 0 {
                        println!("Invert args {:?}", args);
                        print_usage_and_exit();
                    }
                    else {
                        println!("Inverting Image.");
                    }

                    // No need to reassign img as invert changes the image inplace
                    invert(&mut img);
                }

                // **OPTION**
                // Grayscale -- see the grayscale() function below
                "grayscale" => {
                    if args.len() < 0 {
                        print_usage_and_exit();
                    }
                    else {
                        println!("Grayscaling Image.");
                    }

                    img = grayscale(&img);
                }

                // For everything else...
                _ => {
                    println!("Unknown Operation {}.", subcommand);
                    print_usage_and_exit();
                }
            }
        }

        img.save(outfile).expect("Failed writing OUTFILE.");
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE BLUR");
    println!("brighten INFILE OUTFILE BRIGHTEN");
    println!("crop INFILE OUTFILE X Y WIDTH HEIGHT");
    println!("rotate INFILE OUTFILE [-]90|180|270");
    println!("invert INFILE OUTFILE");
    println!("grayscale INFILE OUTFILE");
    println!("fractal OUTFILE");
    println!("generate OUTFILE RED GREEN BLUE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(img: &DynamicImage, blur: f32) -> DynamicImage {
    // Here's how you open an existing image file
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    return img.blur(blur);
    // Here's how you save an image to a file.
}

fn brighten(img: &DynamicImage, brighten: i32) -> DynamicImage {
    // See blur() for an example of how to open / save an image.

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.

    return img.brighten(brighten);
}

fn crop(img: &mut DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    // See blur() for an example of how to open an image.

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.

    return img.crop(x, y, width, height);
}

fn rotate(img: &DynamicImage, rotation: i32) -> DynamicImage {
    // See blur() for an example of how to open an image.

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
    let rotation = if rotation < 0 { 360 + rotation } else { rotation };

    let img2 = match rotation {
        x if x == 90 => {
            img.rotate90()
        },
        x if x == 180 => {
            img.rotate180()
        },
        x if x == 270 => {
            img.rotate270()
        },
        _ => {
            println!("{}", rotation % 360);
            print_usage_and_exit();
            panic!();
        }
    };

    return img2;
}

fn invert(img: &mut DynamicImage){
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.

    (*img).invert();
}

fn grayscale(img: &DynamicImage) -> DynamicImage {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.

    return img.grayscale();
}

fn generate(outfile: String, red: u8, green: u8, blue: u8) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image

    let width = 729;
    let height = 729;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let inverse_red = 255 - red;
    let inverse_green = 255 - green;
    let inverse_blue = 255 - blue;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if in_sierpinski_square(x, y) {
            *pixel = image::Rgb([inverse_red, inverse_green, inverse_blue]);
        }
        else {
            *pixel = image::Rgb([red, green, blue]);
        }
    }

    imgbuf.save(outfile).unwrap();
}

fn in_sierpinski_square(x: u32, y: u32) -> bool {
    let mut xc = x;
    let mut yc = y;
    while xc > 0 && yc > 0 {
        if xc % 3 == 1 && yc % 3 == 1 {
            return false;
        }
        xc /= 3;
        yc /= 3;
    }
    true
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
