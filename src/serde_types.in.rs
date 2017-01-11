use std::collections::{BTreeSet, HashMap};

/// Permission action
#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,  RustcEncodable, RustcDecodable)]
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

#[derive(Serialize, Deserialize, Debug,  RustcEncodable, RustcDecodable)]
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
#[derive(Serialize, Deserialize, Debug,  RustcEncodable, RustcDecodable)]
pub struct AuthReq {
    /// The application identifier for this request
    pub app: AppExchangeInfo,
    /// `true` if the app wants dedicated container for itself. `false`
    /// otherwise.
    pub app_container: bool,
    /// The list of containers it wishes to access (and desired permissions).
    pub containers: HashMap<String, BTreeSet<Permission>>,
}

