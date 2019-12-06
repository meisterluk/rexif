#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let res = rexif::parse_buffer(data);

    const APP_MARKER: &[u8] = &[0xff, 0xd8, 0xff, 0xe1];

    if let Ok(parsed_exif1) = res {

        let serialized_exif1 = parsed_exif1.serialize();

        let serialized_exif1 = if &parsed_exif1.mime == "image/jpeg" {
            let size = (serialized_exif1.len() as u16 + 2).to_be_bytes();
            [APP_MARKER, &size, &serialized_exif1].concat()
        } else {
            parsed_exif1.serialize()
        };

        let parsed_exif2 = rexif::parse_buffer(&serialized_exif1).unwrap();

        let serialized_exif2 = parsed_exif2.serialize();
        let serialized_exif2 = if &parsed_exif2.mime == "image/jpeg" {
            let size = (serialized_exif2.len() as u16 + 2).to_be_bytes();
            [APP_MARKER, &size, &serialized_exif2].concat()
        } else {
            serialized_exif2
        };

        assert_eq!(serialized_exif1, serialized_exif2);
        assert_eq!(parsed_exif1, parsed_exif2);
    }
});
