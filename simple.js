const ffi = require('ffi');
const ref = require('ref');
const StructType = require('ref-struct');


const u8 = ref.types.uint8;
const i32 = ref.types.uint32;
const usize = ref.types.size_t;
const u8Pointer = ref.refType(u8);

// Structs
const FFIString = StructType({
  ptr: u8Pointer,
  len: usize,
  cap: usize
});
const ffiStringPointer = ref.refType(FFIString);

const createFFIString = (str) => {
  const buff = new Buffer(str);
  return new FFIString({
    ptr: buff,
    len: buff.length,
    cap: buff.length
  });
};

// Done with defitions
const lib = ffi.Library('target/debug/libjson_performance', {
  gen_auth_uri: [i32, [FFIString, i32, ffiStringPointer]],
});



module.exports = function(appInfo, containers) {

	const reqId = ref.alloc(i32);
	const encodedStr = ref.alloc(FFIString);

	const request = createFFIString(JSON.stringify({
		app: {
			id: appInfo.id,
			name: appInfo.name,
			scope: appInfo.scope || null,
			vendor: appInfo.vendor
		},
		app_container: true,
		containers: containers || {
			'_private': ['Read']
		}

	}))

	const res = lib.gen_auth_uri(request, reqId, encodedStr);
	// const derf = encodedStr.deref()
	// const scheme = ref.reinterpret(derf.ptr, derf.len).toString();
}