mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::ApllodbErrorKind;
use sql_test::{SessionAB, SqlTest, SqlTestSessionAB, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_begin() {
    SqlTest::default()

        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            "BEGIN",
            StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_commit() {
    SqlTest::default()
        .add_steps(Steps::BeginTransaction)
        .add_step(Step::new("COMMIT", StepRes::Ok))
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        .add_step(Step::new(
            "COMMIT",
            StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_abort() {
    SqlTest::default()
        .add_steps(Steps::BeginTransaction)
        .add_step(Step::new("ABORT", StepRes::Ok))
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new("ABORT", StepRes::Ok))
        .add_step(Step::new(
            "ABORT",
            StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_begin_session_ab() {
    SqlTestSessionAB::default()
        .add_step(SessionAB::A, Step::new("BEGIN", StepRes::Ok))
        .add_step(SessionAB::B, Step::new("BEGIN", StepRes::Ok))
        .add_step(
            SessionAB::A,
            Step::new(
                "BEGIN",
                StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
            ),
        )
        .run()
        .await;
}

#[async_std::test]
async fn test_transaction_ddl_isolation() {
    SqlTestSessionAB::default()
        .add_steps(SessionAB::A, Steps::BeginTransaction)
        .add_steps(SessionAB::B, Steps::BeginTransaction)
        .add_step(
            SessionAB::A,
            Step::new("CREATE TABLE t (id INTEGER, PRIMARY KEY (id))", StepRes::Ok),
        )
        .add_step(
            SessionAB::A,
            Step::new("INSERT INTO t (id) VALUES (1)", StepRes::Ok),
        )
        .add_step(
            // No "Dirty read"
            SessionAB::B,
            Step::new(
                "INSERT INTO t (id) VALUES (2)",
                StepRes::Err(ApllodbErrorKind::UndefinedTable),
            ),
        )
        .add_step(SessionAB::A, Step::new("COMMIT", StepRes::Ok))
        .add_step(
            // No "Phantom read"
            SessionAB::B,
            Step::new(
                "INSERT INTO t (id) VALUES (2)",
                StepRes::Err(ApllodbErrorKind::UndefinedTable),
            ),
        )
        .add_step(SessionAB::B, Step::new("COMMIT", StepRes::Ok))
        .add_step(SessionAB::B, Step::new("BEGIN", StepRes::Ok))
        .add_step(
            SessionAB::B,
            Step::new("INSERT INTO t (id) VALUES (2)", StepRes::Ok),
        )
        .run()
        .await;

    // TODO ALTER 実装ができたら non-repeatable read のテストを追加
}

// #[async_std::test]
// async fn test_transaction_dml_isolation() {
//     SqlTestSessionAB::default()
//         .add_steps(SessionAB::A, Steps::CreateTablePeople)
//         .add_steps(SessionAB::B, Steps::BeginTransaction)
//         .add_step(
//             SessionAB::A,
//             Step::new("INSERT INTO people (id, age) VALUES (1, 18)", StepRes::Ok),
//         )
//         .add_step(
//             SessionAB::A,
//             Step::new(
//                 "SELECT id, age FROM people",
//                 StepRes::OkQuery(Box::new(|records| {
//                     assert_eq!(records.count(), 1);
//                     Ok(())
//                 })),
//             ),
//         )
//         .add_step(
//             // No "Dirty read"
//             SessionAB::B,
//             Step::new(
//                 "SELECT id, age FROM people",
//                 StepRes::OkQuery(Box::new(|records| {
//                     assert_eq!(records.count(), 0);
//                     Ok(())
//                 })),
//             ),
//         )
//         .add_step(SessionAB::A, Step::new("COMMIT", StepRes::Ok))
//         .add_step(
//             // No "Phantom read"
//             SessionAB::B,
//             Step::new(
//                 "SELECT id, age FROM people",
//                 StepRes::OkQuery(Box::new(|records| {
//                     assert_eq!(records.count(), 0);
//                     Ok(())
//                 })),
//             ),
//         )
//         .run()
//         .await;

//     // TODO UPDATE 実装ができたら non-repeatable read のテストを追加
// }
