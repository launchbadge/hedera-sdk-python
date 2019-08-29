use hedera::call_params::CallParams;
use derive_more::From;
use pyo3::prelude::*;

#[pyclass(name = CallParams)]
#[derive(From)]
pub struct PyCallParams {
    pub(crate) inner: CallParams,
}

#[pymethods]
impl PyCallParams {
    #[new]
    pub fn __new__(obj: &PyRawObject, func: Option<String>) -> PyResult<()> {
        let cp = match func {
            Some(name) => {
                CallParams::new(Some(name))
            },
            None => CallParams::new(None)
        };
        obj.init(move || Self {
            inner: cp
        })
    }

    pub fn add_string(&mut self, param: String) -> PyResult<()> {
        self.inner.add_string(param);
        Ok(())
    }

    pub fn add_string_array(&mut self, param: Vec<String>) -> PyResult<()> {
        self.inner.add_string_array(param);
        Ok(())
    }

    pub fn add_fixed_string_array(&mut self, param: Vec<String>, fixed_len: usize) -> PyResult<()> {
        self.inner.add_fixed_string_array(param, fixed_len);
        Ok(())
    }

    pub fn add_bytes(&mut self, param: Vec<u8>) -> PyResult<()> {
        self.inner.add_bytes(param);
        Ok(())
    }

    pub fn add_fixed_bytes(&mut self, param: Vec<u8>, fixed_len: usize) -> PyResult<()> {
        self.inner.add_fixed_bytes(param, fixed_len);
        Ok(())
    }

    pub fn add_byte_array(&mut self, param: Vec<Vec<u8>>) -> PyResult<()> {
        self.inner.add_byte_array(param);
        Ok(())
    }

    pub fn add_fixed_byte_array(&mut self, param: Vec<Vec<u8>>, byte_len: usize) -> PyResult<()> {
        self.inner.add_fixed_byte_array(param, byte_len);
        Ok(())
    }

    pub fn add_byte_fixed_array(&mut self, param: Vec<Vec<u8>>, fixed_len: usize) -> PyResult<()> {
        self.inner.add_byte_fixed_array(param, fixed_len);
        Ok(())
    }

    pub fn add_fixed_byte_fixed_array(&mut self, param: Vec<Vec<u8>>, fixed_byte_len: usize,
                                      fixed_len: usize) -> PyResult<()> {
        self.inner.add_fixed_byte_fixed_array(param, fixed_byte_len, fixed_len);
        Ok(())
    }

    pub fn add_bool(&mut self, param: bool) -> PyResult<()> {
        self.inner.add_bool(param);
        Ok(())
    }

    pub fn add_int(&mut self, param: i64, width: usize) -> PyResult<()> {
        self.inner.add_int(param, width);
        Ok(())
    }

    pub fn add_int_array(&mut self, param: Vec<i64>, width: usize) -> PyResult<()> {
        self.inner.add_int_array(param, width);
        Ok(())
    }

    pub fn add_fixed_int_array(&mut self, param: Vec<i64>, width: usize,
                               fixed_len: usize) -> PyResult<()> {
        self.inner.add_fixed_int_array(param, width, fixed_len);
        Ok(())
    }

    pub fn add_uint(&mut self, param: u64, width: usize) -> PyResult<()> {
        self.inner.add_uint(param, width);
        Ok(())
    }

    pub fn add_uint_array(&mut self, param: Vec<u64>, width: usize) -> PyResult<()> {
        self.inner.add_uint_array(param, width);
        Ok(())
    }

    pub fn add_fixed_uint_array(&mut self, param: Vec<u64>, width: usize,
                               fixed_len: usize) -> PyResult<()> {
        self.inner.add_fixed_uint_array(param, width, fixed_len);
        Ok(())
    }

    pub fn add_address(&mut self, param: Vec<u8>) -> PyResult<()> {
        self.inner.add_address(param);
        Ok(())
    }

    pub fn add_address_string(&mut self, param: String) -> PyResult<()> {
        self.inner.add_address_string(param);
        Ok(())
    }

    pub fn add_address_array(&mut self, param: Vec<Vec<u8>>) -> PyResult<()> {
        self.inner.add_address_array(param);
        Ok(())
    }

    pub fn add_fixed_address_array(&mut self, param: Vec<Vec<u8>>,
                                   fixed_len: usize) -> PyResult<()> {
        self.inner.add_fixed_address_array(param, fixed_len);
        Ok(())
    }

    pub fn add_address_string_array(&mut self, param: Vec<String>) -> PyResult<()> {
        self.inner.add_address_string_array(param);
        Ok(())
    }

    pub fn add_fixed_address_string_array(&mut self, param: Vec<String>,
                                          fixed_len: usize) -> PyResult<()> {
        self.inner.add_fixed_address_string_array(param, fixed_len);
        Ok(())
    }

    pub fn add_function(&mut self, addr: Vec<u8>, selector: Vec<u8>) -> PyResult<()> {
        self.inner.add_function(addr, selector);
        Ok(())
    }

    pub fn add_function_string(&mut self, addr: String, selector: String) -> PyResult<()> {
        self.inner.add_function_string(addr, selector);
        Ok(())
    }

    pub fn assemble(&self) -> PyResult<Vec<u8>> {
        let out = self.inner.assemble();
        Ok(out)
    }
}