# linux-usb-functionfs-sys
Naive bindgen translation of the Linux functionfs and USB specification chaper
9 headers.

Also included is a simple test to check the struct layout against an example.

Ideally in the future the entire set of Linux headers will be namespaced.

## TODO
Compile `ffs-desc-dump.c` and use it to dump the descriptor layout for use in
the test.
