use crate::support::{create_fs_loader, load_test_data};
use serde_json::json;
use std::ops::Deref;
use std::sync::Arc;
use zen_engine::{Decision, EvaluationError};

mod support;

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn decision_from_content() {
    let table_content = load_test_data("table.json");
    let decision = Decision::from(table_content);

    let context = json!({ "input": 5 });
    let result = decision.evaluate(&context).await;

    assert_eq!(result.unwrap().result, json!({"output": 0}));
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn decision_from_content_switch() {
    let switch_content = load_test_data("switch-node.json");
    let decision = Decision::from(switch_content);

    let context = json!({ "group": 2, "letter": "A", "number": 8 });
    let result = decision.evaluate(&context).await.unwrap();

    println!("result is {:?}", result);
}

#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn decision_from_content_recursive() {
    let recursive_content = load_test_data("recursive-table1.json");
    let decision = Decision::from(recursive_content);

    let context = json!({});
    let result = decision.evaluate(&context).await;
    match result.unwrap_err().deref() {
        EvaluationError::NodeError(e) => {
            assert_eq!(e.node_id, "0b8dcf6b-fc04-47cb-bf82-bda764e6c09b");
            assert!(e.source.to_string().contains("Loader failed"));
        }
        _ => assert!(false, "Depth limit not exceeded"),
    }

    let with_loader = decision.with_loader(Arc::new(create_fs_loader()));
    let new_result = with_loader.evaluate(&context).await;
    match new_result.unwrap_err().deref() {
        EvaluationError::NodeError(e) => {
            assert_eq!(e.source.to_string(), "Depth limit exceeded")
        }
        _ => assert!(false, "Depth limit not exceeded"),
    }
}

#[tokio::test]
async fn decision_expression_node() {
    let decision = Decision::from(load_test_data("expression.json"));
    let context = json!({
        "numbers": [1, 5, 15, 25],
        "firstName": "John",
        "lastName": "Doe"
    });

    let result = decision.evaluate(&context).await;
    assert_eq!(
        result.unwrap().result,
        json!({
            "largeNumbers": [15, 25],
            "smallNumbers": [1, 5],
            "fullName": "John Doe",
            "deep": {
                "nested": {
                    "sum": 46
                }
            }
        })
    )
}
