pub mod factory_macro;

/// Creates HashMap.
///
/// # Examples
///
/// ```
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
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
