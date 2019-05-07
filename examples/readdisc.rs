use discid_sys::*;
use std::ffi::{CStr, CString};

fn main() {
    unsafe {
        let handle = discid_new();
        let device = CString::new("/dev/cdrom").expect("CString::new failed");
        let features = discid_feature::DISCID_FEATURE_READ | discid_feature::DISCID_FEATURE_MCN;
        let status = discid_read_sparse(handle, device.into_raw(), features);

        if status == 0 {
            let error_msg_ptr = discid_get_error_msg(handle);
            panic!(
                "Reading disc failed: {}",
                CStr::from_ptr(error_msg_ptr).to_string_lossy()
            );
        }

        let id_ptr = discid_get_id(handle);
        println!("Disc ID: {}", CStr::from_ptr(id_ptr).to_string_lossy());

        let mcn_ptr = discid_get_mcn(handle);
        println!("MCN    : {}", CStr::from_ptr(mcn_ptr).to_string_lossy());

        discid_free(handle);
    }
}
