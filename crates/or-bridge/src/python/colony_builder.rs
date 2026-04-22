use pyo3::prelude::*;

/// Python wrapper that mirrors the `or-colony` orchestrator builder pattern.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone, Default)]
pub struct PyColonyBuilder {
    members: Vec<(String, String)>,
}

#[pymethods]
impl PyColonyBuilder {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn add_member(&mut self, name: &str, role: &str) {
        self.members.push((name.to_owned(), role.to_owned()));
    }

    fn member_names(&self) -> Vec<String> {
        self.members.iter().map(|(name, _)| name.clone()).collect()
    }
}
