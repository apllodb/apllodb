mod version_repository_impl;
mod vtable_repository_impl;

pub(in crate::sqlite::transaction::sqlite_tx) use version_repository_impl::VersionRepositoryImpl;
pub(in crate::sqlite::transaction::sqlite_tx) use vtable_repository_impl::VTableRepositoryImpl;
