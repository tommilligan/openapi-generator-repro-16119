/*
 * double-option-hashmap
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "child", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub child: Option<Option<::std::collections::HashMap<String, serde_json::Value>>>,
}

impl Parent {
    pub fn new() -> Parent {
        Parent {
            child,
        }
    }
}


