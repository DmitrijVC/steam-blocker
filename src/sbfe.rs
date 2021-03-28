extern crate libloading as ll;

use std::os::raw::c_char;
use self::ll::{Error, Library, Symbol};

const SBFE_PATH: &'static str = "bin/SBFwEditor.dll";


/// For one time lib initialization.
pub struct SBFE {
    lib: ll::Library,
}

impl SBFE {
    pub unsafe fn init() -> Result<Self, Error> {
        let ex_lib: Library = match ll::Library::new(SBFE_PATH){
            Ok(result) => result,
            Err(error) => return Err(error)
        };

        Ok( Self {
            lib: ex_lib,
        })
    }

    /// Enable the firewall rule.
    pub unsafe fn enable(&self, app_path: *const c_char) -> Result<u32, Error> {
        let enable: Symbol<unsafe extern fn(app_path: *const c_char) -> u32> = match self.lib.get(b"Enable"){
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        Ok( enable(app_path) )
    }

    /// Check if the firewall rule is enabled.
    ///
    /// Should be called only at the start to get status of the rule
    /// and then operate on it.
    pub unsafe fn is_enabled(&self) -> Result<bool, Error> {
        let enable: Symbol<unsafe extern fn() -> u32> = match self.lib.get(b"IsEnabled"){
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        let result: u32 = enable();
        Ok(result != 0)
    }
}

/// ToDo: Replace all sbfe_enable functions in the code!
/// SBFE initialized each time this function is called!
///
/// Enable the firewall rule.
pub unsafe fn sbfe_enable(app_path: *const c_char) -> Result<u32, Error> {
    let lib = match SBFE::init() {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    lib.enable(app_path)
}

/// ToDo: Replace all sbfe_is_enabled functions in the code!
/// SBFE initialized each time this function is called
///
/// Check if the firewall rule is enabled.
///
/// Should be called only at the start to get status of the rule
/// and then operate on it.
pub unsafe fn sbfe_is_enabled() -> Result<bool, Error> {
    let lib = match SBFE::init() {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    lib.is_enabled()
}
