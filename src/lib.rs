/// A SIMD implementation of `is_ascii` for x86_64 AVX2.
use std::arch::x86_64::*;

static ASCII_LIMIT: i8 = -128;

/// Checks if all characters in this string are within the ASCII range.
///
/// # Examples
///
/// ```
/// # use fascii::is_ascii;
///
/// let ascii = "hello! this is a test weeeeeeee\n".to_string();
/// let non_ascii = "Grüße, Jürgen ❤❤❤❤❤".to_string();
///
/// assert!(is_ascii(ascii));
/// assert!(!is_ascii(non_ascii));
/// ```
#[inline(always)]
pub fn is_ascii(s: String) -> bool {
    if !is_x86_feature_detected!("avx2") {
        panic!("Unsupported processor!");
    }
    let bytes = s.as_bytes();
    // let padding = vec![0; 32 - bytes.len() % 32];
    // let bytes = [bytes, &padding[..]].concat();

    let mask;
    unsafe {
        mask = _mm256_set1_epi8(ASCII_LIMIT);
    }

    let length = bytes.len();
    println!("{}", length);
    assert!(length % 32 == 0);

    let mut base = bytes.as_ptr();
    for _ in 0..(length / 32) {
        // check 32 bytes at a time
        unsafe {
            if _mm256_testz_si256(_mm256_lddqu_si256(base as *const __m256i), mask) == 0 {
                return false;
            }
            base = base.offset(32);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii() {
        let ascii = String::from("hello! this is a test weeeeeeee\n");
        assert!(is_ascii(ascii));
    }

    #[test]
    fn non_ascii() {
        let non_ascii = String::from("Grüße, Jürgen ❤❤❤❤❤");
        assert!(!is_ascii(non_ascii));
    }

    // #[test]
    // fn non_aligned_string() {
    //     let ascii = String::from("Hello, world!\n");
    //     let non_ascii = String::from("Grüße, Jürgen ❤");
    //     assert!(is_ascii(ascii));
    //     assert!(!is_ascii(non_ascii));
    // }
}
