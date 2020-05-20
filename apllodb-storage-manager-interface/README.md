# apllodb-storage-manager-interface

apllodb's storage manager interface.

## Installation

```toml
[dependencies]
apllodb-storage-manager-interface = "0.1"
```

## Boundary of Responsibility with Storage Engine

A storage engine is an implementation of this interface crate.

This crate provides:

- Access Methods traits related to:
  - apllodb-DDL
  - apllodb-DML
  - Transaction
  - Getting catalog
- Data structures and operations for them commonly used by every storage engine.
  - Version set
  - Version
- Traits of records and record iterators.
- Catalog data structure with read-only APIs.

And a storage engine MUST provide:

- Access Methods implementation.
- Ways to materialize version sets and versions.
- Implementation of records and record iterators.

License: (TBD)
