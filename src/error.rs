

use toml;

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

pub fn render_toml_error(file: &str, content: String, error: toml::de::Error) -> String {
    let mut view = String::new();
    if let Some((line, column)) = error.line_col() {
        let mut v: Vec<char> = format!("{}", &error).chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let message = v.into_iter().collect::<String>();

        let extra_whitespaces: String = std::iter::repeat(" ")
            .take(line.to_string().len())
            .collect();
        let arrow_whitespaces: String = std::iter::repeat(" ").take(column).collect();
        view.push_str(&message);
        view.push_str(format!("\n -> {}:{}:{}", file, line + 1, column + 1).as_str());
        view.push_str(format!("\n{}   |\n", &extra_whitespaces).as_str());
        view.push_str(format!(" {}  | ", line + 1).as_str());
        let such_line = content.split("\n").skip(line).next().unwrap();
        view.push_str(such_line);
        view.push_str(format!("\n{}   | ", &extra_whitespaces).as_str());
        view.push_str(&arrow_whitespaces);
        view.push_str("^");
    }

    view
}
