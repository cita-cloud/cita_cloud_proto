#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    /// hash of previous BlockHeader
    #[prost(bytes = "vec", tag = "1")]
    pub prevhash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub transactions_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub proposer: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    /// 1. length is 20 bytes for evm.
    /// 2. if executor is multi-vm, it will be a path.
    #[prost(bytes = "vec", tag = "2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    /// length is less than 128
    #[prost(string, tag = "3")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub quota: u64,
    #[prost(uint64, tag = "5")]
    pub valid_until_block: u64,
    #[prost(bytes = "vec", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// length is 32 bytes.
    #[prost(bytes = "vec", tag = "7")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// length is 32 bytes.
    #[prost(bytes = "vec", tag = "8")]
    pub chain_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Witness {
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// add to support multi-address, or we don't know which address algorithm to use
    #[prost(bytes = "vec", tag = "2")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnverifiedTransaction {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
    /// add to support multi-hash, or we don't know which hash algorithm to use
    #[prost(bytes = "vec", tag = "2")]
    pub transaction_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub witness: ::core::option::Option<Witness>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoTransaction {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub pre_tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub output: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnverifiedUtxoTransaction {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<UtxoTransaction>,
    /// add to support multi-hash, or we don't know which hash algorithm to use
    #[prost(bytes = "vec", tag = "2")]
    pub transaction_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub witnesses: ::prost::alloc::vec::Vec<Witness>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawTransaction {
    #[prost(oneof = "raw_transaction::Tx", tags = "1, 2")]
    pub tx: ::core::option::Option<raw_transaction::Tx>,
}
/// Nested message and enum types in `RawTransaction`.
pub mod raw_transaction {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Tx {
        #[prost(message, tag = "1")]
        NormalTx(super::UnverifiedTransaction),
        #[prost(message, tag = "2")]
        UtxoTx(super::UnverifiedUtxoTransaction),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBlockBody {
    /// transaction hash of UnverifiedTransaction or UnverifyedUtxoTransaction.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub tx_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBlock {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<BlockHeader>,
    #[prost(message, optional, tag = "3")]
    pub body: ::core::option::Option<CompactBlockBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<BlockHeader>,
    #[prost(message, repeated, tag = "3")]
    pub body: ::prost::alloc::vec::Vec<RawTransaction>,
}
