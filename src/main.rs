use std::env;
use std::process;

use rexif::{ByteAlign, ExifTag};

/// Tries to extract EXIF data from all files passed as CLI parameters,
/// assuming that the files contain images.
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} image1 image2 ...", args[0]);
        process::exit(2);
    }
    for arg in &args[1..] {
        match rexif::parse_file(&arg) {
            Ok(exif) => {
                println!("{} {} exif entries: {}", arg, exif.mime, exif.entries.len());
                for entry in &exif.entries {
                    if entry.tag == ExifTag::UnknownToMe {
                        /*
                        println!("      {} {}",
                            entry.tag_readable, entry.value_readable);
                        */
                    } else {
                        println!("      {}: {}", entry.tag, entry.value_more_readable);
                    }
                }
                let encoded = exif.serialize(ByteAlign::Motorola);
                println!("{:?}", encoded);
            }
            Err(e) => {
                eprintln!("Error in {}: {}", &arg, e);
            }
        }
    }
}
