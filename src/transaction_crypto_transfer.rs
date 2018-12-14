use crate::PyAccountId;
use hedera::transaction::TransactionCryptoTransfer;
use pyo3::PyResult;

def_transaction!(TransactionCryptoTransfer() {} {
    fn add_transfer(&mut self, id: &PyAccountId, amount: i64) -> PyResult<()> {
        self.inner.transfer(id.clone().into(), amount);
        Ok(())
    }
});
