use std::path::PathBuf;

use rustyline::{error::ReadlineError, Editor};

#[derive(Debug)]
pub(crate) struct ReadLine {
    editor: Editor<()>,
    prompt: String,
    history_path: PathBuf,
}

impl Default for ReadLine {
    fn default() -> Self {
        let history_path = {
            let mut p = dirs::home_dir().expect("cannot detect HOME directory to put history file");
            p.push(".apllodb_history");
            p
        };

        let mut rl = rustyline::Editor::<()>::new(); // TODO SQL completion
        let _ = rl.load_history(&history_path);

        Self {
            editor: rl,
            prompt: "🚀🌙 SQL> ".to_string(),
            history_path,
        }
    }
}

impl ReadLine {
    pub(crate) fn readline(&mut self) -> Result<String, ReadlineError> {
        self.editor.readline(&self.prompt)
    }

    pub(crate) fn add_history(&mut self, line: &str) -> Result<(), ReadlineError> {
        self.editor.add_history_entry(line);
        self.editor.save_history(&self.history_path)?;
        Ok(())
    }
}
