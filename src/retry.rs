// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    client::{
        ConsensusClientTrait, ControllerClientTrait, CryptoClientTrait, EVMClientTrait,
        ExecutorClientTrait, InterceptedSvc, NetworkClientTrait,
        NetworkMsgHandlerServiceClientTrait, RPCClientTrait, RetryConfig, StorageClientTrait,
    },
    consensus::consensus_service_client::ConsensusServiceClient,
    controller::consensus2_controller_service_client::Consensus2ControllerServiceClient,
    controller::rpc_service_client::RpcServiceClient,
    crypto::crypto_service_client::CryptoServiceClient,
    evm::rpc_service_client::RpcServiceClient as EVMServiceClient,
    executor::executor_service_client::ExecutorServiceClient,
    network::network_msg_handler_service_client::NetworkMsgHandlerServiceClient,
    network::network_service_client::NetworkServiceClient,
    storage::storage_service_client::StorageServiceClient,
};
use backoff::{backoff::Backoff, ExponentialBackoff};
use futures_retry::{ErrorHandler, FutureRetry, RetryPolicy};
use log::{debug, warn};
use std::{fmt::Debug, future::Future, time::Duration};
use tonic::Code;

type Result<T, E = tonic::Status> = std::result::Result<T, E>;

/// List of gRPC error codes that client will retry.
pub const RETRYABLE_ERROR_CODES: [Code; 7] = [
    Code::DataLoss,
    Code::Internal,
    Code::Unknown,
    Code::ResourceExhausted,
    Code::Aborted,
    Code::OutOfRange,
    Code::Unavailable,
];

/// A wrapper for a [WorkflowClientTrait] or [crate::WorkflowService] implementor which performs
/// auto-retries
#[derive(Debug, Clone)]
pub struct RetryClient<SG> {
    client: SG,
    retry_config: RetryConfig,
}

impl<SG> RetryClient<SG> {
    /// Use the provided retry config with the provided client
    pub const fn new(client: SG, retry_config: RetryConfig) -> Self {
        Self {
            client,
            retry_config,
        }
    }
}

impl<SG: Clone> RetryClient<SG> {
    /// Return the inner client type
    pub fn get_client(&self) -> &SG {
        &self.client
    }

    /// Return the inner client type mutably
    pub fn get_client_mut(&mut self) -> &mut SG {
        &mut self.client
    }

    pub fn get_client_clone(&self) -> SG {
        self.client.clone()
    }

    /// Disable retry and return the inner client type
    pub fn into_inner(self) -> SG {
        self.client
    }

    /// Wraps a call to the underlying client with retry capability.
    ///
    /// This is the "old" path used by higher-level [WorkflowClientTrait] implementors
    pub(crate) async fn call_with_retry<R, F, Fut>(
        &self,
        factory: F,
        call_name: &'static str,
    ) -> Result<R>
    where
        F: Fn() -> Fut + Unpin,
        Fut: Future<Output = Result<R>>,
    {
        let rtc = self.get_retry_config();
        let res = Self::make_future_retry(rtc, factory, call_name).await;
        Ok(res.map_err(|(e, _attempt)| e)?.0)
    }

    pub(crate) fn get_retry_config(&self) -> RetryConfig {
        self.retry_config.clone()
    }

    pub(crate) fn make_future_retry<R, F, Fut>(
        rtc: RetryConfig,
        factory: F,
        call_name: &'static str,
    ) -> FutureRetry<F, TonicErrorHandler>
    where
        F: FnMut() -> Fut + Unpin,
        Fut: Future<Output = Result<R>>,
    {
        FutureRetry::new(factory, TonicErrorHandler::new(rtc, call_name))
    }
}

#[derive(Debug)]
pub(crate) struct TonicErrorHandler {
    backoff: ExponentialBackoff,
    max_retries: usize,
    call_name: &'static str,
}
impl TonicErrorHandler {
    fn new(cfg: RetryConfig, call_name: &'static str) -> Self {
        Self {
            max_retries: cfg.max_retries,
            backoff: cfg.into(),
            call_name,
        }
    }

    const fn should_log_retry_warning(&self, cur_attempt: usize) -> bool {
        // Warn on more than 5 retries for unlimited retrying
        if self.max_retries == 0 && cur_attempt > 5 {
            return true;
        }
        // Warn if the attempts are more than 50% of max retries
        if self.max_retries > 0 && cur_attempt * 2 >= self.max_retries {
            return true;
        }
        false
    }
}

