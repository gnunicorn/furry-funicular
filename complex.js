const ffi = require('ffi');
const ref = require('ref');
const Enum = require('enum');
const ArrayType = require('ref-array');
const StructType = require('ref-struct');

const u8 = ref.types.uint8;
const u32 = ref.types.uint32;
const i32 = ref.types.int32;
const usize = ref.types.size_t;
const bool = ref.types.bool;

// Array types
const u8Arr = ArrayType(u8);

// Pointer Types
const u8Pointer = ref.refType(u8);
const u32Pointer = ref.refType(u32);

// Structs
const FFIString = StructType({
  ptr: u8Pointer,
  len: usize,
  cap: usize
});
const ffiStringPointer = ref.refType(FFIString);

const AppKeys = StructType({
  owner_key: u8Arr,
  enc_key: u8Arr,
  sign_pk: u8Arr,
  sign_sk: u8Arr,
  enc_pk: u8Arr,
  enc_sk: u8Arr
});

const AppExchangeInfo = StructType({
  id: FFIString,
  scope: u8Pointer,
  scope_len: usize,
  scope_cap: usize,
  name: FFIString,
  vendor: FFIString
});

const Permission = new Enum({
  Read: 0,
  Insert: 1,
  Update: 2,
  Delete: 3,
  ManagePermissions: 4
});
const PermissionArrayType = ArrayType(Permission);
const AppInfo = StructType({
  info: AppExchangeInfo,
  keys: AppKeys
});

const PermissionArray = StructType({
  ptr: PermissionArrayType,
  len: usize,
  cap: usize
});

const ContainerPermissions = StructType({
  cont_name: FFIString,
  access: PermissionArray
});
const ContainersPermissionArrayType = ArrayType(ContainerPermissions);
const ContainerPermissionsArray = StructType({
  ptr: ContainersPermissionArrayType,
  len: usize,
  cap: usize
});

const AuthReq = StructType({
  app: AppExchangeInfo,
  app_container: bool,
  containers: ContainerPermissionsArray
});

const ContainersReq = StructType({
  app: AppExchangeInfo,
  containers: ContainerPermissionsArray
});


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
  encode_auth_req: [i32, [AuthReq, u32Pointer, ffiStringPointer]],
});

module.exports = function(aInfo, containers)  {
	// building complex struct types
	const appInfo = new AppExchangeInfo({
	  id: createFFIString(aInfo.id),
	  scope: ref.NULL,
	  scope_len: 0,
	  scope_cap: 0,
	  name: createFFIString(aInfo.name),
	  vendor: createFFIString(aInfo.vendor),
	});

	let containerPermissionList;

	if (containers) {
		containerPermissionList = new ContainersPermissionArrayType(
			Object.getOwnPropertyNames(containers).map((key) => {
				const permissions = new PermissionArrayType(containers[key].map((k) => Permission[k]));
				const permArray = new PermissionArray({
				  ptr: permissions,
				  len: permissions.length,
				  cap: permissions.length
				});
				return new ContainerPermissions({
				  cont_name: createFFIString(key),
				  access: permArray
				});
			})
		)

	} else {
		const permissions = new PermissionArrayType([
		  Permission.Read
		]);

		const permArray = new PermissionArray({
		  ptr: permissions,
		  len: permissions.length,
		  cap: permissions.length
		});
		const publicContainer = new ContainerPermissions({
		  cont_name: createFFIString('_public'),
		  access: permArray
		});
		//
		containerPermissionList = new ContainersPermissionArrayType([
		  publicContainer
		]);
	}

	// // console.log(containerPermissionList);
	const ctnrs = new ContainerPermissionsArray({
	  ptr: containerPermissionList,
	  len: containerPermissionList.length,
	  cap: containerPermissionList.length
	});
	// // //
	const request = new AuthReq({
	  app: appInfo,
	  ctnrs,
	  app_container: false
	});
	//
	const reqId = ref.alloc(u32);
	const encodedStr = ref.alloc(FFIString);

	const res = lib.encode_auth_req(request, reqId, encodedStr);
	// const derf = encodedStr.deref()
	// const scheme = ref.reinterpret(derf.ptr, derf.len).toString();
}