// build-pass

use spirv_std as _;

#[spirv(fragment)]
pub fn main(i: i32) {
    while i < 10 {
        if i == 0 {
            break;
        }
    }
}