impl ErrorHandler<tonic::Status> for TonicErrorHandler {
    type OutError = tonic::Status;

    fn handle(&mut self, current_attempt: usize, e: tonic::Status) -> RetryPolicy<tonic::Status> {
        // 0 max retries means unlimited retries
        if self.max_retries > 0 && current_attempt >= self.max_retries {
            return RetryPolicy::ForwardError(e);
        }

        if RETRYABLE_ERROR_CODES.contains(&e.code()) {
            if current_attempt == 1 {
                debug!("gRPC call {} failed on first attempt", self.call_name);
            } else if self.should_log_retry_warning(current_attempt) {
                warn!(
                    "gRPC call {} retried {} times",
                    self.call_name, current_attempt
                );
            }

            match self.backoff.next_backoff() {
                None => RetryPolicy::ForwardError(e), // None is returned when we've ran out of time
                Some(backoff) => {
                    if cfg!(test) {
                        // Allow unit tests to do lots of retries quickly. This does *not* apply
                        // during integration testing, importantly.
                        RetryPolicy::WaitRetry(Duration::from_millis(1))
                    } else {
                        RetryPolicy::WaitRetry(backoff)
                    }
                }
            }
        } else {
            RetryPolicy::ForwardError(e)
        }
    }
}

macro_rules! retry_call {
    ($myself:ident, $call_name:ident) => { retry_call!($myself, $call_name,) };
    ($myself:ident, $call_name:ident, $($args:expr),*) => {{
        let call_name_str = stringify!($call_name);
        let fact = || { async { $myself.get_client_clone().$call_name($($args,)*).await.map(|ret| ret.into_inner()) }};
        $myself.call_with_retry(fact, call_name_str).await
    }}
}

use crate::{blockchain, common, controller, crypto, evm, executor, network, storage};

#[async_trait::async_trait]
impl RPCClientTrait for RetryClient<RpcServiceClient<InterceptedSvc>> {
    async fn get_block_number(
        &self,
        flag: controller::Flag,
    ) -> Result<controller::BlockNumber, tonic::Status> {
        retry_call!(self, get_block_number, flag.clone())
    }

    async fn send_raw_transaction(
        &self,
        tx: blockchain::RawTransaction,
    ) -> Result<common::Hash, tonic::Status> {
        retry_call!(self, send_raw_transaction, tx.clone())
    }

    async fn send_raw_transactions(
        &self,
        txs: blockchain::RawTransactions,
    ) -> Result<common::Hashes, tonic::Status> {
        retry_call!(self, send_raw_transactions, txs.clone())
    }

    async fn get_block_by_hash(
        &self,
        hash: common::Hash,
    ) -> Result<blockchain::CompactBlock, tonic::Status> {
        retry_call!(self, get_block_by_hash, hash.clone())
    }

    async fn get_block_by_number(
        &self,
        n: controller::BlockNumber,
    ) -> Result<blockchain::CompactBlock, tonic::Status> {
        retry_call!(self, get_block_by_number, n.clone())
    }

    async fn get_block_detail_by_number(
        &self,
        n: controller::BlockNumber,
    ) -> Result<blockchain::Block, tonic::Status> {
        retry_call!(self, get_block_detail_by_number, n.clone())
    }

    async fn get_transaction(
        &self,
        hash: common::Hash,
    ) -> Result<blockchain::RawTransaction, tonic::Status> {
        retry_call!(self, get_transaction, hash.clone())
    }

    async fn get_system_config(
        &self,
        e: common::Empty,
    ) -> Result<controller::SystemConfig, tonic::Status> {
        retry_call!(self, get_system_config, e.clone())
    }

    async fn get_version(
        &self,
        e: common::Empty,
    ) -> Result<controller::SoftwareVersion, tonic::Status> {
        retry_call!(self, get_version, e.clone())
    }

    async fn get_block_hash(
        &self,
        n: controller::BlockNumber,
    ) -> Result<common::Hash, tonic::Status> {
        retry_call!(self, get_block_hash, n.clone())
    }

    async fn get_transaction_block_number(
        &self,
        hash: common::Hash,
    ) -> Result<controller::BlockNumber, tonic::Status> {
        retry_call!(self, get_transaction_block_number, hash.clone())
    }

