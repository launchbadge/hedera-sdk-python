#![feature(prelude_import)]
#![no_std]
#![feature(specialization)]
#![recursion_limit = "256"]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter)]
// todo: file an issue in pyo3 about this
#![allow(clippy::cast_ptr_alignment)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;

#[macro_use]
extern crate mashup;

#[macro_use]
mod macros {

    macro_rules! def_str(( $ name : ident ) => {
                         impl < 'p > pyo3 :: basic :: PyObjectStrProtocol < 'p
                         > for $ name {
                         type Result = PyResult < Self :: Success > ; type
                         Success = String ; } impl < 'p > pyo3 :: class ::
                         basic :: PyObjectProtocol < 'p > for $ name {
                         fn __str__ ( & 'p self ) -> < $ name as pyo3 :: basic
                         :: PyObjectStrProtocol > :: Result {
                         Ok ( self . inner . to_string (  ) ) } } } ;);
    macro_rules! def_query(( $ query : tt ( $ param : ty ) -> $ ty : ty ) => {
                           mashup ! { m [ "py" ] = Py $ query ; } m ! {
                           # [ pyo3 :: prelude :: pyclass ( name = $ query ) ]
                           pub struct "py" {
                           inner : hedera :: query :: Query < $ query > , }
                           impl "py" {
                           pub fn new (
                           client : & hedera :: Client , _1 : $ param ) ->
                           Self {
                           Self { inner : $ query :: new ( client , _1 ) , } }
                           } # [ pyo3 :: prelude :: pymethods ] impl "py" {
                           pub fn get ( & mut self ) -> pyo3 :: PyResult < $
                           ty > {
                           self . inner . get (  ) . map ( Into :: into ) .
                           map_err ( crate :: errors :: PyException ) } pub fn
                           cost ( & mut self ) -> pyo3 :: PyResult < u64 > {
                           self . inner . cost (  ) . map_err (
                           crate :: errors :: PyException ) } } } } ;);
}
mod account_info {
    use crate::{timestamp::PyDateTime, PyAccountId, PyClaim, PyPublicKey};
    use chrono::{DateTime, Utc};
    use derive_more::From;
    use hedera::AccountInfo;
    use itertools::Itertools;
    use pyo3::prelude::*;
    pub struct PyAccountInfo {
        pub(crate) inner: AccountInfo,
    }
    impl ::std::convert::From<(AccountInfo)> for PyAccountInfo {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (AccountInfo)) -> PyAccountInfo {
            PyAccountInfo { inner: original }
        }
    }
    impl ::pyo3::typeob::PyTypeInfo for PyAccountInfo {
        type Type = PyAccountInfo;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "AccountInfo";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyAccountInfo>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyAccountInfo>()
                - 1)
                / ::std::mem::align_of::<PyAccountInfo>()
                * ::std::mem::align_of::<PyAccountInfo>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyAccountInfo {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyAccountInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyAccountInfo {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyAccountInfo as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyAccountInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyAccountInfo {
        fn account_id(&self) -> PyResult<String> {
            Ok(self.inner.account_id.to_string())
        }
        fn contract_account_id(&self) -> PyResult<String> {
            Ok(self.inner.contract_account_id.to_string())
        }
        fn deleted(&self) -> PyResult<bool> {
            Ok(self.inner.deleted as bool)
        }
        fn proxy_account_id(&self) -> PyResult<Option<PyAccountId>> {
            Ok(self.inner.proxy_account_id.map(Into::into))
        }
        fn proxy_fraction(&self) -> PyResult<i32> {
            Ok(self.inner.proxy_fraction as i32)
        }
        fn proxy_received(&self) -> PyResult<i64> {
            Ok(self.inner.proxy_received as i64)
        }
        fn key(&self) -> PyResult<PyPublicKey> {
            Ok(self.inner.key.clone().into())
        }
        fn balance(&self) -> PyResult<u64> {
            Ok(self.inner.balance as u64)
        }
        fn generate_send_record_threshold(&self) -> PyResult<u64> {
            Ok(self.inner.generate_send_record_threshold as u64)
        }
        fn generate_receive_record_threshold(&self) -> PyResult<u64> {
            Ok(self.inner.generate_receive_record_threshold as u64)
        }
        fn receiver_signature_required(&self) -> PyResult<bool> {
            Ok(self.inner.receiver_signature_required as bool)
        }
        fn claims(&self) -> PyResult<Vec<PyClaim>> {
            let claims = self.inner.claims.clone().into_iter().map_into().collect();
            Ok(claims)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyAccountInfo {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.account_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.account_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "account_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.contract_account_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.contract_account_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "contract_account_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.deleted()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.deleted() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "deleted",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.proxy_account_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.proxy_account_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "proxy_account_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.proxy_fraction()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.proxy_fraction() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "proxy_fraction",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.proxy_received()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.proxy_received() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "proxy_received",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.key()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.key() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "key",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.balance()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.balance() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "balance",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str =
                            "PyAccountInfo.generate_send_record_threshold()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.generate_send_record_threshold() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "generate_send_record_threshold",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str =
                            "PyAccountInfo.generate_receive_record_threshold()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.generate_receive_record_threshold() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "generate_receive_record_threshold",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str =
                            "PyAccountInfo.receiver_signature_required()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.receiver_signature_required() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "receiver_signature_required",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyAccountInfo.claims()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyAccountInfo>(_slf);
                        match _slf.claims() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "claims",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod claim {
    use crate::{crypto::PyPublicKey, id::PyAccountId};
    use derive_more::From;
    use hedera::Claim;
    use pyo3::prelude::*;
    pub struct PyClaim {
        inner: Claim,
    }
    impl ::std::convert::From<(Claim)> for PyClaim {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (Claim)) -> PyClaim {
            PyClaim { inner: original }
        }
    }
    impl ::pyo3::typeob::PyTypeInfo for PyClaim {
        type Type = PyClaim;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "Claim";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize = { Self::OFFSET as usize + ::std::mem::size_of::<PyClaim>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyClaim>()
                - 1)
                / ::std::mem::align_of::<PyClaim>()
                * ::std::mem::align_of::<PyClaim>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyClaim {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyClaim {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyClaim {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyClaim as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyClaim {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyClaim {
        pub fn account(&self) -> PyResult<PyAccountId> {
            Ok(self.inner.account.into())
        }
        pub fn hash(&self) -> PyResult<Vec<u8>> {
            Ok(self.inner.hash.clone())
        }
        pub fn keys(&self) -> PyResult<Vec<PyPublicKey>> {
            let keys = self
                .inner
                .keys
                .clone()
                .into_iter()
                .map(|key| PyPublicKey { inner: key })
                .collect();
            Ok(keys)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyClaim {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyClaim.account()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyClaim>(_slf);
                        match _slf.account() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "account",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyClaim.hash()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyClaim>(_slf);
                        match _slf.hash() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "hash",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyClaim.keys()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyClaim>(_slf);
                        match _slf.keys() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "keys",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod client {
    use super::{
        errors::PyValueError, query_crypto_get_account_balance::*, query_crypto_get_info::*,
        query_file_get_contents::*, query_get_transaction_receipt::*,
    };
    use crate::{
        id::{PyAccountId, PyFileId},
        timestamp::PyDateTime,
        transaction_id::PyTransactionId,
    };
    use chrono::Utc;
    use hedera::{AccountId, Client, FileId, TransactionId};
    use pyo3::prelude::*;
    use std::rc::Rc;
    use try_from::TryInto;
    pub struct PyClient {
        inner: Rc<Client>,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyClient {
        type Type = PyClient;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "Client";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize = { Self::OFFSET as usize + ::std::mem::size_of::<PyClient>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyClient>()
                - 1)
                / ::std::mem::align_of::<PyClient>()
                * ::std::mem::align_of::<PyClient>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyClient {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyClient {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyClient {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyClient as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyClient {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyClient {
        pub fn __new__(obj: &PyRawObject, address: &str) -> PyResult<()> {
            let client = Client::new(address).map_err(PyValueError)?;
            obj.init(move |_| Self {
                inner: Rc::new(client),
            })
        }
        #[doc = " account(self, id: str) -> PartialAccountMessage"]
        #[doc = " --"]
        #[doc = ""]
        #[doc = " Access available operations on a single crypto-currency account."]
        pub fn account(&self, id: &PyAccountId) -> PyResult<PyPartialAccountMessage> {
            Ok(PyPartialAccountMessage {
                client: Rc::clone(&self.inner),
                account: id.inner,
            })
        }
        #[doc = " transaction(self, id: str) -> PartialTransactionMessage"]
        #[doc = " --"]
        #[doc = ""]
        #[doc = " Access available operations on a single transaction."]
        pub fn transaction(&self, id: &PyTransactionId) -> PyResult<PyPartialTransactionMessage> {
            Ok(PyPartialTransactionMessage {
                client: Rc::clone(&self.inner),
                transaction: id.inner.clone(),
            })
        }
        #[doc = " file(self, id: str) -> PartialFileMessage"]
        #[doc = " --"]
        #[doc = ""]
        #[doc = " Access available operations on a single file."]
        pub fn file(&self, id: &PyFileId) -> PyResult<PyPartialFileMessage> {
            Ok(PyPartialFileMessage {
                client: Rc::clone(&self.inner),
                file: id.inner,
            })
        }
        fn now(&self) -> PyResult<PyDateTime> {
            Ok(PyDateTime(Utc::now()))
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyClient {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::New({
                    #[allow(unused_mut)]
                    unsafe extern "C" fn __wrap(
                        _cls: *mut ::pyo3::ffi::PyTypeObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        use pyo3::typeob::PyTypeInfo;
                        const _LOCATION: &'static str = "PyClient.__new__()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        match ::pyo3::typeob::PyRawObject::new(_py, PyClient::type_object(), _cls) {
                            Ok(_obj) => {
                                let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                                let _kwargs: Option<&::pyo3::types::PyDict> =
                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                let _result: ::pyo3::PyResult<
                                    <PyResult<()> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                                > = {
                                    const _PARAMS:
                                        &'static [::pyo3::derive_utils::ParamDescription<'static>] =
                                        &[::pyo3::derive_utils::ParamDescription {
                                            name: "address",
                                            is_optional: false,
                                            kw_only: false,
                                        }];
                                    let mut _output = [None];
                                    match ::pyo3::derive_utils::parse_fn_args(
                                        Some(_LOCATION),
                                        _PARAMS,
                                        &_args,
                                        _kwargs,
                                        false,
                                        false,
                                        &mut _output,
                                    ) {
                                        Ok(_) => {
                                            let mut _iter = _output.iter();
                                            ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                                   {
                                                                                                                                                                       ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(PyClient::__new__(&_obj,
                                                                                                                                                                                                                                                    arg0))
                                                                                                                                                                   })
                                        }
                                        Err(err) => Err(err),
                                    }
                                };
                                match _result {
                                    Ok(_) => ::pyo3::IntoPyPointer::into_ptr(_obj),
                                    Err(e) => {
                                        e.restore(_py);
                                        ::std::ptr::null_mut()
                                    }
                                }
                            }
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "__new__",
                        ml_meth: ::pyo3::class::PyMethodType::PyNewFunc(__wrap),
                        ml_flags: ::pyo3::ffi::METH_VARARGS | ::pyo3::ffi::METH_KEYWORDS,
                        ml_doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyClient.account()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyClient>(_slf);
                        let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                        let _kwargs: Option<&::pyo3::types::PyDict> =
                            _py.from_borrowed_ptr_or_opt(_kwargs);
                        let _result:
                                                                         ::pyo3::PyResult<<PyResult<PyPartialAccountMessage>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         const _PARAMS:
                                                                               &'static [::pyo3::derive_utils::ParamDescription<'static>]
                                                                               =
                                                                             &[::pyo3::derive_utils::ParamDescription{name:
                                                                                                                          "id",
                                                                                                                      is_optional:
                                                                                                                          false,
                                                                                                                      kw_only:
                                                                                                                          false,}];
                                                                         let mut _output =
                                                                             [None];
                                                                         match ::pyo3::derive_utils::parse_fn_args(Some(_LOCATION),
                                                                                                                   _PARAMS,
                                                                                                                   &_args,
                                                                                                                   _kwargs,
                                                                                                                   false,
                                                                                                                   false,
                                                                                                                   &mut _output)
                                                                             {
                                                                             Ok(_)
                                                                             =>
                                                                             {
                                                                                 let mut _iter =
                                                                                     _output.iter();
                                                                                 ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                              {
                                                                                                                                                                  ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.account(arg0))
                                                                                                                                                              })
                                                                             }
                                                                             Err(err)
                                                                             =>
                                                                             Err(err),
                                                                         }
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef{ml_name:
                                                                                            "account",
                                                                                        ml_meth:
                                                                                            ::pyo3::class::PyMethodType::PyCFunctionWithKeywords(__wrap),
                                                                                        ml_flags:
                                                                                            ::pyo3::ffi::METH_VARARGS
                                                                                                |
                                                                                                ::pyo3::ffi::METH_KEYWORDS,
                                                                                        ml_doc:
                                                                                            "account(self, id: str) -> PartialAccountMessage\n--\n\nAccess available operations on a single crypto-currency account.\u{0}",}
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyClient.transaction()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyClient>(_slf);
                        let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                        let _kwargs: Option<&::pyo3::types::PyDict> =
                            _py.from_borrowed_ptr_or_opt(_kwargs);
                        let _result:
                                                                         ::pyo3::PyResult<<PyResult<PyPartialTransactionMessage>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         const _PARAMS:
                                                                               &'static [::pyo3::derive_utils::ParamDescription<'static>]
                                                                               =
                                                                             &[::pyo3::derive_utils::ParamDescription{name:
                                                                                                                          "id",
                                                                                                                      is_optional:
                                                                                                                          false,
                                                                                                                      kw_only:
                                                                                                                          false,}];
                                                                         let mut _output =
                                                                             [None];
                                                                         match ::pyo3::derive_utils::parse_fn_args(Some(_LOCATION),
                                                                                                                   _PARAMS,
                                                                                                                   &_args,
                                                                                                                   _kwargs,
                                                                                                                   false,
                                                                                                                   false,
                                                                                                                   &mut _output)
                                                                             {
                                                                             Ok(_)
                                                                             =>
                                                                             {
                                                                                 let mut _iter =
                                                                                     _output.iter();
                                                                                 ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                              {
                                                                                                                                                                  ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.transaction(arg0))
                                                                                                                                                              })
                                                                             }
                                                                             Err(err)
                                                                             =>
                                                                             Err(err),
                                                                         }
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef{ml_name:
                                                                                            "transaction",
                                                                                        ml_meth:
                                                                                            ::pyo3::class::PyMethodType::PyCFunctionWithKeywords(__wrap),
                                                                                        ml_flags:
                                                                                            ::pyo3::ffi::METH_VARARGS
                                                                                                |
                                                                                                ::pyo3::ffi::METH_KEYWORDS,
                                                                                        ml_doc:
                                                                                            "transaction(self, id: str) -> PartialTransactionMessage\n--\n\nAccess available operations on a single transaction.\u{0}",}
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyClient.file()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyClient>(_slf);
                        let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                        let _kwargs: Option<&::pyo3::types::PyDict> =
                            _py.from_borrowed_ptr_or_opt(_kwargs);
                        let _result:
                                                                         ::pyo3::PyResult<<PyResult<PyPartialFileMessage>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         const _PARAMS:
                                                                               &'static [::pyo3::derive_utils::ParamDescription<'static>]
                                                                               =
                                                                             &[::pyo3::derive_utils::ParamDescription{name:
                                                                                                                          "id",
                                                                                                                      is_optional:
                                                                                                                          false,
                                                                                                                      kw_only:
                                                                                                                          false,}];
                                                                         let mut _output =
                                                                             [None];
                                                                         match ::pyo3::derive_utils::parse_fn_args(Some(_LOCATION),
                                                                                                                   _PARAMS,
                                                                                                                   &_args,
                                                                                                                   _kwargs,
                                                                                                                   false,
                                                                                                                   false,
                                                                                                                   &mut _output)
                                                                             {
                                                                             Ok(_)
                                                                             =>
                                                                             {
                                                                                 let mut _iter =
                                                                                     _output.iter();
                                                                                 ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                              {
                                                                                                                                                                  ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.file(arg0))
                                                                                                                                                              })
                                                                             }
                                                                             Err(err)
                                                                             =>
                                                                             Err(err),
                                                                         }
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef{ml_name:
                                                                                            "file",
                                                                                        ml_meth:
                                                                                            ::pyo3::class::PyMethodType::PyCFunctionWithKeywords(__wrap),
                                                                                        ml_flags:
                                                                                            ::pyo3::ffi::METH_VARARGS
                                                                                                |
                                                                                                ::pyo3::ffi::METH_KEYWORDS,
                                                                                        ml_doc:
                                                                                            "file(self, id: str) -> PartialFileMessage\n--\n\nAccess available operations on a single file.\u{0}",}
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyClient.now()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyClient>(_slf);
                        match _slf.now() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "now",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
    pub struct PyPartialAccountMessage {
        client: Rc<Client>,
        account: AccountId,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyPartialAccountMessage {
        type Type = PyPartialAccountMessage;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "PartialAccountMessage";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyPartialAccountMessage>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyPartialAccountMessage>()
                - 1)
                / ::std::mem::align_of::<PyPartialAccountMessage>()
                * ::std::mem::align_of::<PyPartialAccountMessage>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyPartialAccountMessage {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyPartialAccountMessage {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyPartialAccountMessage {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyPartialAccountMessage as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyPartialAccountMessage {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyPartialAccountMessage {
        #[doc = " balance(self) -> QueryCryptoGetAccountBalance"]
        #[doc = " --"]
        #[doc = ""]
        #[doc = " Get the balance of a crypto-currency account."]
        #[doc = ""]
        #[doc = " This returns only the balance, so it is a smaller and faster reply than"]
        #[doc = " :py:method:`hedera.PartialAccountMessage.info`."]
        pub fn balance(&self) -> PyResult<PyQueryCryptoGetAccountBalance> {
            Ok(PyQueryCryptoGetAccountBalance::new(
                &self.client,
                self.account,
            ))
        }
        pub fn info(&self) -> PyResult<PyQueryCryptoGetInfo> {
            Ok(PyQueryCryptoGetInfo::new(&self.client, self.account))
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyPartialAccountMessage {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyPartialAccountMessage.balance()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyPartialAccountMessage>(_slf);
                        let _result:
                                                                         ::pyo3::PyResult<<PyResult<PyQueryCryptoGetAccountBalance>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.balance())
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef{ml_name:
                                                                                            "balance",
                                                                                        ml_meth:
                                                                                            ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                                                                                        ml_flags:
                                                                                            ::pyo3::ffi::METH_NOARGS,
                                                                                        ml_doc:
                                                                                            "balance(self) -> QueryCryptoGetAccountBalance\n--\n\nGet the balance of a crypto-currency account.\n\nThis returns only the balance, so it is a smaller and faster reply than\n:py:method:`hedera.PartialAccountMessage.info`.\u{0}",}
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyPartialAccountMessage.info()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyPartialAccountMessage>(_slf);
                        let _result:
                                                                         ::pyo3::PyResult<<PyResult<PyQueryCryptoGetInfo>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.info())
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "info",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
    pub struct PyPartialTransactionMessage {
        client: Rc<Client>,
        transaction: TransactionId,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyPartialTransactionMessage {
        type Type = PyPartialTransactionMessage;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "PartialTransactionMessage";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize = {
            Self::OFFSET as usize + ::std::mem::size_of::<PyPartialTransactionMessage>() + 0 + 0
        };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyPartialTransactionMessage>()
                - 1)
                / ::std::mem::align_of::<PyPartialTransactionMessage>()
                * ::std::mem::align_of::<PyPartialTransactionMessage>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyPartialTransactionMessage {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyPartialTransactionMessage {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyPartialTransactionMessage {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyPartialTransactionMessage as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyPartialTransactionMessage {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyPartialTransactionMessage {
        #[doc = " receipt(self) -> QueryGetTransactionReceipt"]
        #[doc = " --"]
        #[doc = ""]
        #[doc = " Get the receipt of the transaction."]
        #[doc = ""]
        #[doc = " Once a transaction reaches consensus, then information about whether it succeeded or"]
        #[doc = " failed will be available until the end of the receipt period (180 seconds)."]
        pub fn receipt(&self) -> PyResult<PyQueryGetTransactionReceipt> {
            Ok(PyQueryGetTransactionReceipt::new(
                &self.client,
                self.transaction.clone(),
            ))
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyPartialTransactionMessage {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyPartialTransactionMessage.receipt()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyPartialTransactionMessage>(_slf);
                        let _result:
                                                                         ::pyo3::PyResult<<PyResult<PyQueryGetTransactionReceipt>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.receipt())
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef{ml_name:
                                                                                            "receipt",
                                                                                        ml_meth:
                                                                                            ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                                                                                        ml_flags:
                                                                                            ::pyo3::ffi::METH_NOARGS,
                                                                                        ml_doc:
                                                                                            "receipt(self) -> QueryGetTransactionReceipt\n--\n\nGet the receipt of the transaction.\n\nOnce a transaction reaches consensus, then information about whether it succeeded or\nfailed will be available until the end of the receipt period (180 seconds).\u{0}",}
                }),
            ];
            METHODS
        }
    }
    pub struct PyPartialFileMessage {
        client: Rc<Client>,
        file: FileId,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyPartialFileMessage {
        type Type = PyPartialFileMessage;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "PartialFileMessage";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyPartialFileMessage>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyPartialFileMessage>()
                - 1)
                / ::std::mem::align_of::<PyPartialFileMessage>()
                * ::std::mem::align_of::<PyPartialFileMessage>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyPartialFileMessage {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyPartialFileMessage {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyPartialFileMessage {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyPartialFileMessage as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyPartialFileMessage {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyPartialFileMessage {
        pub fn contents(&self) -> PyResult<PyQueryFileGetContents> {
            Ok(PyQueryFileGetContents::new(&self.client, self.file))
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyPartialFileMessage {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyPartialFileMessage.contents()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyPartialFileMessage>(_slf);
                        let _result:
                                                                         ::pyo3::PyResult<<PyResult<PyQueryFileGetContents>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.contents())
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "contents",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod contract_info {
    use crate::{crypto::PyPublicKey, id::PyAccountId};
    use derive_more::From;
    use hedera::ContractInfo;
    use pyo3::prelude::*;
    pub struct PyContractInfo {
        pub(crate) inner: ContractInfo,
    }
    impl ::std::convert::From<(ContractInfo)> for PyContractInfo {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (ContractInfo)) -> PyContractInfo {
            PyContractInfo { inner: original }
        }
    }
    impl ::pyo3::typeob::PyTypeInfo for PyContractInfo {
        type Type = PyContractInfo;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "ContractInfo";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyContractInfo>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyContractInfo>()
                - 1)
                / ::std::mem::align_of::<PyContractInfo>()
                * ::std::mem::align_of::<PyContractInfo>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyContractInfo {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyContractInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyContractInfo {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyContractInfo as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyContractInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyContractInfo {
        pub fn account_id(&self) -> PyResult<PyAccountId> {
            Ok(PyAccountId {
                inner: self.inner.account_id,
            })
        }
        pub fn contract_account_id(&self) -> PyResult<String> {
            Ok(self.inner.contract_account_id.clone())
        }
        pub fn admin_key(&self) -> PyResult<Option<PyPublicKey>> {
            Ok(self
                .inner
                .admin_key
                .clone()
                .map(|key| PyPublicKey { inner: key }))
        }
        pub fn storage(&self) -> PyResult<i64> {
            Ok(self.inner.storage)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyContractInfo {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyContractInfo.account_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyContractInfo>(_slf);
                        match _slf.account_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "account_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyContractInfo.contract_account_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyContractInfo>(_slf);
                        match _slf.contract_account_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "contract_account_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyContractInfo.admin_key()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyContractInfo>(_slf);
                        match _slf.admin_key() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "admin_key",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyContractInfo.storage()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyContractInfo>(_slf);
                        match _slf.storage() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "storage",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod crypto {
    use super::errors::PyValueError;
    use derive_more::From;
    use hedera::{PublicKey, SecretKey, Signature};
    use pyo3::prelude::*;
    pub struct PyPublicKey {
        pub(crate) inner: PublicKey,
    }
    impl ::std::convert::From<(PublicKey)> for PyPublicKey {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (PublicKey)) -> PyPublicKey {
            PyPublicKey { inner: original }
        }
    }
    impl ::pyo3::typeob::PyTypeInfo for PyPublicKey {
        type Type = PyPublicKey;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "PublicKey";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyPublicKey>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyPublicKey>()
                - 1)
                / ::std::mem::align_of::<PyPublicKey>()
                * ::std::mem::align_of::<PyPublicKey>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyPublicKey {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyPublicKey {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyPublicKey {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyPublicKey as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyPublicKey {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyPublicKey {
        pub fn __new__(obj: &PyRawObject, s: &str) -> PyResult<()> {
            let key = PublicKey::from_bytes(s).map_err(PyValueError)?;
            obj.init(|_| Self { inner: key })
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyPublicKey {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::New({
                    #[allow(unused_mut)]
                    unsafe extern "C" fn __wrap(
                        _cls: *mut ::pyo3::ffi::PyTypeObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        use pyo3::typeob::PyTypeInfo;
                        const _LOCATION: &'static str = "PyPublicKey.__new__()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        match ::pyo3::typeob::PyRawObject::new(
                            _py,
                            PyPublicKey::type_object(),
                            _cls,
                        ) {
                            Ok(_obj) => {
                                let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                                let _kwargs: Option<&::pyo3::types::PyDict> =
                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                let _result: ::pyo3::PyResult<
                                    <PyResult<()> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                                > = {
                                    const _PARAMS:
                                        &'static [::pyo3::derive_utils::ParamDescription<'static>] =
                                        &[::pyo3::derive_utils::ParamDescription {
                                            name: "s",
                                            is_optional: false,
                                            kw_only: false,
                                        }];
                                    let mut _output = [None];
                                    match ::pyo3::derive_utils::parse_fn_args(
                                        Some(_LOCATION),
                                        _PARAMS,
                                        &_args,
                                        _kwargs,
                                        false,
                                        false,
                                        &mut _output,
                                    ) {
                                        Ok(_) => {
                                            let mut _iter = _output.iter();
                                            ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                                   {
                                                                                                                                                                       ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(PyPublicKey::__new__(&_obj,
                                                                                                                                                                                                                                                       arg0))
                                                                                                                                                                   })
                                        }
                                        Err(err) => Err(err),
                                    }
                                };
                                match _result {
                                    Ok(_) => ::pyo3::IntoPyPointer::into_ptr(_obj),
                                    Err(e) => {
                                        e.restore(_py);
                                        ::std::ptr::null_mut()
                                    }
                                }
                            }
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "__new__",
                        ml_meth: ::pyo3::class::PyMethodType::PyNewFunc(__wrap),
                        ml_flags: ::pyo3::ffi::METH_VARARGS | ::pyo3::ffi::METH_KEYWORDS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
    impl<'p> pyo3::basic::PyObjectStrProtocol<'p> for PyPublicKey {
        type Result = PyResult<Self::Success>;
        type Success = String;
    }
    impl<'p> pyo3::class::basic::PyObjectProtocol<'p> for PyPublicKey {
        fn __str__(&'p self) -> <PyPublicKey as pyo3::basic::PyObjectStrProtocol>::Result {
            Ok(self.inner.to_string())
        }
    }
    pub struct PySecretKey {
        pub(crate) inner: SecretKey,
    }
    impl ::std::convert::From<(SecretKey)> for PySecretKey {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (SecretKey)) -> PySecretKey {
            PySecretKey { inner: original }
        }
    }
    impl ::pyo3::typeob::PyTypeInfo for PySecretKey {
        type Type = PySecretKey;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "SecretKey";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PySecretKey>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PySecretKey>()
                - 1)
                / ::std::mem::align_of::<PySecretKey>()
                * ::std::mem::align_of::<PySecretKey>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PySecretKey {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PySecretKey {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PySecretKey {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PySecretKey as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PySecretKey {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl<'p> pyo3::basic::PyObjectStrProtocol<'p> for PySecretKey {
        type Result = PyResult<Self::Success>;
        type Success = String;
    }
    impl<'p> pyo3::class::basic::PyObjectProtocol<'p> for PySecretKey {
        fn __str__(&'p self) -> <PySecretKey as pyo3::basic::PyObjectStrProtocol>::Result {
            Ok(self.inner.to_string())
        }
    }
    pub struct PySignature {
        pub(crate) inner: Signature,
    }
    impl ::std::convert::From<(Signature)> for PySignature {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (Signature)) -> PySignature {
            PySignature { inner: original }
        }
    }
    impl ::pyo3::typeob::PyTypeInfo for PySignature {
        type Type = PySignature;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "Signature";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PySignature>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PySignature>()
                - 1)
                / ::std::mem::align_of::<PySignature>()
                * ::std::mem::align_of::<PySignature>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PySignature {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PySignature {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PySignature {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PySignature as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PySignature {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl<'p> pyo3::basic::PyObjectStrProtocol<'p> for PySignature {
        type Result = PyResult<Self::Success>;
        type Success = String;
    }
    impl<'p> pyo3::class::basic::PyObjectProtocol<'p> for PySignature {
        fn __str__(&'p self) -> <PySignature as pyo3::basic::PyObjectStrProtocol>::Result {
            Ok(self.inner.to_string())
        }
    }
}
mod errors {
    #![allow(non_snake_case)]
    use pyo3::{types::exceptions, PyErr};
    use std::fmt::Display;
    pub(crate) fn PyException(err: impl Display) -> PyErr {
        exceptions::Exception::py_err(err.to_string())
    }
    pub(crate) fn PyValueError(err: impl Display) -> PyErr {
        exceptions::ValueError::py_err(err.to_string())
    }
}
mod file_info {
    use derive_more::From;
    use hedera::FileInfo;
    use pyo3::prelude::*;
    pub struct PyFileInfo {
        inner: FileInfo,
    }
    impl ::std::convert::From<(FileInfo)> for PyFileInfo {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (FileInfo)) -> PyFileInfo {
            PyFileInfo { inner: original }
        }
    }
    impl ::pyo3::typeob::PyTypeInfo for PyFileInfo {
        type Type = PyFileInfo;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "FileInfo";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize = { Self::OFFSET as usize + ::std::mem::size_of::<PyFileInfo>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyFileInfo>()
                - 1)
                / ::std::mem::align_of::<PyFileInfo>()
                * ::std::mem::align_of::<PyFileInfo>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyFileInfo {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyFileInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyFileInfo {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyFileInfo as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyFileInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyFileInfo {
        pub fn file_id(&mut self) -> PyResult<String> {
            Ok(self.inner.file_id.to_string())
        }
        pub fn size(&mut self) -> PyResult<i64> {
            Ok(self.inner.size)
        }
        pub fn deleted(&mut self) -> PyResult<bool> {
            Ok(self.inner.deleted)
        }
        pub fn keys(&mut self) -> PyResult<Vec<String>> {
            Ok(self.inner.keys.iter().map(|key| key.to_string()).collect())
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyFileInfo {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyFileInfo.file_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyFileInfo>(_slf);
                        match _slf.file_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "file_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyFileInfo.size()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyFileInfo>(_slf);
                        match _slf.size() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "size",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyFileInfo.deleted()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyFileInfo>(_slf);
                        match _slf.deleted() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "deleted",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyFileInfo.keys()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyFileInfo>(_slf);
                        match _slf.keys() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "keys",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod id {
    use super::errors::PyValueError;
    use hedera::{AccountId, ContractId, FileId};
    use pyo3::prelude::*;
    use std::str::FromStr;
    macro_rules! impl_id(( $ pyname : ident , $ rname : ident ) => {
                         # [ pymethods ] impl $ pyname {
                         # [ new ] fn __new__ (
                         obj : & PyRawObject , s : & str ) -> PyResult < (  )
                         > {
                         let id = $ rname :: from_str ( s ) . map_err (
                         PyValueError ) ? ; obj . init (
                         | _ | Self { inner : id } ) } } impl From < $ rname >
                         for $ pyname {
                         fn from ( id : $ rname ) -> Self {
                         Self { inner : id } } } def_str ! ( $ pyname ) ; }
                         ;);
    pub struct PyAccountId {
        pub(crate) inner: AccountId,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyAccountId {
        type Type = PyAccountId;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "AccountId";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyAccountId>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyAccountId>()
                - 1)
                / ::std::mem::align_of::<PyAccountId>()
                * ::std::mem::align_of::<PyAccountId>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyAccountId {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyAccountId {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyAccountId {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyAccountId as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyAccountId {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyAccountId {
        fn __new__(obj: &PyRawObject, s: &str) -> PyResult<()> {
            let id = AccountId::from_str(s).map_err(PyValueError)?;
            obj.init(|_| Self { inner: id })
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyAccountId {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::New({
                    #[allow(unused_mut)]
                    unsafe extern "C" fn __wrap(
                        _cls: *mut ::pyo3::ffi::PyTypeObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        use pyo3::typeob::PyTypeInfo;
                        const _LOCATION: &'static str = "PyAccountId.__new__()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        match ::pyo3::typeob::PyRawObject::new(
                            _py,
                            PyAccountId::type_object(),
                            _cls,
                        ) {
                            Ok(_obj) => {
                                let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                                let _kwargs: Option<&::pyo3::types::PyDict> =
                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                let _result: ::pyo3::PyResult<
                                    <PyResult<()> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                                > = {
                                    const _PARAMS:
                                        &'static [::pyo3::derive_utils::ParamDescription<'static>] =
                                        &[::pyo3::derive_utils::ParamDescription {
                                            name: "s",
                                            is_optional: false,
                                            kw_only: false,
                                        }];
                                    let mut _output = [None];
                                    match ::pyo3::derive_utils::parse_fn_args(
                                        Some(_LOCATION),
                                        _PARAMS,
                                        &_args,
                                        _kwargs,
                                        false,
                                        false,
                                        &mut _output,
                                    ) {
                                        Ok(_) => {
                                            let mut _iter = _output.iter();
                                            ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                                   {
                                                                                                                                                                       ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(PyAccountId::__new__(&_obj,
                                                                                                                                                                                                                                                       arg0))
                                                                                                                                                                   })
                                        }
                                        Err(err) => Err(err),
                                    }
                                };
                                match _result {
                                    Ok(_) => ::pyo3::IntoPyPointer::into_ptr(_obj),
                                    Err(e) => {
                                        e.restore(_py);
                                        ::std::ptr::null_mut()
                                    }
                                }
                            }
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "__new__",
                        ml_meth: ::pyo3::class::PyMethodType::PyNewFunc(__wrap),
                        ml_flags: ::pyo3::ffi::METH_VARARGS | ::pyo3::ffi::METH_KEYWORDS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
    impl From<AccountId> for PyAccountId {
        fn from(id: AccountId) -> Self {
            Self { inner: id }
        }
    }
    impl<'p> pyo3::basic::PyObjectStrProtocol<'p> for PyAccountId {
        type Result = PyResult<Self::Success>;
        type Success = String;
    }
    impl<'p> pyo3::class::basic::PyObjectProtocol<'p> for PyAccountId {
        fn __str__(&'p self) -> <PyAccountId as pyo3::basic::PyObjectStrProtocol>::Result {
            Ok(self.inner.to_string())
        }
    }
    pub struct PyFileId {
        pub(crate) inner: FileId,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyFileId {
        type Type = PyFileId;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "FileId";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize = { Self::OFFSET as usize + ::std::mem::size_of::<PyFileId>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyFileId>()
                - 1)
                / ::std::mem::align_of::<PyFileId>()
                * ::std::mem::align_of::<PyFileId>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyFileId {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyFileId {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyFileId {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyFileId as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyFileId {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyFileId {
        fn __new__(obj: &PyRawObject, s: &str) -> PyResult<()> {
            let id = FileId::from_str(s).map_err(PyValueError)?;
            obj.init(|_| Self { inner: id })
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyFileId {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::New({
                    #[allow(unused_mut)]
                    unsafe extern "C" fn __wrap(
                        _cls: *mut ::pyo3::ffi::PyTypeObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        use pyo3::typeob::PyTypeInfo;
                        const _LOCATION: &'static str = "PyFileId.__new__()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        match ::pyo3::typeob::PyRawObject::new(_py, PyFileId::type_object(), _cls) {
                            Ok(_obj) => {
                                let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                                let _kwargs: Option<&::pyo3::types::PyDict> =
                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                let _result: ::pyo3::PyResult<
                                    <PyResult<()> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                                > = {
                                    const _PARAMS:
                                        &'static [::pyo3::derive_utils::ParamDescription<'static>] =
                                        &[::pyo3::derive_utils::ParamDescription {
                                            name: "s",
                                            is_optional: false,
                                            kw_only: false,
                                        }];
                                    let mut _output = [None];
                                    match ::pyo3::derive_utils::parse_fn_args(
                                        Some(_LOCATION),
                                        _PARAMS,
                                        &_args,
                                        _kwargs,
                                        false,
                                        false,
                                        &mut _output,
                                    ) {
                                        Ok(_) => {
                                            let mut _iter = _output.iter();
                                            ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                                   {
                                                                                                                                                                       ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(PyFileId::__new__(&_obj,
                                                                                                                                                                                                                                                    arg0))
                                                                                                                                                                   })
                                        }
                                        Err(err) => Err(err),
                                    }
                                };
                                match _result {
                                    Ok(_) => ::pyo3::IntoPyPointer::into_ptr(_obj),
                                    Err(e) => {
                                        e.restore(_py);
                                        ::std::ptr::null_mut()
                                    }
                                }
                            }
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "__new__",
                        ml_meth: ::pyo3::class::PyMethodType::PyNewFunc(__wrap),
                        ml_flags: ::pyo3::ffi::METH_VARARGS | ::pyo3::ffi::METH_KEYWORDS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
    impl From<FileId> for PyFileId {
        fn from(id: FileId) -> Self {
            Self { inner: id }
        }
    }
    impl<'p> pyo3::basic::PyObjectStrProtocol<'p> for PyFileId {
        type Result = PyResult<Self::Success>;
        type Success = String;
    }
    impl<'p> pyo3::class::basic::PyObjectProtocol<'p> for PyFileId {
        fn __str__(&'p self) -> <PyFileId as pyo3::basic::PyObjectStrProtocol>::Result {
            Ok(self.inner.to_string())
        }
    }
    pub struct PyContractId {
        pub(crate) inner: ContractId,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyContractId {
        type Type = PyContractId;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "ContractId";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyContractId>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyContractId>()
                - 1)
                / ::std::mem::align_of::<PyContractId>()
                * ::std::mem::align_of::<PyContractId>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyContractId {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyContractId {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyContractId {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyContractId as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyContractId {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyContractId {
        fn __new__(obj: &PyRawObject, s: &str) -> PyResult<()> {
            let id = ContractId::from_str(s).map_err(PyValueError)?;
            obj.init(|_| Self { inner: id })
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyContractId {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::New({
                    #[allow(unused_mut)]
                    unsafe extern "C" fn __wrap(
                        _cls: *mut ::pyo3::ffi::PyTypeObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        use pyo3::typeob::PyTypeInfo;
                        const _LOCATION: &'static str = "PyContractId.__new__()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        match ::pyo3::typeob::PyRawObject::new(
                            _py,
                            PyContractId::type_object(),
                            _cls,
                        ) {
                            Ok(_obj) => {
                                let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                                let _kwargs: Option<&::pyo3::types::PyDict> =
                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                let _result: ::pyo3::PyResult<
                                    <PyResult<()> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                                > = {
                                    const _PARAMS:
                                        &'static [::pyo3::derive_utils::ParamDescription<'static>] =
                                        &[::pyo3::derive_utils::ParamDescription {
                                            name: "s",
                                            is_optional: false,
                                            kw_only: false,
                                        }];
                                    let mut _output = [None];
                                    match ::pyo3::derive_utils::parse_fn_args(
                                        Some(_LOCATION),
                                        _PARAMS,
                                        &_args,
                                        _kwargs,
                                        false,
                                        false,
                                        &mut _output,
                                    ) {
                                        Ok(_) => {
                                            let mut _iter = _output.iter();
                                            ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                                   {
                                                                                                                                                                       ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(PyContractId::__new__(&_obj,
                                                                                                                                                                                                                                                        arg0))
                                                                                                                                                                   })
                                        }
                                        Err(err) => Err(err),
                                    }
                                };
                                match _result {
                                    Ok(_) => ::pyo3::IntoPyPointer::into_ptr(_obj),
                                    Err(e) => {
                                        e.restore(_py);
                                        ::std::ptr::null_mut()
                                    }
                                }
                            }
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "__new__",
                        ml_meth: ::pyo3::class::PyMethodType::PyNewFunc(__wrap),
                        ml_flags: ::pyo3::ffi::METH_VARARGS | ::pyo3::ffi::METH_KEYWORDS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
    impl From<ContractId> for PyContractId {
        fn from(id: ContractId) -> Self {
            Self { inner: id }
        }
    }
    impl<'p> pyo3::basic::PyObjectStrProtocol<'p> for PyContractId {
        type Result = PyResult<Self::Success>;
        type Success = String;
    }
    impl<'p> pyo3::class::basic::PyObjectProtocol<'p> for PyContractId {
        fn __str__(&'p self) -> <PyContractId as pyo3::basic::PyObjectStrProtocol>::Result {
            Ok(self.inner.to_string())
        }
    }
}
mod query_contract_get_bytecode {
    use hedera::{query::QueryContractGetBytecode, ContractId};
    #[allow(unused)]
    enum ProcMacroHack {
        Input = ("m [ \"py\" ] = Py QueryContractGetBytecode ;", 0).1,
    }
    macro_rules! m((
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) (
                   $ ( $ first : tt ) * ) $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_paren $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) [
                   $ ( $ first : tt ) * ] $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_bracket $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) {
                   $ ( $ first : tt ) * } $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_brace $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_paren $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * ( $ ( $ close ) * ) ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_bracket $ ( $ rest : tt )
                   * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * [ $ ( $ close ) * ] ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_brace $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * { $ ( $ close ) * } ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ v0 : tt ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) "py" $ (
                   $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ v0 ) ( ( $ ( $ top ) * $ v0 ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) $ first : tt
                   $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * $ first ) $ ( $ stack ) * ) $ ( $ rest ) *
                   } } ; ( @ ( $ ( $ v : tt ) * ) ( ( $ ( $ top : tt ) + ) ) )
                   => { $ ( $ top ) + } ; ( $ ( $ tt : tt ) * ) => {
                   m ! {
                   @ ( PyQueryContractGetBytecode ) ( (  ) ) $ ( $ tt ) * }
                   });
    pub struct PyQueryContractGetBytecode {
        inner: hedera::query::Query<QueryContractGetBytecode>,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyQueryContractGetBytecode {
        type Type = PyQueryContractGetBytecode;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "QueryContractGetBytecode";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyQueryContractGetBytecode>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyQueryContractGetBytecode>()
                - 1)
                / ::std::mem::align_of::<PyQueryContractGetBytecode>()
                * ::std::mem::align_of::<PyQueryContractGetBytecode>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyQueryContractGetBytecode {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyQueryContractGetBytecode {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyQueryContractGetBytecode {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyQueryContractGetBytecode as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyQueryContractGetBytecode {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyQueryContractGetBytecode {
        pub fn new(client: &hedera::Client, _1: ContractId) -> Self {
            Self {
                inner: QueryContractGetBytecode::new(client, _1),
            }
        }
    }
    impl PyQueryContractGetBytecode {
        pub fn get(&mut self) -> pyo3::PyResult<Vec<u8>> {
            self.inner
                .get()
                .map(Into::into)
                .map_err(crate::errors::PyException)
        }
        pub fn cost(&mut self) -> pyo3::PyResult<u64> {
            self.inner.cost().map_err(crate::errors::PyException)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyQueryContractGetBytecode {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryContractGetBytecode.get()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryContractGetBytecode>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<Vec<u8>> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.get())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "get",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryContractGetBytecode.cost()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryContractGetBytecode>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<u64> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.cost())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "cost",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod query_contract_get_info {
    use crate::PyContractInfo;
    use hedera::{query::QueryContractGetInfo, ContractId};
    #[allow(unused)]
    enum ProcMacroHack {
        Input = ("m [ \"py\" ] = Py QueryContractGetInfo ;", 0).1,
    }
    macro_rules! m((
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) (
                   $ ( $ first : tt ) * ) $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_paren $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) [
                   $ ( $ first : tt ) * ] $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_bracket $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) {
                   $ ( $ first : tt ) * } $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_brace $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_paren $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * ( $ ( $ close ) * ) ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_bracket $ ( $ rest : tt )
                   * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * [ $ ( $ close ) * ] ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_brace $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * { $ ( $ close ) * } ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ v0 : tt ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) "py" $ (
                   $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ v0 ) ( ( $ ( $ top ) * $ v0 ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) $ first : tt
                   $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * $ first ) $ ( $ stack ) * ) $ ( $ rest ) *
                   } } ; ( @ ( $ ( $ v : tt ) * ) ( ( $ ( $ top : tt ) + ) ) )
                   => { $ ( $ top ) + } ; ( $ ( $ tt : tt ) * ) => {
                   m ! { @ ( PyQueryContractGetInfo ) ( (  ) ) $ ( $ tt ) * }
                   });
    pub struct PyQueryContractGetInfo {
        inner: hedera::query::Query<QueryContractGetInfo>,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyQueryContractGetInfo {
        type Type = PyQueryContractGetInfo;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "QueryContractGetInfo";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyQueryContractGetInfo>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyQueryContractGetInfo>()
                - 1)
                / ::std::mem::align_of::<PyQueryContractGetInfo>()
                * ::std::mem::align_of::<PyQueryContractGetInfo>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyQueryContractGetInfo {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyQueryContractGetInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyQueryContractGetInfo {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyQueryContractGetInfo as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyQueryContractGetInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyQueryContractGetInfo {
        pub fn new(client: &hedera::Client, _1: ContractId) -> Self {
            Self {
                inner: QueryContractGetInfo::new(client, _1),
            }
        }
    }
    impl PyQueryContractGetInfo {
        pub fn get(&mut self) -> pyo3::PyResult<PyContractInfo> {
            self.inner
                .get()
                .map(Into::into)
                .map_err(crate::errors::PyException)
        }
        pub fn cost(&mut self) -> pyo3::PyResult<u64> {
            self.inner.cost().map_err(crate::errors::PyException)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyQueryContractGetInfo {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryContractGetInfo.get()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryContractGetInfo>(_slf);
                        let _result:
                                                                         ::pyo3::PyResult<<pyo3::PyResult<PyContractInfo>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.get())
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "get",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryContractGetInfo.cost()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryContractGetInfo>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<u64> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.cost())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "cost",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod query_crypto_get_account_balance {
    use hedera::{query::QueryCryptoGetAccountBalance, AccountId};
    #[allow(unused)]
    enum ProcMacroHack {
        Input = ("m [ \"py\" ] = Py QueryCryptoGetAccountBalance ;", 0).1,
    }
    macro_rules! m((
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) (
                   $ ( $ first : tt ) * ) $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_paren $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) [
                   $ ( $ first : tt ) * ] $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_bracket $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) {
                   $ ( $ first : tt ) * } $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_brace $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_paren $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * ( $ ( $ close ) * ) ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_bracket $ ( $ rest : tt )
                   * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * [ $ ( $ close ) * ] ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_brace $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * { $ ( $ close ) * } ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ v0 : tt ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) "py" $ (
                   $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ v0 ) ( ( $ ( $ top ) * $ v0 ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) $ first : tt
                   $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * $ first ) $ ( $ stack ) * ) $ ( $ rest ) *
                   } } ; ( @ ( $ ( $ v : tt ) * ) ( ( $ ( $ top : tt ) + ) ) )
                   => { $ ( $ top ) + } ; ( $ ( $ tt : tt ) * ) => {
                   m ! {
                   @ ( PyQueryCryptoGetAccountBalance ) ( (  ) ) $ ( $ tt ) *
                   } });
    pub struct PyQueryCryptoGetAccountBalance {
        inner: hedera::query::Query<QueryCryptoGetAccountBalance>,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyQueryCryptoGetAccountBalance {
        type Type = PyQueryCryptoGetAccountBalance;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "QueryCryptoGetAccountBalance";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize = {
            Self::OFFSET as usize + ::std::mem::size_of::<PyQueryCryptoGetAccountBalance>() + 0 + 0
        };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyQueryCryptoGetAccountBalance>()
                - 1)
                / ::std::mem::align_of::<PyQueryCryptoGetAccountBalance>()
                * ::std::mem::align_of::<PyQueryCryptoGetAccountBalance>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyQueryCryptoGetAccountBalance {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyQueryCryptoGetAccountBalance {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyQueryCryptoGetAccountBalance {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyQueryCryptoGetAccountBalance as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyQueryCryptoGetAccountBalance {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyQueryCryptoGetAccountBalance {
        pub fn new(client: &hedera::Client, _1: AccountId) -> Self {
            Self {
                inner: QueryCryptoGetAccountBalance::new(client, _1),
            }
        }
    }
    impl PyQueryCryptoGetAccountBalance {
        pub fn get(&mut self) -> pyo3::PyResult<u64> {
            self.inner
                .get()
                .map(Into::into)
                .map_err(crate::errors::PyException)
        }
        pub fn cost(&mut self) -> pyo3::PyResult<u64> {
            self.inner.cost().map_err(crate::errors::PyException)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyQueryCryptoGetAccountBalance {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryCryptoGetAccountBalance.get()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf =
                            _py.mut_from_borrowed_ptr::<PyQueryCryptoGetAccountBalance>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<u64> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.get())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "get",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryCryptoGetAccountBalance.cost()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf =
                            _py.mut_from_borrowed_ptr::<PyQueryCryptoGetAccountBalance>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<u64> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.cost())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "cost",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod query_crypto_get_info {
    use crate::PyAccountInfo;
    use hedera::{query::QueryCryptoGetInfo, AccountId};
    #[allow(unused)]
    enum ProcMacroHack {
        Input = ("m [ \"py\" ] = Py QueryCryptoGetInfo ;", 0).1,
    }
    macro_rules! m((
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) (
                   $ ( $ first : tt ) * ) $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_paren $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) [
                   $ ( $ first : tt ) * ] $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_bracket $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) {
                   $ ( $ first : tt ) * } $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_brace $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_paren $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * ( $ ( $ close ) * ) ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_bracket $ ( $ rest : tt )
                   * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * [ $ ( $ close ) * ] ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_brace $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * { $ ( $ close ) * } ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ v0 : tt ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) "py" $ (
                   $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ v0 ) ( ( $ ( $ top ) * $ v0 ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) $ first : tt
                   $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * $ first ) $ ( $ stack ) * ) $ ( $ rest ) *
                   } } ; ( @ ( $ ( $ v : tt ) * ) ( ( $ ( $ top : tt ) + ) ) )
                   => { $ ( $ top ) + } ; ( $ ( $ tt : tt ) * ) => {
                   m ! { @ ( PyQueryCryptoGetInfo ) ( (  ) ) $ ( $ tt ) * }
                   });
    pub struct PyQueryCryptoGetInfo {
        inner: hedera::query::Query<QueryCryptoGetInfo>,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyQueryCryptoGetInfo {
        type Type = PyQueryCryptoGetInfo;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "QueryCryptoGetInfo";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyQueryCryptoGetInfo>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyQueryCryptoGetInfo>()
                - 1)
                / ::std::mem::align_of::<PyQueryCryptoGetInfo>()
                * ::std::mem::align_of::<PyQueryCryptoGetInfo>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyQueryCryptoGetInfo {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyQueryCryptoGetInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyQueryCryptoGetInfo {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyQueryCryptoGetInfo as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyQueryCryptoGetInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyQueryCryptoGetInfo {
        pub fn new(client: &hedera::Client, _1: AccountId) -> Self {
            Self {
                inner: QueryCryptoGetInfo::new(client, _1),
            }
        }
    }
    impl PyQueryCryptoGetInfo {
        pub fn get(&mut self) -> pyo3::PyResult<PyAccountInfo> {
            self.inner
                .get()
                .map(Into::into)
                .map_err(crate::errors::PyException)
        }
        pub fn cost(&mut self) -> pyo3::PyResult<u64> {
            self.inner.cost().map_err(crate::errors::PyException)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyQueryCryptoGetInfo {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryCryptoGetInfo.get()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryCryptoGetInfo>(_slf);
                        let _result:
                                                                         ::pyo3::PyResult<<pyo3::PyResult<PyAccountInfo>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.get())
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "get",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryCryptoGetInfo.cost()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryCryptoGetInfo>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<u64> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.cost())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "cost",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod query_file_get_contents {
    use hedera::{query::QueryFileGetContents, FileId};
    #[allow(unused)]
    enum ProcMacroHack {
        Input = ("m [ \"py\" ] = Py QueryFileGetContents ;", 0).1,
    }
    macro_rules! m((
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) (
                   $ ( $ first : tt ) * ) $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_paren $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) [
                   $ ( $ first : tt ) * ] $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_bracket $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) {
                   $ ( $ first : tt ) * } $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_brace $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_paren $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * ( $ ( $ close ) * ) ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_bracket $ ( $ rest : tt )
                   * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * [ $ ( $ close ) * ] ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_brace $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * { $ ( $ close ) * } ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ v0 : tt ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) "py" $ (
                   $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ v0 ) ( ( $ ( $ top ) * $ v0 ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) $ first : tt
                   $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * $ first ) $ ( $ stack ) * ) $ ( $ rest ) *
                   } } ; ( @ ( $ ( $ v : tt ) * ) ( ( $ ( $ top : tt ) + ) ) )
                   => { $ ( $ top ) + } ; ( $ ( $ tt : tt ) * ) => {
                   m ! { @ ( PyQueryFileGetContents ) ( (  ) ) $ ( $ tt ) * }
                   });
    pub struct PyQueryFileGetContents {
        inner: hedera::query::Query<QueryFileGetContents>,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyQueryFileGetContents {
        type Type = PyQueryFileGetContents;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "QueryFileGetContents";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyQueryFileGetContents>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyQueryFileGetContents>()
                - 1)
                / ::std::mem::align_of::<PyQueryFileGetContents>()
                * ::std::mem::align_of::<PyQueryFileGetContents>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyQueryFileGetContents {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyQueryFileGetContents {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyQueryFileGetContents {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyQueryFileGetContents as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyQueryFileGetContents {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyQueryFileGetContents {
        pub fn new(client: &hedera::Client, _1: FileId) -> Self {
            Self {
                inner: QueryFileGetContents::new(client, _1),
            }
        }
    }
    impl PyQueryFileGetContents {
        pub fn get(&mut self) -> pyo3::PyResult<Vec<u8>> {
            self.inner
                .get()
                .map(Into::into)
                .map_err(crate::errors::PyException)
        }
        pub fn cost(&mut self) -> pyo3::PyResult<u64> {
            self.inner.cost().map_err(crate::errors::PyException)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyQueryFileGetContents {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryFileGetContents.get()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryFileGetContents>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<Vec<u8>> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.get())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "get",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryFileGetContents.cost()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryFileGetContents>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<u64> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.cost())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "cost",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod query_file_get_info {
    use crate::PyFileInfo;
    use hedera::{query::QueryFileGetInfo, FileId};
    #[allow(unused)]
    enum ProcMacroHack {
        Input = ("m [ \"py\" ] = Py QueryFileGetInfo ;", 0).1,
    }
    macro_rules! m((
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) (
                   $ ( $ first : tt ) * ) $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_paren $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) [
                   $ ( $ first : tt ) * ] $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_bracket $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) {
                   $ ( $ first : tt ) * } $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_brace $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_paren $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * ( $ ( $ close ) * ) ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_bracket $ ( $ rest : tt )
                   * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * [ $ ( $ close ) * ] ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_brace $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * { $ ( $ close ) * } ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ v0 : tt ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) "py" $ (
                   $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ v0 ) ( ( $ ( $ top ) * $ v0 ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) $ first : tt
                   $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * $ first ) $ ( $ stack ) * ) $ ( $ rest ) *
                   } } ; ( @ ( $ ( $ v : tt ) * ) ( ( $ ( $ top : tt ) + ) ) )
                   => { $ ( $ top ) + } ; ( $ ( $ tt : tt ) * ) => {
                   m ! { @ ( PyQueryFileGetInfo ) ( (  ) ) $ ( $ tt ) * } });
    pub struct PyQueryFileGetInfo {
        inner: hedera::query::Query<QueryFileGetInfo>,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyQueryFileGetInfo {
        type Type = PyQueryFileGetInfo;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "QueryFileGetInfo";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyQueryFileGetInfo>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyQueryFileGetInfo>()
                - 1)
                / ::std::mem::align_of::<PyQueryFileGetInfo>()
                * ::std::mem::align_of::<PyQueryFileGetInfo>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyQueryFileGetInfo {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyQueryFileGetInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyQueryFileGetInfo {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyQueryFileGetInfo as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyQueryFileGetInfo {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyQueryFileGetInfo {
        pub fn new(client: &hedera::Client, _1: FileId) -> Self {
            Self {
                inner: QueryFileGetInfo::new(client, _1),
            }
        }
    }
    impl PyQueryFileGetInfo {
        pub fn get(&mut self) -> pyo3::PyResult<PyFileInfo> {
            self.inner
                .get()
                .map(Into::into)
                .map_err(crate::errors::PyException)
        }
        pub fn cost(&mut self) -> pyo3::PyResult<u64> {
            self.inner.cost().map_err(crate::errors::PyException)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyQueryFileGetInfo {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryFileGetInfo.get()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryFileGetInfo>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<PyFileInfo> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.get())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "get",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryFileGetInfo.cost()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryFileGetInfo>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<u64> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.cost())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "cost",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod query_get_transaction_receipt {
    use crate::PyTransactionReceipt;
    use hedera::{query::QueryGetTransactionReceipt, TransactionId};
    #[allow(unused)]
    enum ProcMacroHack {
        Input = ("m [ \"py\" ] = Py QueryGetTransactionReceipt ;", 0).1,
    }
    macro_rules! m((
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) (
                   $ ( $ first : tt ) * ) $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_paren $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) [
                   $ ( $ first : tt ) * ] $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_bracket $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) ( $ ( $ stack : tt ) * ) {
                   $ ( $ first : tt ) * } $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) ( (  ) $ ( $ stack ) * ) $ ( $ first ) *
                   __mashup_close_brace $ ( $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_paren $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * ( $ ( $ close ) * ) ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_bracket $ ( $ rest : tt )
                   * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * [ $ ( $ close ) * ] ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ close : tt ) * ) ( $ ( $ top : tt ) * ) $ (
                   $ stack : tt ) * ) __mashup_close_brace $ ( $ rest : tt ) *
                   ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * { $ ( $ close ) * } ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ v0 : tt ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) "py" $ (
                   $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ v0 ) ( ( $ ( $ top ) * $ v0 ) $ ( $ stack ) * ) $ (
                   $ rest ) * } } ; (
                   @ ( $ ( $ v : tt ) * ) (
                   ( $ ( $ top : tt ) * ) $ ( $ stack : tt ) * ) $ first : tt
                   $ ( $ rest : tt ) * ) => {
                   m ! {
                   @ ( $ ( $ v ) * ) (
                   ( $ ( $ top ) * $ first ) $ ( $ stack ) * ) $ ( $ rest ) *
                   } } ; ( @ ( $ ( $ v : tt ) * ) ( ( $ ( $ top : tt ) + ) ) )
                   => { $ ( $ top ) + } ; ( $ ( $ tt : tt ) * ) => {
                   m ! {
                   @ ( PyQueryGetTransactionReceipt ) ( (  ) ) $ ( $ tt ) * }
                   });
    pub struct PyQueryGetTransactionReceipt {
        inner: hedera::query::Query<QueryGetTransactionReceipt>,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyQueryGetTransactionReceipt {
        type Type = PyQueryGetTransactionReceipt;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "QueryGetTransactionReceipt";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize = {
            Self::OFFSET as usize + ::std::mem::size_of::<PyQueryGetTransactionReceipt>() + 0 + 0
        };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyQueryGetTransactionReceipt>()
                - 1)
                / ::std::mem::align_of::<PyQueryGetTransactionReceipt>()
                * ::std::mem::align_of::<PyQueryGetTransactionReceipt>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyQueryGetTransactionReceipt {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyQueryGetTransactionReceipt {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyQueryGetTransactionReceipt {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyQueryGetTransactionReceipt as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyQueryGetTransactionReceipt {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyQueryGetTransactionReceipt {
        pub fn new(client: &hedera::Client, _1: TransactionId) -> Self {
            Self {
                inner: QueryGetTransactionReceipt::new(client, _1),
            }
        }
    }
    impl PyQueryGetTransactionReceipt {
        pub fn get(&mut self) -> pyo3::PyResult<PyTransactionReceipt> {
            self.inner
                .get()
                .map(Into::into)
                .map_err(crate::errors::PyException)
        }
        pub fn cost(&mut self) -> pyo3::PyResult<u64> {
            self.inner.cost().map_err(crate::errors::PyException)
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyQueryGetTransactionReceipt {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryGetTransactionReceipt.get()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryGetTransactionReceipt>(_slf);
                        let _result:
                                                                         ::pyo3::PyResult<<pyo3::PyResult<PyTransactionReceipt>
                                                                                          as
                                                                                          ::pyo3::ReturnTypeIntoPyResult>::Inner> =
                                                                     {
                                                                         ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.get())
                                                                     };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "get",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Method({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyQueryGetTransactionReceipt.cost()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyQueryGetTransactionReceipt>(_slf);
                        let _result: ::pyo3::PyResult<
                            <pyo3::PyResult<u64> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                        > = {
                            ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(_slf.cost())
                        };
                        ::pyo3::callback::cb_convert(
                            ::pyo3::callback::PyObjectCallbackConverter,
                            _py,
                            _result,
                        )
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "cost",
                        ml_meth: ::pyo3::class::PyMethodType::PyNoArgsFunction(__wrap),
                        ml_flags: ::pyo3::ffi::METH_NOARGS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
}
mod timestamp {
    use chrono::{prelude::*, DateTime, Utc};
    use derive_more::From;
    use pyo3::{
        prelude::*,
        types::{self, PyObjectRef},
    };
    pub struct PyDateTime(pub DateTime<Utc>);
    impl ::std::convert::From<(DateTime<Utc>)> for PyDateTime {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (DateTime<Utc>)) -> PyDateTime {
            PyDateTime(original)
        }
    }
    impl IntoPyObject for PyDateTime {
        fn into_object(self, py: Python) -> PyObject {
            let year = self.0.year();
            let month = self.0.month() as u8;
            let day = self.0.day() as u8;
            let hour = self.0.hour() as u8;
            let minute = self.0.minute() as u8;
            let second = self.0.second() as u8;
            let microsecond = (self.0.nanosecond() % 1000000000) / 1000;
            match types::PyDateTime::new(py, year, 41, day, hour, minute, second, microsecond, None)
            {
                Ok(a) => a.into_object(py),
                Err(e) => {
                    let no = None::<()>.into_object(py);
                    e.restore(py);
                    no
                }
            }
        }
    }
}
mod transaction_id {
    use crate::errors::PyValueError;
    use derive_more::From;
    use hedera::TransactionId;
    use pyo3::prelude::*;
    use std::str::FromStr;
    pub struct PyTransactionId {
        pub(crate) inner: TransactionId,
    }
    impl ::std::convert::From<(TransactionId)> for PyTransactionId {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (TransactionId)) -> PyTransactionId {
            PyTransactionId { inner: original }
        }
    }
    impl ::pyo3::typeob::PyTypeInfo for PyTransactionId {
        type Type = PyTransactionId;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "TransactionId";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyTransactionId>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyTransactionId>()
                - 1)
                / ::std::mem::align_of::<PyTransactionId>()
                * ::std::mem::align_of::<PyTransactionId>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyTransactionId {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyTransactionId {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyTransactionId {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyTransactionId as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyTransactionId {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyTransactionId {
        pub fn __new__(obj: &PyRawObject, s: &str) -> PyResult<()> {
            let id = TransactionId::from_str(s).map_err(PyValueError)?;
            obj.init(|_| Self { inner: id })
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyTransactionId {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::New({
                    #[allow(unused_mut)]
                    unsafe extern "C" fn __wrap(
                        _cls: *mut ::pyo3::ffi::PyTypeObject,
                        _args: *mut ::pyo3::ffi::PyObject,
                        _kwargs: *mut ::pyo3::ffi::PyObject,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        use pyo3::typeob::PyTypeInfo;
                        const _LOCATION: &'static str = "PyTransactionId.__new__()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        match ::pyo3::typeob::PyRawObject::new(
                            _py,
                            PyTransactionId::type_object(),
                            _cls,
                        ) {
                            Ok(_obj) => {
                                let _args = _py.from_borrowed_ptr::<::pyo3::types::PyTuple>(_args);
                                let _kwargs: Option<&::pyo3::types::PyDict> =
                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                let _result: ::pyo3::PyResult<
                                    <PyResult<()> as ::pyo3::ReturnTypeIntoPyResult>::Inner,
                                > = {
                                    const _PARAMS:
                                        &'static [::pyo3::derive_utils::ParamDescription<'static>] =
                                        &[::pyo3::derive_utils::ParamDescription {
                                            name: "s",
                                            is_optional: false,
                                            kw_only: false,
                                        }];
                                    let mut _output = [None];
                                    match ::pyo3::derive_utils::parse_fn_args(
                                        Some(_LOCATION),
                                        _PARAMS,
                                        &_args,
                                        _kwargs,
                                        false,
                                        false,
                                        &mut _output,
                                    ) {
                                        Ok(_) => {
                                            let mut _iter = _output.iter();
                                            ::pyo3::ObjectProtocol::extract(_iter.next().unwrap().unwrap()).and_then(|arg0|
                                                                                                                                                                   {
                                                                                                                                                                       ::pyo3::ReturnTypeIntoPyResult::return_type_into_py_result(PyTransactionId::__new__(&_obj,
                                                                                                                                                                                                                                                           arg0))
                                                                                                                                                                   })
                                        }
                                        Err(err) => Err(err),
                                    }
                                };
                                match _result {
                                    Ok(_) => ::pyo3::IntoPyPointer::into_ptr(_obj),
                                    Err(e) => {
                                        e.restore(_py);
                                        ::std::ptr::null_mut()
                                    }
                                }
                            }
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyMethodDef {
                        ml_name: "__new__",
                        ml_meth: ::pyo3::class::PyMethodType::PyNewFunc(__wrap),
                        ml_flags: ::pyo3::ffi::METH_VARARGS | ::pyo3::ffi::METH_KEYWORDS,
                        ml_doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
    impl<'p> pyo3::basic::PyObjectStrProtocol<'p> for PyTransactionId {
        type Result = PyResult<Self::Success>;
        type Success = String;
    }
    impl<'p> pyo3::class::basic::PyObjectProtocol<'p> for PyTransactionId {
        fn __str__(&'p self) -> <PyTransactionId as pyo3::basic::PyObjectStrProtocol>::Result {
            Ok(self.inner.to_string())
        }
    }
}
mod transaction_receipt {
    use hedera::TransactionReceipt;
    use pyo3::prelude::*;
    pub struct PyTransactionReceipt {
        pub(crate) inner: TransactionReceipt,
    }
    impl ::pyo3::typeob::PyTypeInfo for PyTransactionReceipt {
        type Type = PyTransactionReceipt;
        type BaseType = ::pyo3::types::PyObjectRef;
        const NAME: &'static str = "TransactionReceipt";
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0;
        const SIZE: usize =
            { Self::OFFSET as usize + ::std::mem::size_of::<PyTransactionReceipt>() + 0 + 0 };
        const OFFSET: isize = {
            ((<::pyo3::types::PyObjectRef as ::pyo3::typeob::PyTypeInfo>::SIZE
                + ::std::mem::align_of::<PyTransactionReceipt>()
                - 1)
                / ::std::mem::align_of::<PyTransactionReceipt>()
                * ::std::mem::align_of::<PyTransactionReceipt>()) as isize
        };
        #[inline]
        unsafe fn type_object() -> &'static mut ::pyo3::ffi::PyTypeObject {
            static mut TYPE_OBJECT: ::pyo3::ffi::PyTypeObject = ::pyo3::ffi::PyTypeObject_INIT;
            &mut TYPE_OBJECT
        }
    }
    impl ::pyo3::IntoPyObject for PyTransactionReceipt {
        fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            ::pyo3::Py::new(py, |_| self).unwrap().into_object(py)
        }
    }
    impl ::pyo3::ToPyObject for PyTransactionReceipt {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl ::pyo3::ToPyPointer for PyTransactionReceipt {
        fn as_ptr(&self) -> *mut ::pyo3::ffi::PyObject {
            unsafe {
                { self as *const _ as *mut u8 }
                    .offset(-<PyTransactionReceipt as ::pyo3::typeob::PyTypeInfo>::OFFSET)
                    as *mut ::pyo3::ffi::PyObject
            }
        }
    }
    impl<'a> ::pyo3::ToPyObject for &'a mut PyTransactionReceipt {
        fn to_object(&self, py: ::pyo3::Python) -> ::pyo3::PyObject {
            use pyo3::python::ToPyPointer;
            unsafe { ::pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr()) }
        }
    }
    impl PyTransactionReceipt {
        pub fn status(&self) -> PyResult<u8> {
            Ok(self.inner.status as u8)
        }
        pub fn account_id(&self) -> PyResult<Option<String>> {
            Ok(self.inner.account_id.as_ref().map(|id| id.to_string()))
        }
        pub fn contract_id(&self) -> PyResult<Option<String>> {
            Ok(self.inner.contract_id.as_ref().map(|id| id.to_string()))
        }
        pub fn file_id(&self) -> PyResult<Option<String>> {
            Ok(self.inner.file_id.as_ref().map(|id| id.to_string()))
        }
    }
    impl ::pyo3::class::methods::PyMethodsProtocolImpl for PyTransactionReceipt {
        fn py_methods() -> &'static [::pyo3::class::PyMethodDefType] {
            static METHODS: &'static [::pyo3::class::PyMethodDefType] = &[
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyTransactionReceipt.status()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyTransactionReceipt>(_slf);
                        match _slf.status() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "status",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyTransactionReceipt.account_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyTransactionReceipt>(_slf);
                        match _slf.account_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "account_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyTransactionReceipt.contract_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyTransactionReceipt>(_slf);
                        match _slf.contract_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "contract_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
                ::pyo3::class::PyMethodDefType::Getter({
                    unsafe extern "C" fn __wrap(
                        _slf: *mut ::pyo3::ffi::PyObject,
                        _: *mut ::std::os::raw::c_void,
                    ) -> *mut ::pyo3::ffi::PyObject {
                        const _LOCATION: &'static str = "PyTransactionReceipt.file_id()";
                        let _pool = ::pyo3::GILPool::new();
                        let _py = ::pyo3::Python::assume_gil_acquired();
                        let _slf = _py.mut_from_borrowed_ptr::<PyTransactionReceipt>(_slf);
                        match _slf.file_id() {
                            Ok(val) => ::pyo3::IntoPyPointer::into_ptr(val.into_object(_py)),
                            Err(e) => {
                                e.restore(_py);
                                ::std::ptr::null_mut()
                            }
                        }
                    }
                    ::pyo3::class::PyGetterDef {
                        name: "file_id",
                        meth: __wrap,
                        doc: "\u{0}",
                    }
                }),
            ];
            METHODS
        }
    }
    impl From<TransactionReceipt> for PyTransactionReceipt {
        fn from(receipt: TransactionReceipt) -> Self {
            Self { inner: receipt }
        }
    }
}
use self::{
    account_info::PyAccountInfo,
    claim::PyClaim,
    client::*,
    contract_info::PyContractInfo,
    crypto::{PyPublicKey, PySecretKey, PySignature},
    file_info::PyFileInfo,
    id::{PyAccountId, PyContractId, PyFileId},
    query_crypto_get_account_balance::PyQueryCryptoGetAccountBalance,
    query_file_get_info::PyQueryFileGetInfo,
    query_get_transaction_receipt::PyQueryGetTransactionReceipt,
    transaction_id::PyTransactionId,
    transaction_receipt::PyTransactionReceipt,
};
use pyo3::prelude::*;
fn hedera(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyClient>()?;
    m.add_class::<PyPartialAccountMessage>()?;
    m.add_class::<PyPartialFileMessage>()?;
    m.add_class::<PyPartialTransactionMessage>()?;
    m.add_class::<PyQueryCryptoGetAccountBalance>()?;
    m.add_class::<PyQueryGetTransactionReceipt>()?;
    m.add_class::<PyQueryFileGetInfo>()?;
    m.add_class::<PyAccountId>()?;
    m.add_class::<PyFileId>()?;
    m.add_class::<PyContractId>()?;
    m.add_class::<PyPublicKey>()?;
    m.add_class::<PySecretKey>()?;
    m.add_class::<PySignature>()?;
    m.add_class::<PyAccountInfo>()?;
    m.add_class::<PyContractInfo>()?;
    m.add_class::<PyFileInfo>()?;
    m.add_class::<PyTransactionId>()?;
    m.add_class::<PyTransactionReceipt>()?;
    Ok(())
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc = r" This autogenerated function is called by the python interpreter when importing"]
#[doc = r" the module."]
pub unsafe extern "C" fn PyInit_hedera() -> *mut ::pyo3::ffi::PyObject {
    ::pyo3::derive_utils::make_module("hedera\u{0}", "", hedera)
}
