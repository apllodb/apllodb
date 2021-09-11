# apllodb

![MSRV](https://img.shields.io/badge/rustc-1.51+-lightgray.svg)
[![ci](https://github.com/apllodb/apllodb/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/apllodb/apllodb/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/apllodb/apllodb/branch/main/graph/badge.svg?token=621C0ARVUD)](https://codecov.io/gh/apllodb/apllodb)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/apllodb/apllodb/blob/main/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/apllodb/apllodb/blob/main/LICENSE-APACHE)

![apllodb logo](./doc/apllodb_logo.svg)

apllodb is a RDBMS purely written in Rust.

It has the following distinguished features:

- **Plugable storage engine:**
  - Implementing apllodb's storage engine is unambiguous. Your storage engine crate just depends on `apllodb-storage-engine-interface` crate and implements `StorageEngine` trait (and its associated types).
  - apllodb's default storage engine is **Immutable Schema** Engine (`apllodb-immutable-schema-engine`). This engine never deletes / requires to delete old records on `UPDATE`, `DELETE`, even on `ALTER TABLE` and `DROP TABLE`.

Also, we have plan to develop the following unique features:

- Ambiguous data ("about 100 years ago", for example) and query toward them.
- Algebraic data type as SQL types.

## Getting Started

Here shows how to build & run `apllodb-cli` and execute simple SQLs.

You are supposed to have installed [Cargo](https://github.com/rust-lang/cargo).

```bash
git clone git@github.com:eukarya-inc/apllodb.git

cd apllodb
cargo build

./target/debug/apllodb-cli
🚀🌙 SQL>   # Press Ctrl-D to exit
```

```sql
🚀🌙 SQL> create database my_db;
🚀🌙 SQL> use database my_db;

🚀🌙 SQL> create table t (id integer, name text, primary key (id));
  -- Oops! You need open transaction even for DDL.

🚀🌙 SQL> begin;
🚀🌙 SQL> create table t (id integer, name text, primary key (id));
🚀🌙 SQL> select id, name from t;

0 records in total

🚀🌙 SQL> insert into t (id, name) values (1, "name 1");
🚀🌙 SQL> insert into t (id, name) values (2, "name 2");
🚀🌙 SQL> select id, name from t;
t.id: 2 t.name: "name 2"
t.id: 1 t.name: "name 1"

2 records in total

🚀🌙 SQL> commit;
```

## Try Immutable Schema

Current core feature of apllodb is **Immutable Schema**.
Immutable Schema consists of Immutable **DDL** and Immutable **DML**.

With Immutable DDL, any kind of `ALTER TABLE` or `DROP TABLE` succeed without modifying existing records.
For example, if `t` has 1 or more records,

```sql
ALTER TABLE t ADD COLUMN c_new INTEGER NOT NULL;
```

would cause error in many RDMBSs because existing records cannot be NULL but this ALTER does not specify default value for `c_new`.

Immutable Schema preserves existing records in an old **version** and creates new version.

```sql
 t (version1)
| id | c_old |
|----|-------|
|  1 | "a"   |
|  2 | "b"   |

ALTER TABLE t ADD COLUMN c_new INTEGER NOT NULL;

 t (version1)    t (version2) 
| id | c_old |  | id | c_old | c_new |
|----|-------|  |----|-------|-------|
|  1 | "a"   |
|  2 | "b"   |

INSERT INTO t (id, c_old, c_new) VALUES (3, "c", 42);

 t (version1)    t (version2) 
| id | c_old |  | id | c_old | c_new |
|----|-------|  |----|-------|-------|
|  1 | "a"   |  |  3 |   "c" |    42 |
|  2 | "b"   |

INSERT INTO t (id, c_old) VALUES (4, "d");

 t (version1)    t (version2) 
| id | c_old |  | id | c_old | c_new |
|----|-------|  |----|-------|-------|
|  1 | "a"   |  |  3 |   "c" |    42 |
|  2 | "b"   |
|  4 | "d"   |
```

As the above example shows, DML like `INSERT` automatically choose appropriate version to modify.

To learn more about Immutable Schema, check [this slide](https://docs.google.com/presentation/d/1C6YsUNfMb4cioc2KWMwO2-85IpNfq558-IjxJh6LvPg/edit?usp=sharing) ([Japanese version](https://docs.google.com/presentation/d/1pV287_Q5LDbY9GWn3lK1iJdFz9rTnMsbmQ0a98YUY90/edit?usp=sharing)).

Currently, both Immutable DDL and Immutable DML are under development and some SQLs do not work as expected.
[doc/immutable-ddl-demo.sql](doc/immutable-ddl-demo.sql) is a working example of Immutable DDL. Try it by copy-and-paste to `apllodb-cli`.

## Development

This repository is a [multi-package project](https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/cargo-workspaces-for-multi-package-projects.html).

Many useful tasks for development are defined in `Makefile.toml`. Install [cargo-make](https://github.com/sagiegurari/cargo-make) to participate in apllodb's development.

```bash
# (clone repository)

cd apllodb
cargo make test

# (write your code)
cargo make build
cargo make test

# (before making pull-request)
cargo make format
cargo make lint

# (generate rustdoc)
cargo make doc
```

## Architecture

We refer to ["Architecture of a Database System"](https://dsf.berkeley.edu/papers/fntdb07-architecture.pdf) to set boundaries between each components (crates).

The following diagram, similarly illustrated to Fig. 1.1 of the paper, shows sub-crates and their rolls.
(Box with gray text are unimplemented roles)

![apllodb's Architecture (src: https://www.figma.com/file/9pBZXpEHkA8rtSH7w1Itqi/apllodb's-Architecture?node-id=1%3A2&viewport=552%2C484%2C0.7679687738418579)](./doc/apllodb-architecture.svg)

Entry points in `apllodb-server`, `apllodb-sql-processor`, and `apllodb-storage-engine-interface` are **async** functions so clients can run multiple SQLs at a time.

`apllodb-server` is the component to choose storage engine to use. `apllodb-immutable-schema-engine::ApllodbImmutableSchemaEngine` is specified at compile-time (as type parameter) for now.

Currently, apllodb has a single client; `apllodb-cli`. `apllodb-cli` runs from a shell, takes SQL text from stdin, and outputs query result records (or error messages) to stdout/stderr.
Also, `apllodb-cli` works as single-process database. `apllodb-server` currently does not run solely.

Of course we have plan to:

- Split server and client.
- Provides client library for programming languages (Rust binding may be the first one).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in apllodb by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
