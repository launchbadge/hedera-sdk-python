use crate::{PyAccountId, PyClaim, PyContractId, PyFileId};
use hedera::{query::QueryGetByKey, Entity, PublicKey};
use pyo3::{IntoPyObject, PyObject, Python};

def_query!(QueryGetByKey(PublicKey) -> Vec<PyObject> => {
    |entities| {
        let py = unsafe { Python::assume_gil_acquired() };

        entities.iter().map(|entity| match entity {
            Entity::Account(id) => PyAccountId::from(*id).into_object(py),
            Entity::Contract(id) => PyContractId::from(*id).into_object(py),
            Entity::File(id) => PyFileId::from(*id).into_object(py),
            Entity::Claim(claim) => PyClaim::from(claim.clone()).into_object(py),
        }).collect()
    }
});
