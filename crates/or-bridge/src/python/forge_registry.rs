use pyo3::prelude::*;
use std::collections::BTreeSet;

/// Python wrapper that mirrors an `or_forge::ForgeRegistry`-style tool table.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone, Default)]
pub struct PyForgeRegistry {
    tools: BTreeSet<String>,
}

#[pymethods]
impl PyForgeRegistry {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn register(&mut self, name: &str, _handler: Py<PyAny>) {
        self.tools.insert(name.to_owned());
    }

    fn len(&self) -> usize {
        self.tools.len()
    }

    fn tool_names(&self) -> Vec<String> {
        self.tools.iter().cloned().collect()
    }
}
