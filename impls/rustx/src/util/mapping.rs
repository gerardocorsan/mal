use std::collections::HashMap;

pub fn smap(origin_map: HashMap<&str, &str>) -> HashMap<String, String> {
    let map: HashMap<String, String> = origin_map
        .iter()
        .map(|(key, value)| (format!("{}", key), format!("{}", value)))
        .collect();

    map
}