    async fn get_transaction_index(
        &self,
        hash: common::Hash,
    ) -> Result<controller::TransactionIndex, tonic::Status> {
        retry_call!(self, get_transaction_index, hash.clone())
    }

    async fn get_peer_count(
        &self,
        e: common::Empty,
    ) -> Result<controller::PeerCount, tonic::Status> {
        retry_call!(self, get_peer_count, e.clone())
    }

    async fn add_node(
        &self,
        node_info: common::NodeNetInfo,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, add_node, node_info.clone())
    }

    async fn get_peers_info(
        &self,
        e: common::Empty,
    ) -> Result<common::TotalNodeInfo, tonic::Status> {
        retry_call!(self, get_peers_info, e.clone())
    }
}

#[async_trait::async_trait]
impl EVMClientTrait for RetryClient<EVMServiceClient<InterceptedSvc>> {
    async fn get_transaction_receipt(
        &self,
        hash: common::Hash,
    ) -> Result<evm::Receipt, tonic::Status> {
        retry_call!(self, get_transaction_receipt, hash.clone())
    }

    async fn get_code(&self, address: common::Address) -> Result<evm::ByteCode, tonic::Status> {
        retry_call!(self, get_code, address.clone())
    }

    async fn get_balance(&self, address: common::Address) -> Result<evm::Balance, tonic::Status> {
        retry_call!(self, get_balance, address.clone())
    }

    async fn get_transaction_count(
        &self,
        address: common::Address,
    ) -> Result<evm::Nonce, tonic::Status> {
        retry_call!(self, get_transaction_count, address.clone())
    }

    async fn get_abi(&self, address: common::Address) -> Result<evm::ByteAbi, tonic::Status> {
        retry_call!(self, get_abi, address.clone())
    }
}

#[async_trait::async_trait]
impl ControllerClientTrait for RetryClient<Consensus2ControllerServiceClient<InterceptedSvc>> {
    async fn get_proposal(
        &self,
        e: common::Empty,
    ) -> Result<common::ProposalResponse, tonic::Status> {
        retry_call!(self, get_proposal, e.clone())
    }

    async fn check_proposal(
        &self,
        proposal: common::Proposal,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, check_proposal, proposal.clone())
    }

    async fn commit_block(
        &self,
        pp: common::ProposalWithProof,
    ) -> Result<common::ConsensusConfigurationResponse, tonic::Status> {
        retry_call!(self, commit_block, pp.clone())
    }
}

#[async_trait::async_trait]
impl NetworkClientTrait for RetryClient<NetworkServiceClient<InterceptedSvc>> {
    async fn send_msg(
        &self,
        msg: network::NetworkMsg,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, send_msg, msg.clone())
    }

    async fn broadcast(
        &self,
        msg: network::NetworkMsg,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, broadcast, msg.clone())
    }

    async fn get_network_status(
        &self,
        e: common::Empty,
    ) -> Result<network::NetworkStatusResponse, tonic::Status> {
        retry_call!(self, get_network_status, e.clone())
    }

    async fn register_network_msg_handler(
        &self,
        info: network::RegisterInfo,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, register_network_msg_handler, info.clone())
    }

    async fn add_node(
        &self,
        info: common::NodeNetInfo,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, add_node, info.clone())
    }

    async fn get_peers_net_info(
        &self,
        e: common::Empty,
    ) -> Result<common::TotalNodeNetInfo, tonic::Status> {
        retry_call!(self, get_peers_net_info, e.clone())
    }
}

#[async_trait::async_trait]
impl NetworkMsgHandlerServiceClientTrait
    for RetryClient<NetworkMsgHandlerServiceClient<InterceptedSvc>>
{
    async fn process_network_msg(
        &self,
        msg: network::NetworkMsg,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, process_network_msg, msg.clone())
    }
}

#[async_trait::async_trait]
impl ConsensusClientTrait for RetryClient<ConsensusServiceClient<InterceptedSvc>> {
    async fn reconfigure(
        &self,
        config: common::ConsensusConfiguration,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, reconfigure, config.clone())
    }

    async fn check_block(
        &self,
        pp: common::ProposalWithProof,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, check_block, pp.clone())
    }
}

