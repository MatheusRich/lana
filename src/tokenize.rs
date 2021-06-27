pub fn tokenize(code: String) -> Vec<String> {
    code.replace("(", " ( ")
        .replace(")", " ) ")
        .replace(",", " ")
        .split_whitespace()
        .map(|it| it.to_string())
        .collect()
}



