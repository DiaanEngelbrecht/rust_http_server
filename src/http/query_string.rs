use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>,
}

#[derive(Debug)]
pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(input: &'buffer str) -> Self {
        let mut data: HashMap<&str, Value> = HashMap::new();

        for param in input.split('&') {
            let mut key = param;
            let mut value = "";
            if let Some(index) = param.find('=') {
                key = &param[..index];
                value = &param[index + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_value) => {
                        *existing = Value::Multiple(vec![prev_value, value]);
                    }
                    Value::Multiple(vec_of_values) => vec_of_values.push(value),
                })
                .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}
