#[derive(Debug)]
pub enum CaseStyle {
    PascalCase,
    CamelCase,
    SnakeCase,
}

pub fn convert_case(input: &str, style: CaseStyle) -> String {
    match style {
        CaseStyle::PascalCase => input
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect(),

        CaseStyle::CamelCase => input
            .split('_')
            .enumerate()
            .map(|(i, word)| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => {
                        if i == 0 {
                            first.to_lowercase().collect::<String>() + chars.as_str()
                        } else {
                            first.to_uppercase().collect::<String>() + chars.as_str()
                        }
                    }
                    None => String::new(),
                }
            })
            .collect(),

        CaseStyle::SnakeCase => input.to_string(),
    }
}