
extern crate safe_core;
extern crate ffi_utils;

extern crate serde_json;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use ffi_utils::FfiString;
use safe_core::ipc::req::ffi::AuthReq as FfiAuthReq;
use safe_core::ipc::AuthReq as OrigAuthReq;

#[no_mangle]
pub unsafe extern "C" fn encode_auth_req(req: FfiAuthReq,
                                         o_req_id: *mut u32,
                                         o_encoded: *mut FfiString)
                                         -> i32 {
    let req = OrigAuthReq::from_repr_c(req).unwrap();
    let req_id = gen_req_id();
    // *o_req_id = req_id;
    // *o_encoded = FfiString::from_string(req.app.id);

    0
}


/// Generate unique request ID.
pub fn gen_req_id() -> u32 {
    // Generate the number in range 1..MAX inclusive.
    1
}


#[no_mangle]
pub unsafe extern "C" fn gen_auth_uri(inp: FfiString,
                                         o_req_id: *mut u32,
                                         o_encoded: *mut FfiString)
                                         -> i32 {
    let serialized = inp.to_string().unwrap();
    let req: AuthReq = serde_json::from_str(&serialized).unwrap();
    let req_id = gen_req_id();

    // *o_req_id = req_id;
    // *o_encoded = FfiString::from_string(req.app.id);

    0
}