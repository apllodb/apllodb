pub(crate) fn setup() {
    let _ = env_logger::builder().is_test(true).try_init();
}

/// Creates HashMap.
///
/// # Examples
///
/// ```
/// use apllodb_shared_components::hmap;
/// use std::collections::HashMap;
///
/// let h = hmap! { "k" => "v" };
///
/// let mut h2: HashMap<&str, &str> = HashMap::new();
/// h2.insert("k", "v");
///
/// assert_eq!(h, h2);
/// ```
#[macro_export]
macro_rules! hmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
