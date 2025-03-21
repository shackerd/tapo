use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tapo::responses::{T300Log, TriggerLogsResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(get_all)]
#[allow(missing_docs)]
pub struct TriggerLogsT300Result {
    start_id: u64,
    sum: u64,
    logs: Vec<T300Log>,
}

impl From<TriggerLogsResult<T300Log>> for TriggerLogsT300Result {
    fn from(result: TriggerLogsResult<T300Log>) -> Self {
        Self {
            start_id: result.start_id,
            sum: result.sum,
            logs: result.logs,
        }
    }
}

#[pyo3::pymethods]
impl TriggerLogsT300Result {
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        tapo::python::serde_object_to_py_dict(py, &value)
    }
}
