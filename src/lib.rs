#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C, packed)]
struct usb_functionfs_ep_descs {
    intf: usb_interface_descriptor,
    sink: usb_endpoint_descriptor_no_audio,
    source: usb_endpoint_descriptor_no_audio,
}

// Per example this is not packed.
struct usb_functionfs_ss_ep_descs {
    intf: usb_interface_descriptor,
    sink: usb_endpoint_descriptor_no_audio,
    sink_comp: usb_ss_ep_comp_descriptor,
    source: usb_endpoint_descriptor_no_audio,
    source_comp: usb_ss_ep_comp_descriptor,
}

#[repr(C, packed)]
struct usb_functionfs_descriptors {
    header: usb_functionfs_descs_head_v2,
    fs_count: u32,
    hs_count: u32,
    ss_count: u32,
    fs_descs: usb_functionfs_ep_descs,
    hs_descs: usb_functionfs_ep_descs,
    ss_descs: usb_functionfs_ss_ep_descs,
}

#[cfg(test)]
mod linux_usb_functionfs_sys {
    use std::mem;
    use crate::*;

    #[test]
    fn test_struct_contents() {
        let descriptors = usb_functionfs_descriptors {
            header: usb_functionfs_descs_head_v2 {
                magic: FUNCTIONFS_DESCRIPTORS_MAGIC_V2.to_le(),
                flags: (functionfs_flags_FUNCTIONFS_HAS_FS_DESC |
                        functionfs_flags_FUNCTIONFS_HAS_HS_DESC |
                        functionfs_flags_FUNCTIONFS_HAS_SS_DESC).to_le(),
                length: (mem::size_of::<usb_functionfs_descriptors>() as u32).to_le(),
            },
            fs_count: 3u32.to_le(),
            hs_count: 3u32.to_le(),
            ss_count: 5u32.to_le(),
            fs_descs: usb_functionfs_ep_descs {
                intf: usb_interface_descriptor {
                    bLength: mem::size_of::<usb_interface_descriptor>() as u8,
                    bDescriptorType: USB_DT_INTERFACE as u8,
                    bInterfaceNumber: 0,
                    bAlternateSetting: 0,
                    bNumEndpoints: 2,
                    bInterfaceClass: USB_CLASS_VENDOR_SPEC as u8,
                    bInterfaceSubClass: 0,
                    bInterfaceProtocol: 0,
                    iInterface: 1,
                },
                sink: usb_endpoint_descriptor_no_audio {
                    bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
                    bDescriptorType: USB_DT_ENDPOINT as u8,
                    bEndpointAddress: 1 | USB_DIR_IN as u8,
                    bmAttributes: USB_ENDPOINT_XFER_BULK as u8,
                    wMaxPacketSize: 0,
                    bInterval: 0,
                },
                source: usb_endpoint_descriptor_no_audio {
                    bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
                    bDescriptorType: USB_DT_ENDPOINT as u8,
                    bEndpointAddress: 2 | USB_DIR_OUT as u8,
                    bmAttributes: USB_ENDPOINT_XFER_BULK as u8,
                    wMaxPacketSize: 0,
                    bInterval: 0,
                },
            },
            hs_descs: usb_functionfs_ep_descs {
                intf: usb_interface_descriptor {
                    bLength: mem::size_of::<usb_interface_descriptor>() as u8,
                    bDescriptorType: USB_DT_INTERFACE as u8,
                    bInterfaceNumber: 0,
                    bAlternateSetting: 0,
                    bNumEndpoints: 2,
                    bInterfaceClass: USB_CLASS_VENDOR_SPEC as u8,
                    bInterfaceSubClass: 0,
                    bInterfaceProtocol: 0,
                    iInterface: 1,
                },
                sink: usb_endpoint_descriptor_no_audio {
                    bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
                    bDescriptorType: USB_DT_ENDPOINT as u8,
                    bEndpointAddress: 1 | USB_DIR_IN as u8,
                    bmAttributes: USB_ENDPOINT_XFER_BULK as u8,
                    wMaxPacketSize: 512u16.to_le(),
                    bInterval: 0,
                },
                source: usb_endpoint_descriptor_no_audio {
                    bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
                    bDescriptorType: USB_DT_ENDPOINT as u8,
                    bEndpointAddress: 2 | USB_DIR_OUT as u8,
                    bmAttributes: USB_ENDPOINT_XFER_BULK as u8,
                    wMaxPacketSize: 512u16.to_le(),
                    bInterval: 1,
                },
            },
            ss_descs: usb_functionfs_ss_ep_descs {
                intf: usb_interface_descriptor {
                    bLength: mem::size_of::<usb_interface_descriptor>() as u8,
                    bDescriptorType: USB_DT_INTERFACE as u8,
                    bInterfaceNumber: 0,
                    bAlternateSetting: 0,
                    bNumEndpoints: 2,
                    bInterfaceClass: USB_CLASS_VENDOR_SPEC as u8,
                    bInterfaceSubClass: 0,
                    bInterfaceProtocol: 0,
                    iInterface: 1,
                },
                sink: usb_endpoint_descriptor_no_audio {
                    bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
                    bDescriptorType: USB_DT_ENDPOINT as u8,
                    bEndpointAddress: 1 | USB_DIR_IN as u8,
                    bmAttributes: USB_ENDPOINT_XFER_BULK as u8,
                    wMaxPacketSize: 1024u16.to_le(),
                    bInterval: 0,
                },
                sink_comp: usb_ss_ep_comp_descriptor {
                    bLength: USB_DT_SS_EP_COMP_SIZE as u8,
                    bDescriptorType: USB_DT_SS_ENDPOINT_COMP as u8,
                    bMaxBurst: 0,
                    bmAttributes: 0,
                    wBytesPerInterval: 0,
                },
                source: usb_endpoint_descriptor_no_audio {
                    bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
                    bDescriptorType: USB_DT_ENDPOINT as u8,
                    bEndpointAddress: 2 | USB_DIR_OUT as u8,
                    bmAttributes: USB_ENDPOINT_XFER_BULK as u8,
                    wMaxPacketSize: 1024u16.to_le(),
                    bInterval: 1,
                },
                source_comp: usb_ss_ep_comp_descriptor {
                    bLength: USB_DT_SS_EP_COMP_SIZE as u8,
                    bDescriptorType: USB_DT_SS_ENDPOINT_COMP as u8,
                    bMaxBurst: 0,
                    bmAttributes: 0,
                    wBytesPerInterval: 0,
                },
            },
        };

        // This should catch most potential errors -- rust is very lenient about packing
        // and byte order.
        let size = mem::size_of::<usb_functionfs_descriptors>();
        assert_eq!(size, 105, "Descriptor size does not match reference implementation size");

        // But just to be sure, check the entire contents.
        let pointer: *const usb_functionfs_descriptors = &descriptors;
        let slice = unsafe { std::slice::from_raw_parts(&descriptors as *const _ as *const u8, size) };

        // TODO: Include this from output of program compiled against Linux headers.
        let real = vec![0x03, 0x00, 0x00, 0x00, 0x69, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x09, 0x04, 0x00, 0x00, 0x02, 0xff, 0x00, 0x00, 0x01, 0x07, 0x05, 0x81, 0x02, 0x00, 0x00, 0x00, 0x07, 0x05, 0x02, 0x02, 0x00, 0x00, 0x00, 0x09, 0x04, 0x00, 0x00, 0x02, 0xff, 0x00, 0x00, 0x01, 0x07, 0x05, 0x81, 0x02, 0x00, 0x02, 0x00, 0x07, 0x05, 0x02, 0x02, 0x00, 0x02, 0x01, 0x09, 0x04, 0x00, 0x00, 0x02, 0xff, 0x00, 0x00, 0x01, 0x07, 0x05, 0x81, 0x02, 0x00, 0x04, 0x00, 0x06, 0x30, 0x00, 0x00, 0x00, 0x00, 0x07, 0x05, 0x02, 0x02, 0x00, 0x04, 0x01, 0x06, 0x30, 0x00, 0x00, 0x00, 0x00, ];
          
        // Shown to user if test fails.
        for i in 0..size {
            if slice[i] != real[i] {
                println!("{}", i);
            }
        }

        assert_eq!(slice.to_vec(), real);
    }
}
