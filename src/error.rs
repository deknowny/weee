

#[derive(Debug)]
pub struct CLIError {
    pub title: &'static str,
    pub description: &'static str,
    pub payload: Vec<(&'static str, String)>,
}

#[macro_export]
macro_rules! show_err {
    ([$title:ident] => $description:expr, $( $key:ident = $value:expr ),* $(,)?) => {
        {
            let payload = vec![
                $(
                    (stringify!($key), $value.to_string()),
                )*
            ];
            dbg!(stringify!($title));

            Err(CLIError {
                title: stringify!($title),
                description: $description,
                payload
            })
        }
    };

    ([$title:ident] => $description:expr) => {
        show_err!([$title] => $description,)
    };
}
