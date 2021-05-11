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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BftProposal {
    #[prost(bytes = "vec", tag = "1")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub pre_state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub pre_proof: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalEnum {
    #[prost(oneof = "proposal_enum::Proposal", tags = "1")]
    pub proposal: ::core::option::Option<proposal_enum::Proposal>,
}
/// Nested message and enum types in `ProposalEnum`.
pub mod proposal_enum {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Proposal {
        #[prost(message, tag = "1")]
        BftProposal(super::BftProposal),
    }
}
