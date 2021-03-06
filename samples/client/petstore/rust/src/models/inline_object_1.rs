/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject1 {
    /// Additional data to pass to server
    #[serde(rename = "additionalMetadata", skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<String>,
    /// file to upload
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<std::path::PathBuf>,
}

impl InlineObject1 {
    pub fn new() -> InlineObject1 {
        InlineObject1 {
            additional_metadata: None,
            file: None,
        }
    }
}


