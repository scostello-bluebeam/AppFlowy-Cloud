use crate::util::test_client::TestClient;
use app_error::ErrorCode;
use collab_entity::CollabType;
use database_entity::dto::{CollabParams, CreateCollabParams, QueryCollab};
use serde::Serialize;
use serde_json::json;

use uuid::Uuid;

#[tokio::test]
async fn batch_insert_collab_with_empty_payload_test() {
  let mut test_client = TestClient::new_user().await;
  let workspace_id = test_client.workspace_id().await;

  let error = test_client
    .batch_create_collab(&workspace_id, vec![])
    .await
    .unwrap_err();

  assert_eq!(error.code, ErrorCode::InvalidRequest);
}

#[tokio::test]
async fn batch_insert_collab_success_test() {
  let mut test_client = TestClient::new_user().await;
  let workspace_id = test_client.workspace_id().await;

  let params_list = (0..5)
    .map(|_| CollabParams {
      object_id: Uuid::new_v4().to_string(),
      encoded_collab_v1: vec![0, 200],
      collab_type: CollabType::Document,
    })
    .collect::<Vec<_>>();

  test_client
    .batch_create_collab(&workspace_id, params_list.clone())
    .await
    .unwrap();

  let params = params_list
    .into_iter()
    .map(|params| QueryCollab {
      object_id: params.object_id,
      collab_type: params.collab_type,
    })
    .collect::<Vec<_>>();

  let result = test_client
    .batch_get_collab(&workspace_id, params)
    .await
    .unwrap();

  assert_eq!(result.0.values().len(), 5);
}

#[tokio::test]
async fn create_collab_params_compatibility_serde_test() {
  // This test is to make sure that the CreateCollabParams is compatible with the old InsertCollabParams
  let old_version_value = json!(InsertCollabParams {
    object_id: "object_id".to_string(),
    encoded_collab_v1: vec![0, 200],
    workspace_id: "workspace_id".to_string(),
    collab_type: CollabType::Document,
  });

  let new_version_create_params =
    serde_json::from_value::<CreateCollabParams>(old_version_value.clone()).unwrap();

  let new_version_value = serde_json::to_value(new_version_create_params.clone()).unwrap();
  assert_eq!(old_version_value, new_version_value);

  assert_eq!(new_version_create_params.object_id, "object_id".to_string());
  assert_eq!(new_version_create_params.encoded_collab_v1, vec![0, 200]);
  assert_eq!(
    new_version_create_params.workspace_id,
    "workspace_id".to_string()
  );
  assert_eq!(new_version_create_params.collab_type, CollabType::Document);
}

#[derive(Serialize)]
struct InsertCollabParams {
  pub object_id: String,
  pub encoded_collab_v1: Vec<u8>,
  pub workspace_id: String,
  pub collab_type: CollabType,
}