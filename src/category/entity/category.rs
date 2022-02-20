use chrono::DateTime;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize)]
pub struct CategoryCreate {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct CategoryUpdate {
    pub name: String,
}
