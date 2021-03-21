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

apllodb-immutable-schema-engine applies [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
in order to safely replace transactions and buffer managers implementation (in Frameworks & Derivers, or Infrastructure layer)
without changing Immutable Schema logics and structures (in Enterprise Business Rules / Applications Business Rules, also called Domain / Application layers).

Here is the diagram describing Clean Architecture used in `apllodb-immutable-schema-engine*` sub-crates.

![apllodb-immutable-schema-engine Clean Architecture](https://user-images.githubusercontent.com/498788/85363246-5b802e80-b55b-11ea-98ca-a3d97f68a53a.png)

## Limitations

`async-std` is the only tested async runtime for this storage engine.

This engine internally uses `sqlx::Pool`, which seems not to work with tokio.
