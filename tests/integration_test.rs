use rexif::{ExifEntry, ExifTag, Namespace};

#[cfg(test)]
fn check_tags(entries: &Vec<ExifEntry>, expected_tags: Vec<ExifTag>) {
    let tags: Vec<ExifTag> = entries.iter().map(|entry| entry.tag).collect();
    assert_eq!(tags, expected_tags);
}

#[test]
fn test_parse_simple_morotola_jpeg() {
    let exif = rexif::parse_file("./tests/img/simple-motorola.jpg");
    assert!(exif.is_ok(), "{:?}", exif);

    let exif = exif.unwrap();
    assert_eq!(exif.mime, "image/jpeg");
    assert_eq!(exif.entries.len(), 7);

    let expected_tags = vec![
        ExifTag::Orientation,
        ExifTag::XResolution,
        ExifTag::YResolution,
        ExifTag::ResolutionUnit,
        ExifTag::ExifOffset,
        ExifTag::UnknownToMe,
        ExifTag::UnknownToMe,
    ];
    check_tags(&exif.entries, expected_tags);

    assert!(exif.entries.iter().all(|e| e.namespace == Namespace::Standard),
            "Expected all tags to be from the standard namespace")
}
