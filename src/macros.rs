// If a struct X has a single field called `inner` that impls ToString,
// this macro can be used on that struct (`def_str!(X);`) to allow it
// to be str()'d on the Python side.
#[macro_export]
macro_rules! def_str {
    ($name:ident) => {
        impl<'p> pyo3::basic::PyObjectStrProtocol<'p> for $name {
            type Result = PyResult<Self::Success>;
            type Success = String;
        }

        impl<'p> pyo3::class::basic::PyObjectProtocol<'p> for $name {
            fn __str__(&'p self) -> <$name as pyo3::basic::PyObjectStrProtocol>::Result {
                Ok(self.inner.to_string())
            }
        }
    };
}
