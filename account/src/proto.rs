use osmosis_std_derive::CosmwasmExt;

#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "xion.v1.Query/WebAuthNVerifyRegister")]
#[proto_query(path = "xion.v1.Query/WebAuthNVerifyRegister", response_type = QueryWebAuthNVerifyRegisterResponse)]
pub struct QueryWebAuthNVerifyRegisterRequest {
    #[prost(string, tag = "1")]
    pub addr: String,
    #[prost(string, tag = "2")]
    pub challenge: String,
    #[prost(string, tag = "3")]
    pub rp: String,
    #[prost(bytes, tag = "4")]
    pub data: Vec<u8>,
}

// We define the response as a prost message to be able to decode the protobuf data.
#[derive(Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct QueryWebAuthNVerifyRegisterResponse {
    #[prost(bytes, tag = "1")]
    pub credential: Vec<u8>,
}
