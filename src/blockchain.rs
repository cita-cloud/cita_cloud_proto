#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    /// hash of previous BlockHeader
    #[prost(bytes, tag = "1")]
    pub prevhash: std::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    #[prost(bytes, tag = "4")]
    pub transactions_root: std::vec::Vec<u8>,
    #[prost(bytes, tag = "5")]
    pub proposer: std::vec::Vec<u8>,
    /// 1. use aggregated signature, it's proof of current block
    /// 2. otherwise, it's proof of previous block
    #[prost(bytes, tag = "6")]
    pub proof: std::vec::Vec<u8>,
    /// 1. executed before consensus, it's hash of current ExecutedBlock
    /// 2. otherwise, it's hash of previous ExecutedBlock
    /// Note: ExecutedBlock is defined by executor.
    #[prost(bytes, tag = "7")]
    pub executed_block_hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    /// 1. length is 20 bytes for evm.
    /// 2. if executor is multi-vm, it will be a path.
    #[prost(bytes, tag = "2")]
    pub to: std::vec::Vec<u8>,
    /// length is less than 128
    #[prost(string, tag = "3")]
    pub nonce: std::string::String,
    #[prost(uint64, tag = "4")]
    pub quota: u64,
    #[prost(uint64, tag = "5")]
    pub valid_until_block: u64,
    #[prost(bytes, tag = "6")]
    pub data: std::vec::Vec<u8>,
    /// length is 32 bytes.
    #[prost(bytes, tag = "7")]
    pub value: std::vec::Vec<u8>,
    /// length is 32 bytes.
    #[prost(bytes, tag = "8")]
    pub chain_id: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Witness {
    #[prost(bytes, tag = "1")]
    pub signature: std::vec::Vec<u8>,
    /// add to support multi-address, or we don't know which address algorithm to use
    #[prost(bytes, tag = "2")]
    pub sender: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnverifiedTransaction {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::std::option::Option<Transaction>,
    /// add to support multi-hash, or we don't know which hash algorithm to use
    #[prost(bytes, tag = "2")]
    pub transaction_hash: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub witness: ::std::option::Option<Witness>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoTransaction {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(bytes, tag = "2")]
    pub pre_tx_hash: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub output: std::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnverifiedUtxoTransaction {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::std::option::Option<UtxoTransaction>,
    /// add to support multi-hash, or we don't know which hash algorithm to use
    #[prost(bytes, tag = "2")]
    pub transaction_hash: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub witnesses: ::std::vec::Vec<Witness>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBlockBody {
    /// transaction hash of UnverifiedTransaction or UnverifyedUtxoTransaction.
    #[prost(bytes, repeated, tag = "1")]
    pub tx_hashes: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBlock {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(message, optional, tag = "2")]
    pub header: ::std::option::Option<BlockHeader>,
    #[prost(message, optional, tag = "3")]
    pub body: ::std::option::Option<CompactBlockBody>,
}
