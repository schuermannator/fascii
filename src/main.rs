/// A SIMD implementation of `is_ascii` for x86_64 AVX2.
use std::arch::x86_64::*;

static ASCII_LIMIT: i8 = -128;

#[inline(always)]
fn is_ascii(s: String) -> bool {
    let bytes = s.as_bytes();
    // let padding = vec![0; 32 - bytes.len() % 32];
    // let bytes = [bytes, &padding[..]].concat();

    let mask;
    unsafe {
        mask = _mm256_set1_epi8(ASCII_LIMIT);
    }

    let length = bytes.len();
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

fn main() {
    // let mut bytes = vec![0xf0, 0x9f, 0x98, 0x81];
    let mut bytes = vec![];
    let mut long = vec![122; 600_000_000];
    bytes.append(&mut long); // 60 MB's
    println!("{}", bytes.len());
    let s = String::from_utf8(bytes).unwrap();
    println!("{}", is_ascii(s));
}
