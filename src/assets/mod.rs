use std::collections::HashMap;

pub fn get_asset_list() -> Vec<(&'static str, &'static str)> {

    vec![
        ("building", include_str!("./things/building/stockpile.json")),
    ]

}

pub fn get_text_list(locale: &str) -> HashMap<&'static str, &'static str> {

    let json = match locale {
        "en" => include_str!("./text/en.json"),
        _ => include_str!("./text/en.json"),
    };

    serde_json::from_str(json).unwrap()

}
