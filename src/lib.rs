#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::const_static_lifetime)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::os::raw::c_char;

    #[test]
    fn test_discid_put() {
        unsafe {
            let disc = discid_new();
            let first_track = 1;
            let offsets: [i32; 3] = [2000, 150, 1000];
            let last_track = (offsets.len() - 1) as i32;
            let success = discid_put(disc, first_track, last_track, offsets.as_ptr() as *mut i32);
            let error_msg_ptr = discid_get_error_msg(disc);
            assert!(success == 1, "discid_put: {}", from_str_ptr(error_msg_ptr));

            let id_str_ptr = discid_get_id(disc);
            assert_eq!("YPv1TJ03f7pu6fgwp1cpr3cVsf0-", from_str_ptr(id_str_ptr));
            discid_free(disc);
        }
    }

    #[test]
    fn test_discid_feature() {
        let features = discid_feature::DISCID_FEATURE_READ | discid_feature::DISCID_FEATURE_ISRC;
        assert_eq!(5, features);
    }

    #[test]
    fn test_has_feature() {
        unsafe {
            assert_eq!(1, discid_has_feature(discid_feature::DISCID_FEATURE_READ));
        }
    }

    fn from_str_ptr(str_ptr: *mut c_char) -> &'static str {
        let c_str: &CStr = unsafe { CStr::from_ptr(str_ptr) };
        c_str.to_str().unwrap()
    }
}
