use apllodb_server::Record;

pub(super) trait RecordCliDisplay {
    fn cli_display(self) -> String;
}

impl RecordCliDisplay for Record {
    fn cli_display(self) -> String {
        let mut s = String::new();
        for (name, value) in self.into_name_values() {
            s.push_str(&format!("{}: {}\t", name, value));
        }
        s
    }
}
