pub fn snake_case_reducer(mut name: String, c: char) -> String {
    if c.is_uppercase() {
        name.push('_');
        name.extend(c.to_lowercase());
    } else {
        name.push(c);
    }
    name
}

pub fn pascal_case_reducer(mut name: String, c: char) -> String {
    if c != '_' {
        name.push(c)
    }
    name
}
