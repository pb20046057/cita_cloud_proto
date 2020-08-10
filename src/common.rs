#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleResponse {
    #[prost(bool, tag = "1")]
    pub is_success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    #[prost(bytes, tag = "1")]
    pub hash: std::vec::Vec<u8>,
}
