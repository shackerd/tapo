use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, DefaultStateType, TapoResponseExt};

/// Device info of Tapo P100, P105, P110 and P115. Superset of [`crate::responses::GenericDeviceInfoResult`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct PlugDeviceInfoResult {
    //
    // Inherited from GenericDeviceInfoResult
    //
    pub device_id: String,
    pub r#type: String,
    pub model: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub fw_id: String,
    pub fw_ver: String,
    pub oem_id: String,
    pub mac: String,
    pub ip: String,
    pub ssid: String,
    pub signal_level: u8,
    pub rssi: i16,
    pub specs: String,
    pub lang: String,
    pub device_on: bool,
    /// The time in seconds this device has been ON since the last state change (ON/OFF).
    pub on_time: u64,
    pub overheated: bool,
    pub nickname: String,
    pub avatar: String,
    pub has_set_location_info: bool,
    pub region: Option<String>,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub time_diff: Option<i64>,
    //
    // Unique to this device
    //
    /// The default state of a device to be used when internet connectivity is lost after a power cut.
    pub default_states: PlugDefaultState,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl PlugDeviceInfoResult {
    /// Get all the properties of this result as a dictionary.
    pub fn to_dict<'a>(&self, py: pyo3::Python<'a>) -> pyo3::PyResult<&'a pyo3::types::PyDict> {
        let serialized = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        if let Some(object) = serialized.as_object() {
            let dict = crate::python::serde_object_to_py_dict(py, object)?;

            Ok(dict)
        } else {
            Ok(pyo3::types::PyDict::new(py))
        }
    }
}

impl TapoResponseExt for PlugDeviceInfoResult {}

impl DecodableResultExt for PlugDeviceInfoResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;
        self.nickname = decode_value(&self.nickname)?;

        Ok(self)
    }
}

/// Plug Default State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct PlugDefaultState {
    pub r#type: DefaultStateType,
    pub state: PlugState,
}

/// Plug State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct PlugState {
    pub on: Option<bool>,
}