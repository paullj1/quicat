
mod client;
mod server;

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn server(cert_p: *const c_char, key_p: *const c_char, address_p: *const c_char, port: u16) {
    unsafe {
        let cert = CStr::from_ptr(cert_p);
        let cert = cert.to_str().expect("Invalid cert");

        let key = CStr::from_ptr(key_p);
        let key = key.to_str().expect("Invalid key");

        let address = CStr::from_ptr(address_p);
        let address = address.to_str().expect("Invalid address");

        let runtime = tokio::runtime::Runtime::new().unwrap();

        let _ = runtime.block_on(server::_server(cert, key, address, port));
    }
}

#[no_mangle]
pub extern "C" fn client(cert_p: *const c_char, address_p: *const c_char, port: u16) {
    unsafe {
        let cert = CStr::from_ptr(cert_p);
        let cert = cert.to_str().expect("Invalid cert");

        let address = CStr::from_ptr(address_p);
        let address = address.to_str().expect("Invalid address");

        let runtime = tokio::runtime::Runtime::new().unwrap();

        let _ = runtime.block_on(client::_client(cert, address, port));
    }
}
