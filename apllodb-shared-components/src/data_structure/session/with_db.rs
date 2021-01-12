use crate::{ApllodbError, ApllodbErrorKind, ApllodbResult, Database, DatabaseName, Transaction};

/// Session with open database.
///
/// Most SQL commands are executed via this type of session.
#[derive(Hash, Debug)]
pub struct SessionWithDb<Db: Database, Tx: Transaction> {
    db: Db,
    tx: Option<Tx>,
}

impl<Db: Database, Tx: Transaction> SessionWithDb<Db, Tx> {
    /// Construct a session with open database.
    pub fn new(db_name: DatabaseName) -> ApllodbResult<Self> {
        let db = Db::use_database(db_name)?;
        Ok(Self { db, tx: None })
    }

    /// Begins a transaction inside a database.
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has already begun in this session.
    pub fn begin(&mut self) -> ApllodbResult<()> {
        if self.tx.is_some() {
            return Err(ApllodbError::new(
                ApllodbErrorKind::InvalidTransactionState,
                format!("transaction has already begun: {:#?}", self.tx),
                None,
            ));
        }

        let tx = Tx::begin(db)?;
        self.tx.replace(tx);

        Ok(())
    }

    /// Get ref to transaction.
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    pub fn get_tx(&self) -> ApllodbResult<&Tx> {
        self.tx.as_ref().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::InvalidTransactionState,
                "transaction has not begun: {:#?}",
                None,
            )
        })
    }

    /// Get mut ref to transaction.
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    pub fn get_tx_mut(&mut self) -> ApllodbResult<&mut Tx> {
        self.tx.as_mut().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::InvalidTransactionState,
                "transaction has not begun: {:#?}",
                None,
            )
        })
    }

    /// Get ownership of transaction.
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    pub fn take_tx(&mut self) -> ApllodbResult<Tx> {
        self.tx.take().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::InvalidTransactionState,
                "transaction has not begun: {:#?}",
                None,
            )
        })
    }
}
