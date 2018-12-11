// If a struct X has a single field called `inner` that impls ToString,
// this macro can be used on that struct (`def_str!(X);`) to allow it
// to be str()'d on the Python side.
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

macro_rules! def_query {
    (@into Vec<u8>) => {
        Into::into
    };

    (@into Vec<$ty:ty>) => {
        |values| values.clone().into_iter().map(Into::into).collect()
    };

    (@into $ty:ty) => {
        Into::into
    };

    ($query:tt($param:ty) -> $($ty:tt)+) => {
        mashup! {
            m["py"] = Py $query;
        }

        m! {
            #[pyo3::prelude::pyclass(name = $query)]
            pub struct "py" {
                inner: hedera::query::Query<$query>,
            }

            impl "py" {
                pub fn new(client: &hedera::Client, _1: $param) -> Self {
                    Self {
                        inner: $query::new(client, _1),
                    }
                }
            }

            #[pyo3::prelude::pymethods]
            impl "py" {
                pub fn get(&mut self) -> pyo3::PyResult<$($ty)+> {
                    self.inner
                        .get()
                        .map(def_query!(@into $($ty)+))
                        .map_err(crate::errors::PyException)
                }

                pub fn cost(&mut self) -> pyo3::PyResult<u64> {
                    self.inner.cost().map_err(crate::errors::PyException)
                }
            }
        }
    };
}
