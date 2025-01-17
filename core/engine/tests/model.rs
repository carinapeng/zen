use std::collections::HashMap;
use crate::support::test_data_root;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::json;
use zen_engine::model::DecisionContent;

mod support;

#[cfg(feature = "bincode")]
mod bincode_tests {
    use crate::support::load_test_data;
    use bincode::config;
    use zen_engine::model::DecisionContent;

    #[test]
    fn jdm_bincode() {
        let decision_content = load_test_data("table.json");
        let cache_slice_r = bincode::encode_to_vec(&decision_content, config::standard());

        assert!(cache_slice_r.is_ok(), "Bincode serialisation failed");

        let cache_slice = cache_slice_r.unwrap();
        let decode_res =
            bincode::decode_from_slice::<DecisionContent, _>(&cache_slice, config::standard());

        assert!(decode_res.is_ok(), "Bincode deserialization failed");

        let decoded_decision_content = decode_res.unwrap();
        assert_eq!(decoded_decision_content.0, decision_content);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn jdm_serde() {
    let root_dir = test_data_root();
    let files = fs::read_dir(Path::new(root_dir.as_str())).unwrap();
    for maybe_file in files {
        let file = maybe_file.unwrap();
        let file_contents = fs::read_to_string(file.path()).unwrap();
        let serialized = serde_json::from_str::<DecisionContent>(&file_contents).unwrap();

        assert!(serde_json::to_string(&serialized).is_ok());
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Foo {
    bar: Bar
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
enum Bar {
    Str(String),
    HM(HashMap<String, String>)
}

#[test]
fn serde_des() {
    let json_data = r#"{"bar": "test"}"#;
    let data = serde_json::from_str::<Foo>(json_data);
    println!("{:?}", data)
}