#[async_trait::async_trait]
impl CryptoClientTrait for RetryClient<CryptoServiceClient<InterceptedSvc>> {
    async fn get_crypto_info(
        &self,
        e: common::Empty,
    ) -> Result<crypto::GetCryptoInfoResponse, tonic::Status> {
        retry_call!(self, get_crypto_info, e.clone())
    }

    async fn hash_data(
        &self,
        data: crypto::HashDataRequest,
    ) -> Result<common::HashResponse, tonic::Status> {
        retry_call!(self, hash_data, data.clone())
    }

    async fn verify_data_hash(
        &self,
        verify_data: crypto::VerifyDataHashRequest,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, verify_data_hash, verify_data.clone())
    }

    async fn sign_message(
        &self,
        msg: crypto::SignMessageRequest,
    ) -> Result<crypto::SignMessageResponse, tonic::Status> {
        retry_call!(self, sign_message, msg.clone())
    }

    async fn recover_signature(
        &self,
        sig: crypto::RecoverSignatureRequest,
    ) -> Result<crypto::RecoverSignatureResponse, tonic::Status> {
        retry_call!(self, recover_signature, sig.clone())
    }

    async fn check_transactions(
        &self,
        txs: blockchain::RawTransactions,
    ) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, check_transactions, txs.clone())
    }
}

#[async_trait::async_trait]
impl ExecutorClientTrait for RetryClient<ExecutorServiceClient<InterceptedSvc>> {
    async fn exec(&self, block: blockchain::Block) -> Result<common::HashResponse, tonic::Status> {
        retry_call!(self, exec, block.clone())
    }

    async fn call(
        &self,
        request: executor::CallRequest,
    ) -> Result<executor::CallResponse, tonic::Status> {
        retry_call!(self, call, request.clone())
    }
}

#[async_trait::async_trait]
impl StorageClientTrait for RetryClient<StorageServiceClient<InterceptedSvc>> {
    async fn store(&self, content: storage::Content) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, store, content.clone())
    }

    async fn load(&self, key: storage::ExtKey) -> Result<storage::Value, tonic::Status> {
        retry_call!(self, load, key.clone())
    }

    async fn delete(&self, key: storage::ExtKey) -> Result<common::StatusCode, tonic::Status> {
        retry_call!(self, delete, key.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Status;

    static mut COUNT: u64 = 0;

    #[derive(Clone)]
    struct TestClient {
        code: Code,
    }

    impl TestClient {
        fn new(code: Code) -> Self {
            Self { code }
        }

        async fn test(&mut self, i: u64) -> Result<tonic::Response<()>, Status> {
            unsafe {
                if i < COUNT {
                    Ok(tonic::Response::new(()))
                } else {
                    COUNT += 1;
                    Err(Status::new(self.code, "failure"))
                }
            }
        }
    }

    #[async_trait::async_trait]
    trait TestClientTrait {
        async fn test(&self, i: u64) -> Result<(), Status>;
    }

    #[async_trait::async_trait]
    impl TestClientTrait for RetryClient<TestClient> {
        async fn test(&self, i: u64) -> Result<(), Status> {
            retry_call!(self, test, i)
        }
    }

    #[tokio::test]
    async fn non_retryable_errors() {
        for code in [
            Code::InvalidArgument,
            Code::NotFound,
            Code::AlreadyExists,
            Code::PermissionDenied,
            Code::FailedPrecondition,
            Code::Cancelled,
            Code::DeadlineExceeded,
            Code::Unauthenticated,
            Code::Unimplemented,
        ] {
            unsafe {
                COUNT = 0;
            }
            let mock_client = TestClient::new(code);
            let retry_client = RetryClient::new(mock_client, Default::default());
            let result = retry_client.test(1).await;
            // Expecting an error after a single attempt, since there was a non-retryable error.
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn retryable_errors() {
        for code in RETRYABLE_ERROR_CODES {
            unsafe {
                COUNT = 0;
            }
            let mock_client = TestClient::new(code);
            let retry_client = RetryClient::new(mock_client, Default::default());
            let result = retry_client.test(3).await;
            // Expecting successful response after retries
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn deadline_retryable_errors() {
        for code in RETRYABLE_ERROR_CODES {
            unsafe {
                COUNT = 0;
            }
            let mock_client = TestClient::new(code);
            let retry_client = RetryClient::new(mock_client, Default::default());
            let result = retry_client.test(50).await;
            // Expecting successful response after retries
            assert!(result.is_err());
        }
    }
}
