use std::collections::{BTreeSet, HashMap};
use ffi_utils::FfiString;
use std::mem;


// FFI VERSION

/// Permission action
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, RustcEncodable, RustcDecodable)]
pub enum Permission {
    /// Read
    Read,
    /// Insert
    Insert,
    /// Update
    Update,
    /// Delete
    Delete,
    /// Modify permissions
    ManagePermissions,
}

/// Represents the set of permissions for a given container
#[repr(C)]
pub struct ContainerPermissions {
    /// The UTF-8 encoded id
    pub cont_name: FfiString,

    /// The `Permission` array
    pub access: PermissionArray,
}


/// Wrapper for `ContainerPermissions` arrays to be passed across FFI boundary.
#[repr(C)]
pub struct ContainerPermissionsArray {
    /// Pointer to first byte
    pub ptr: *mut ContainerPermissions,
    /// Number of elements
    pub len: usize,
    /// Reserved by Rust allocator
    pub cap: usize,
}

/// Wrapper for `Permission` arrays to be passed across FFI boundary.
#[repr(C)]
pub struct PermissionArray {
    /// Pointer to first byte
    pub ptr: *mut Permission,
    /// Number of elements
    pub len: usize,
    /// Reserved by Rust allocator
    pub cap: usize,
}

/// Represents an application ID in the process of asking permissions
#[repr(C)]
pub struct FfiAppExchangeInfo {
    /// UTF-8 encoded id
    pub id: FfiString,

    /// Reserved by the frontend
    ///
    /// null if not present
    pub scope: *const u8,
    /// `scope`'s length.
    ///
    /// 0 if `scope` is null
    pub scope_len: usize,
    /// Used by the Rust memory allocator.
    ///
    /// 0 if `scope` is null
    pub scope_cap: usize,

    /// UTF-8 encoded application friendly-name.
    pub name: FfiString,

    /// UTF-8 encoded application provider/vendor (e.g. MaidSafe)
    pub vendor: FfiString,
}


/// Represents an authorization request
#[repr(C)]
pub struct FfiAuthReq {
    /// The application identifier for this request
    pub app: FfiAppExchangeInfo,
    /// `true` if the app wants dedicated container for itself. `false`
    /// otherwise.
    pub app_container: bool,

    /// Array of `ContainerPermissions`
    pub containers: ContainerPermissionsArray,
}



impl ContainerPermissionsArray {

    /// Consumes this `ContainerPermissionsArray` into a `Vec`
    #[allow(unsafe_code)]
    pub unsafe fn into_vec(self) -> Vec<ContainerPermissions> {
        Vec::from_raw_parts(self.ptr, self.len, self.cap)
    }
}

impl PermissionArray {

    /// Consumes this `PermissionArray` into a `Vec`
    #[allow(unsafe_code)]
    pub unsafe fn into_vec(self) -> Vec<Permission> {
        Vec::from_raw_parts(self.ptr, self.len, self.cap)
    }
}

#[derive(Eq, PartialEq, RustcEncodable, RustcDecodable, Debug)]
pub struct AppExchangeInfo {
    /// The ID. It must be unique.
    pub id: String,
    /// Reserved by the frontend.
    pub scope: Option<String>,
    /// The application friendly-name.
    pub name: String,
    /// The application provider/vendor (e.g. MaidSafe)
    pub vendor: String,
}




/// Represents an authorization request
#[derive(Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct AuthReq {
    /// The application identifier for this request
    pub app: AppExchangeInfo,
    /// `true` if the app wants dedicated container for itself. `false`
    /// otherwise.
    pub app_container: bool,
    /// The list of containers it wishes to access (and desired permissions).
    pub containers: HashMap<String, BTreeSet<Permission>>,
}



pub unsafe fn exchange_info_from_repr_c(raw: FfiAppExchangeInfo) -> AppExchangeInfo {
   let scope = match (raw.scope, raw.scope_len, raw.scope_cap) {
        (p, _, _) if p.is_null() => None,
        (p, l, c) => Some(String::from_raw_parts(p as *mut u8, l, c)),
    };

    let id = raw.id.to_string().unwrap();
    let name = raw.name.to_string().unwrap();
    let vendor = raw.vendor.to_string().unwrap();

    AppExchangeInfo {
        id: id,
        scope: scope,
        name: name,
        vendor: vendor,
    }
}

/// Constructs the object from a raw pointer.
///
/// After calling this function, the raw pointer is owned by the resulting
/// object.
#[allow(unsafe_code)]
pub unsafe fn containers_from_repr_c(raw: ContainerPermissionsArray)
                                     -> HashMap<String, BTreeSet<Permission>> {
    let mut result = HashMap::new();
    let vec = raw.into_vec();

    for raw in vec {
        let cont_name = raw.cont_name.to_string();

        let _ = result.insert(cont_name.unwrap(), raw.access.into_vec().into_iter().collect());
    }

    result
}



impl AuthReq {
    /// Constructs the object from the FFI counterpart.
    ///
    /// After calling this function, the subobjects memory is owned by the
    /// resulting object.
    #[allow(unsafe_code)]
    pub unsafe fn from_repr_c(repr_c: FfiAuthReq) -> Self {
        
        let FfiAuthReq { app, app_container, containers } = repr_c;
        AuthReq {
            app: exchange_info_from_repr_c(app),
            app_container: app_container,
            containers: containers_from_repr_c(containers),
        }
    }
}