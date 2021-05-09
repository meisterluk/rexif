use super::types::*;

/// Find a tag of given type
fn other_tag(tag: ExifTag, entries: &[ExifEntry]) -> Option<&ExifEntry> {
    entries.into_iter().find(|entry| entry.tag == tag)
}

/// Does postprocessing in tags that depend on other tags to have a complete interpretation
/// e.g. when the unit of a tag is annotated on another tag
pub fn exif_postprocessing(entry: &mut ExifEntry, entries: &[ExifEntry]) {
    match entry.tag {
        ExifTag::XResolution | ExifTag::YResolution => {
            if let Some(f) = other_tag(ExifTag::ResolutionUnit, entries) {
                entry.unit = f.value_more_readable.clone();
                let v = entry.value_more_readable.to_mut();
                v.push_str(" pixels per ");
                v.push_str(&f.value_more_readable);
            }
        }

        ExifTag::FocalPlaneXResolution | ExifTag::FocalPlaneYResolution => {
            if let Some(f) = other_tag(ExifTag::FocalPlaneResolutionUnit, entries) {
                entry.unit = f.value_more_readable.clone();
                let v = entry.value_more_readable.to_mut();
                v.push_str(" pixels per ");
                v.push_str(&f.value_more_readable);
            }
        }

        ExifTag::GPSLatitude => {
            if let Some(f) = other_tag(ExifTag::GPSLatitudeRef, entries) {
                let v = entry.value_more_readable.to_mut();
                v.push(' ');
                v.push_str(&f.value_more_readable);
            }
        }

        ExifTag::GPSLongitude => {
            if let Some(f) = other_tag(ExifTag::GPSLongitudeRef, entries) {
                let v = entry.value_more_readable.to_mut();
                v.push(' ');
                v.push_str(&f.value_more_readable);
            }
        }

        ExifTag::GPSAltitude => {
            if let Some(f) = other_tag(ExifTag::GPSAltitudeRef, entries) {
                let altref = match f.value {
                    TagValue::U8(ref fv) => fv[0],
                    _ => return,
                };

                if altref != 0 {
                    entry.value_more_readable.to_mut().push_str(" below sea level");
                }
            }
        }

        ExifTag::GPSDestLatitude => {
            if let Some(f) = other_tag(ExifTag::GPSDestLatitudeRef, entries) {
                let v = entry.value_more_readable.to_mut();
                v.push(' ');
                v.push_str(&f.value_more_readable);
            }
        }

        ExifTag::GPSDestLongitude => {
            if let Some(f) = other_tag(ExifTag::GPSDestLongitudeRef, entries) {
                let v = entry.value_more_readable.to_mut();
                v.push(' ');
                v.push_str(&f.value_more_readable);
            }
        }

        ExifTag::GPSDestDistance => {
            if let Some(f) = other_tag(ExifTag::GPSDestDistanceRef, entries) {
                entry.unit = f.value_more_readable.clone();
                let v = entry.value_more_readable.to_mut();
                v.push(' ');
                v.push_str(&f.value_more_readable);
            }
        }

        ExifTag::GPSSpeed => {
            if let Some(f) = other_tag(ExifTag::GPSSpeedRef, entries) {
                entry.unit = f.value_more_readable.clone();
                let v = entry.value_more_readable.to_mut();
                v.push(' ');
                v.push_str(&f.value_more_readable);
            }
        }
        _ => (),
    }
}
