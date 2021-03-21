# apllodb-immutable-schema-engine

An apllodb's storage engine implementation.

## Installation

```toml
[dependencies]
apllodb-immutable-schema-engine = "0.1"
```

This crate provides:

- Immutable Schema
  - Immutable DDL
  - Immutable DML
- ACID transaction

## Architecture

apllodb-immutable-schema-engine is a storage engine of apllodb. In other words, apllodb-immutable-schema-engine has access methods (see section 4.5 of ["Architecture of a Database System"](https://dsf.berkeley.edu/papers/fntdb07-architecture.pdf)) implementations, whose interfaces are in apllodb-storage-engine-interface crate.

The most distinguished part of this engine is logics and data structures for Immutable Schema.
apllodb-immutable-schema-engine applies [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html) in order to put Immutable Schema implementation within Enterprise Business Rules / Applications Business Rules layers (also called Domain / Application layers). Then the core logics and data structures are independent of persistent data format (like B-Tree), transaction management, and buffer management.

Currently SQLite is used in Frameworks & Derivers layer (or Infrastructure layer) for persistent storage and transaction management but we have plant to replace it into our original implementation for better performance.

Here is the diagram describing Clean Architecture used in `apllodb-immutable-schema-engine-*` sub-crates.

![apllodb-immutable-schema-engine Clean Architecture](https://user-images.githubusercontent.com/498788/85363246-5b802e80-b55b-11ea-98ca-a3d97f68a53a.png)

Note that Interface Adapters layer is not used for the following reasons.

- **Controllers**: Some of access methods implementations in infrastructure layer do the controller work. They get inputs from storage engine callers, convert into use cases' input data, and call use cases. Other access methods do their work completely inside infrastructure layer without calling use cases. Access methods to manage transactions, for example, access transaction data implemented in infrastructure directly. Transaction management is usually done just by calling some methods a transaction data provides so making use cases for transaction (using generics) seems too much. In such cases, controllers are not necessary.
- **Presenters**: Similar to controllers, access methods implementations in infrastructure layer do the presenter work.
- **Gateways**: Not used in apllodb-immutable-schema-engine. `ImmutableSchemaAbstractTypes` in apllodb-immutable-schema-engine-domain does the similar work in that it has abstract associated types of repositories.

## Limitations

`async-std` is the only tested async runtime for this storage engine.

This engine internally uses `sqlx::Pool`, which seems not to work with tokio.
