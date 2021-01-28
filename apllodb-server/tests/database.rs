mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{ApllodbErrorKind, ApllodbResult};
use sql_test::{SqlTest, Step, StepRes};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_create_database() -> ApllodbResult<()> {
    let mut t = SqlTest::default();
    t.add_step(Step::new(
        "CREATE DATABASE test_create_database",
        StepRes::Ok,
    ));
    t.add_step(Step::new(
        "CREATE DATABASE test_create_database",
        StepRes::Err(ApllodbErrorKind::DuplicateDatabase),
    ));
    t.run().await;

    Ok(())
}

#[async_std::test]
async fn test_use_database() -> ApllodbResult<()> {
    let mut t = SqlTest::default();
    t.add_step(Step::new(
        "USE DATABASE test_use_database",
        StepRes::Err(ApllodbErrorKind::UndefinedObject),
    ));
    t.add_step(Step::new("CREATE DATABASE test_use_database", StepRes::Ok));
    t.add_step(Step::new("USE DATABASE test_use_database", StepRes::Ok));
    t.run().await;

    Ok(())
}
