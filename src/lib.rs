
extern crate ffi_utils;

extern crate rustc_serialize;
extern crate serde_json;
use std::mem;


include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

mod ffi_types;

use ffi_types::{AuthReq as OrigAuthReq, FfiAuthReq, FfiAppExchangeInfo};
use ffi_utils::FfiString;



#[no_mangle]
pub unsafe extern "C" fn encode_auth_req(req: FfiAuthReq,
                                         o_req_id: *mut u32,
                                         o_encoded: *mut FfiString)
                                         -> i32 {

    let req = OrigAuthReq::from_repr_c(req);
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

    // *o_req_id = req_id;
    // *o_encoded = FfiString::from_string(req.app.id);

    0
}