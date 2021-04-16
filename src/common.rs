#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleResponse {
    #[prost(bool, tag = "1")]
    pub is_success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalWithProof {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<Proposal>,
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
}
