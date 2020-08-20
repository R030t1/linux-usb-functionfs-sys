#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod linux_usb_functionfs_sys {
    #[test]
    fn test_struct_contents() {
        assert_eq!(0, 1, "Not implemented.");
    }
}
