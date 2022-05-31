use std;

#[derive(Debug)]
pub struct CLIError {
    pub title: &'static str,
    pub description: &'static str,
    pub payload: std::collections::HashMap<&'static str, String>,
}

#[macro_export]
macro_rules! show_err {
    ([$title:ident] => $description:expr, $( $key:ident = $value:expr ),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut payload = std::collections::HashMap::new();
            $(
                payload.insert(stringify!($key), $value.to_string());
            )*
            Err(CLIError {
                title: stringify!($title),
                description: $description,
                payload: payload
            })
        }
    };

    ([$title:ident] => $description:expr) => {
        show_err!([$title] => $description,)
    };
}
