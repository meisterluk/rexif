#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let (res, _) = rexif::parse_buffer_quiet(data);

    // The JPEG application marker.
    const APP_MARKER: &[u8] = &[0xff, 0xd8, 0xff, 0xe1];

    if let Ok(parsed_exif1) = res {

        let serialized_exif1 = parsed_exif1.serialize().expect("in fuzz unwrap 1");

        // If the image is a JPEG, `parse_buffer` expects the content to begin with the JPEG app
        // marker
        let serialized_exif1 = if parsed_exif1.mime == "image/jpeg" {
            let size = (serialized_exif1.len() as u16 + 2).to_be_bytes();
            [APP_MARKER, &size, &serialized_exif1].concat()
        } else {
            parsed_exif1.serialize().expect("in fuzz unwrap 2")
        };

        let (parsed_exif2, _) = rexif::parse_buffer_quiet(&serialized_exif1);
        let parsed_exif2 = parsed_exif2.expect("in fuzz unwrap 3");

        let serialized_exif2 = parsed_exif2.serialize().expect("in fuzz unwrap 4");
        let serialized_exif2 = if parsed_exif2.mime == "image/jpeg" {
            let size = (serialized_exif2.len() as u16 + 2).to_be_bytes();
            [APP_MARKER, &size, &serialized_exif2].concat()
        } else {
            serialized_exif2
        };

        assert_eq!(serialized_exif1, serialized_exif2);
        assert_eq!(parsed_exif1, parsed_exif2);
    }
});
