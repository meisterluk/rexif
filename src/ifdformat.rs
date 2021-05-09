use super::lowlevel::*;
use super::types::*;
use std::fmt::Display;
use num::Float;
use std::fmt;
use std::cell::RefCell;

pub(crate) struct NumArray<I>(RefCell<Option<I>>);

impl<I> NumArray<I> {
    pub fn new(i: I) -> Self {
        Self(RefCell::new(Some(i)))
    }
}

impl<T: Display, I: IntoIterator<Item=T>> Display for NumArray<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        let iter = self.0.borrow_mut().take().unwrap();
        for number in iter {
            if !first {
                write!(f, ", {}", number)?;
                first = false;
            } else {
                write!(f, "{}", number)?;
            }
        }
        Ok(())
    }
}

/// Convert a IfdEntry into a tuple of TagValue
pub fn tag_value_new(f: &IfdEntry) -> TagValue {
    match f.format {
        IfdFormat::Ascii => {
            // Remove \0, there may be more than one
            let mut tot = f.data.len();
            while tot > 0 && f.data[tot - 1] == 0 {
                tot -= 1;
            }
            // In theory it should be pure ASCII but we admit UTF-8
            let s = String::from_utf8_lossy(&f.data[0..tot]);
            let s = s.into_owned();
            TagValue::Ascii(s)
        }
        IfdFormat::U16 => {
            if f.data.len() < (f.count as usize * 2) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_u16_array(f.le, f.count, &f.data).unwrap();
            TagValue::U16(a)
        }
        IfdFormat::I16 => {
            if f.data.len() < (f.count as usize * 2) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_i16_array(f.le, f.count, &f.data).unwrap();
            TagValue::I16(a)
        }
        IfdFormat::U8 => {
            if f.data.len() < (f.count as usize * 1) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = f.data.clone();
            TagValue::U8(a)
        }
        IfdFormat::I8 => {
            if f.data.len() < (f.count as usize * 1) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_i8_array(f.count, &f.data).unwrap();
            TagValue::I8(a)
        }
        IfdFormat::U32 => {
            if f.data.len() < (f.count as usize * 4) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_u32_array(f.le, f.count, &f.data).unwrap();
            TagValue::U32(a)
        }
        IfdFormat::I32 => {
            if f.data.len() < (f.count as usize * 4) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_i32_array(f.le, f.count, &f.data).unwrap();
            TagValue::I32(a)
        }
        IfdFormat::F32 => {
            if f.data.len() < (f.count as usize * 4) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_f32_array(f.count, &f.data).unwrap();
            TagValue::F32(a)
        }
        IfdFormat::F64 => {
            if f.data.len() < (f.count as usize * 8) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_f64_array(f.count, &f.data).unwrap();
            TagValue::F64(a)
        }
        IfdFormat::URational => {
            if f.data.len() < (f.count as usize * 8) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_urational_array(f.le, f.count, &f.data).unwrap();
            TagValue::URational(a)
        }
        IfdFormat::IRational => {
            if f.data.len() < (f.count as usize * 8) {
                return TagValue::Invalid(f.data.clone(), f.le, f.format as u16, f.count);
            }
            let a = read_irational_array(f.le, f.count, &f.data).unwrap();
            TagValue::IRational(a)
        }

        IfdFormat::Undefined => {
            let a = f.data.clone();
            TagValue::Undefined(a, f.le)
        }

        _ => TagValue::Unknown(f.data.clone(), f.le),
    }
}

/// Compare two vectors of floats, and always consider NaN == NaN.
fn vec_cmp<F: Float>(va: &[F], vb: &[F]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| (a.is_nan() && b.is_nan() || (a == b)))
}

/// Check if `left` == `right`. If the `left` and `right` are float vectors, this returns `true` even
/// if they contain NaN values (as long as the two vectors are otherwise identical, and contain NaN
/// values at the same positions).
pub fn tag_value_eq(left: &TagValue, right: &TagValue) -> bool {
    match (left, right) {
        (TagValue::F32(x), TagValue::F32(y)) => vec_cmp(&x, &y),
        (TagValue::F64(x), TagValue::F64(y)) => vec_cmp(&x, &y),
        (x, y) => x == y,
    }
}
