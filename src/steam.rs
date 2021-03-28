use std::path::PathBuf;
use registry::Hive;

#[cfg(target_arch = "x86_64")]
pub const REG_PATH: &'static str = r"SOFTWARE\WOW6432Node\Valve\Steam";

#[cfg(target_arch = "x86")]
pub const REG_PATH: &'static str = r"SOFTWARE\Valve\Steam";


fn get_install_path() -> Result<String, ()> {
    let hklm = Hive::LocalMachine;
    return match hklm.open(REG_PATH, registry::Security::Read) {
        Ok(reg_key) => {
            match reg_key.value("InstallPath") {
                Ok(value) => Ok(value.to_string()),
                Err(_) => Err(()),
            }
        }
        Err(_) => {
            Err(())
        }
    };
}

pub fn find_path() -> Result<String, ()> {
    let (path, path_string) = match get_install_path() {
        Ok(path) => {
            let x = format!("{}\\steam.exe", path);
            (PathBuf::new().join(x.clone()), x)
        }
        Err(_) => return Err(()),
    };

    return if path.exists() {
        Ok(path_string)
    } else {
        Err(())
    }
}
