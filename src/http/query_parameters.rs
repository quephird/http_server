use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryParameters<'qp> {
    params: HashMap<&'qp str, Vec<&'qp str>>,
}

impl<'qp> QueryParameters<'qp> {
    pub fn new(params: HashMap<&'qp str, Vec<&'qp str>>) -> Self {
        Self {
            params,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Vec<&str>> {
        self.params.get(key)
    }
}

impl<'qp> From<&'qp str> for QueryParameters<'qp> {
    fn from(query_string: &'qp str) -> Self {
        let mut params = HashMap::new();
        let key_value_pairs = query_string.split("&");

        for key_value_pair in key_value_pairs {
            let key_value_vec : Vec<&str> = key_value_pair.splitn(2, "=").collect();
            if key_value_vec[0].is_empty() {
                continue;
            }
            
            let key = key_value_vec[0];
            let new_value = if key_value_vec.len() == 1 { "" } else { key_value_vec[1] };

            params
                .entry(key)
                .and_modify(|existing_values: &mut Vec<&str>| existing_values.push(new_value))
                .or_insert(vec!(new_value));
        }

        QueryParameters{ params }
    }
}