use alvr_common::{
    ToAny,
    anyhow::{Result, bail},
};
use encoding_rs_io::DecodeReaderBytes;
use serde_json as json;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

fn openvr_source_file_path() -> Result<PathBuf> {
    let path = if cfg!(windows) {
        dirs::cache_dir()
    } else {
        dirs::config_dir()
    }
    .to_any()?
    .join("openvr/openvrpaths.vrpath");

    if path.exists() {
        Ok(path)
    } else {
        bail!("{} does not exist", path.to_string_lossy())
    }
}

pub fn steamvr_settings_file_path() -> Result<PathBuf> {
    let path = if cfg!(windows) {
        // N.B. if ever implementing this: given Steam can be installed on another
        // drive, etc., this should probably start by looking at Windows registry keys.
        bail!("Not implemented for Windows.") // Original motive for implementation had little reason for Windows.
    } else {
        dirs::data_dir()
    }
    .to_any()?
    .join("Steam/config/steamvr.vrsettings");

    if path.exists() {
        Ok(path)
    } else {
        bail!("{} does not exist", path.to_string_lossy())
    }
}

pub fn load_openvr_paths_json() -> Result<json::Value> {
    let file = File::open(openvr_source_file_path()?)?;

    let mut file_content_decoded = String::new();
    DecodeReaderBytes::new(&file).read_to_string(&mut file_content_decoded)?;

    let value = json::from_str(&file_content_decoded)?;

    Ok(value)
}

pub fn save_openvr_paths_json(openvr_paths: &json::Value) -> Result<()> {
    let file_content = json::to_string_pretty(openvr_paths)?;

    fs::write(openvr_source_file_path()?, file_content)?;

    Ok(())
}

pub fn from_openvr_paths(paths: &json::Value) -> Vec<std::path::PathBuf> {
    let Some(paths_vec) = paths.as_array() else {
        return vec![];
    };

    paths_vec
        .iter()
        .filter_map(json::Value::as_str)
        .map(|s| PathBuf::from(s.replace(r"\\", r"\")))
        .collect()
}

pub fn to_openvr_paths(paths: &[PathBuf]) -> json::Value {
    let paths_vec = paths
        .iter()
        .map(|p| p.to_string_lossy().into())
        .map(json::Value::String) // backslashes gets duplicated here
        .collect::<Vec<_>>();

    json::Value::Array(paths_vec)
}

fn get_single_openvr_path(path_type: &str) -> Result<PathBuf> {
    let openvr_paths_json = load_openvr_paths_json()?;
    let paths_json = openvr_paths_json.get(path_type).to_any()?;
    from_openvr_paths(paths_json).first().cloned().to_any()
}

pub fn steamvr_root_dir() -> Result<PathBuf> {
    get_single_openvr_path("runtime")
}
