use glob::glob;
use std::env;

use apllodb_shared_components::ApllodbResult;

/// recursively rm all test sqlite3 files under PWD.
pub fn clean_test_sqlite3() -> ApllodbResult<()> {
    let cd = env::current_dir()?;

    log::debug!(
        "clean_test_sqlite3(): searching .sqlite3* files under current dir: {}",
        cd.display()
    );

    for entry in glob("./**/*.sqlite3*").unwrap() { // TODO test_* にする？
        if let Ok(path) = entry {
            log::debug!(
                "clean_test_sqlite3(): found {}. removing...",
                path.display()
            );
            std::fs::remove_file(&path)?;
        }
    }

    log::debug!("clean_test_sqlite3(): done",);

    Ok(())
}
