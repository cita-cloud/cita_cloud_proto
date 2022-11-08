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

use crate::retry::RetryClient;
use crate::{
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
use backoff::{ExponentialBackoff, SystemClock};
use http::uri::InvalidUri;
use std::time::{Duration, Instant};
use tonic::{
    codegen::InterceptedService, metadata::MetadataValue, service::Interceptor, transport::Channel,
    Status,
};

pub type InterceptedSvc = InterceptedService<Channel, ServiceCallInterceptor>;

pub struct ClientOptions {
    client_name: String,
    target_url: String,
    retry_config: RetryConfig,
}

impl ClientOptions {
    pub fn new(client_name: String, target_url: String) -> Self {
        Self {
            client_name,
            target_url,
            retry_config: RetryConfig::default(),
        }
    }

    pub fn connect_controller(
        &self,
    ) -> Result<RetryClient<Consensus2ControllerServiceClient<InterceptedSvc>>, ClientInitError>
    {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = Consensus2ControllerServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }

    pub fn connect_network(
        &self,
    ) -> Result<RetryClient<NetworkServiceClient<InterceptedSvc>>, ClientInitError> {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = NetworkServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }

    pub fn connect_network_msg_handler(
        &self,
    ) -> Result<RetryClient<NetworkMsgHandlerServiceClient<InterceptedSvc>>, ClientInitError> {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = NetworkMsgHandlerServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }

    pub fn connect_consensus(
        &self,
    ) -> Result<RetryClient<ConsensusServiceClient<InterceptedSvc>>, ClientInitError> {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = ConsensusServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }

    pub fn connect_crypto(
        &self,
    ) -> Result<RetryClient<CryptoServiceClient<InterceptedSvc>>, ClientInitError> {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = CryptoServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }

    pub fn connect_executor(
        &self,
    ) -> Result<RetryClient<ExecutorServiceClient<InterceptedSvc>>, ClientInitError> {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = ExecutorServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }

    pub fn connect_storage(
        &self,
    ) -> Result<RetryClient<StorageServiceClient<InterceptedSvc>>, ClientInitError> {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = StorageServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }

    pub fn connect_rpc(
        &self,
    ) -> Result<RetryClient<RpcServiceClient<InterceptedSvc>>, ClientInitError> {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = RpcServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }

    pub fn connect_evm(
        &self,
    ) -> Result<RetryClient<EVMServiceClient<InterceptedSvc>>, ClientInitError> {
        let channel = Channel::from_shared(self.target_url.to_string())?;
        let channel = channel.connect_lazy();
        let interceptor = ServiceCallInterceptor {
            client_name: self.client_name.clone(),
        };
        let client = EVMServiceClient::with_interceptor(channel, interceptor);
        let retry_client = RetryClient::new(client, self.retry_config.clone());
        Ok(retry_client)
    }
}

/// Interceptor which attaches common metadata (like "client-name") to every outgoing call
#[derive(Clone)]
pub struct ServiceCallInterceptor {
    client_name: String,
}

impl Interceptor for ServiceCallInterceptor {
    /// This function will get called on each outbound request. Returning a `Status` here will
    /// cancel the request and have that status returned to the caller.
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let metadata = request.metadata_mut();
        metadata.insert(
            "client-name",
            self.client_name
                .parse()
                .unwrap_or_else(|_| MetadataValue::from_static("")),
        );

        Ok(request)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ClientInitError {
    /// Invalid URI. Configuration error, fatal.
    #[error("Invalid URI: {0:?}")]
    InvalidUri(#[from] InvalidUri),
    /// Server connection error. Crashing and restarting the worker is likely best.
    #[error("Server connection error: {0:?}")]
    TonicTransportError(#[from] tonic::transport::Error),
    /// We couldn't successfully make the `get_system_info` call at connection time to establish
    /// server capabilities / verify server is responding.
    #[error("`get_system_info` call error after connection: {0:?}")]
    SystemInfoCallError(tonic::Status),
}

/// Configuration for retrying requests to the server
#[derive(Clone, Debug)]
pub struct RetryConfig {
    /// initial wait time before the first retry.
    pub initial_interval: Duration,
    /// randomization jitter that is used as a multiplier for the current retry interval
    /// and is added or subtracted from the interval length.
    pub randomization_factor: f64,
    /// rate at which retry time should be increased, until it reaches max_interval.
    pub multiplier: f64,
    /// maximum amount of time to wait between retries.
    pub max_interval: Duration,
    /// maximum total amount of time requests should be retried for, if None is set then no limit
    /// will be used.
    pub max_elapsed_time: Option<Duration>,
    /// maximum number of retry attempts.
    pub max_retries: usize,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            initial_interval: Duration::from_millis(100), // 100 ms wait by default.
            randomization_factor: 0.2,                    // +-20% jitter.
            multiplier: 1.5, // each next retry delay will increase by 50%
            max_interval: Duration::from_secs(5), // until it reaches 5 seconds.
            max_elapsed_time: Some(Duration::from_secs(10)), // 10 seconds total allocated time for all retries.
            max_retries: 10,
        }
    }
}

impl From<RetryConfig> for ExponentialBackoff {
    fn from(c: RetryConfig) -> Self {
        Self {
            current_interval: c.initial_interval,
            initial_interval: c.initial_interval,
            randomization_factor: c.randomization_factor,
            multiplier: c.multiplier,
            max_interval: c.max_interval,
            max_elapsed_time: c.max_elapsed_time,
            clock: SystemClock::default(),
            start_time: Instant::now(),
        }
    }
}

use crate::{blockchain, common, controller, crypto, evm, executor, network, storage};

/// Define Traits for clients
#[async_trait::async_trait]
pub trait RPCClientTrait {
    async fn get_block_number(
        &self,
        flag: controller::Flag,
    ) -> Result<controller::BlockNumber, tonic::Status>;

    async fn send_raw_transaction(
        &self,
        tx: blockchain::RawTransaction,
    ) -> Result<common::Hash, tonic::Status>;

    async fn send_raw_transactions(
        &self,
        txs: blockchain::RawTransactions,
    ) -> Result<common::Hashes, tonic::Status>;

    async fn get_block_by_hash(
        &self,
        hash: common::Hash,
    ) -> Result<blockchain::CompactBlock, tonic::Status>;

    async fn get_height_by_hash(
        &self,
        hash: common::Hash,
    ) -> Result<controller::BlockNumber, tonic::Status>;

    async fn get_block_by_number(
        &self,
        n: controller::BlockNumber,
    ) -> Result<blockchain::CompactBlock, tonic::Status>;

    async fn get_state_root_by_number(
        &self,
        n: controller::BlockNumber,
    ) -> Result<common::StateRoot, tonic::Status>;

    async fn get_proof_by_number(
        &self,
        n: controller::BlockNumber,
    ) -> Result<common::Proof, tonic::Status>;

    async fn get_block_detail_by_number(
        &self,
        n: controller::BlockNumber,
    ) -> Result<blockchain::Block, tonic::Status>;

    async fn get_transaction(
        &self,
        hash: common::Hash,
    ) -> Result<blockchain::RawTransaction, tonic::Status>;

    async fn get_system_config(
        &self,
        e: common::Empty,
    ) -> Result<controller::SystemConfig, tonic::Status>;

    async fn get_system_config_by_number(
        &self,
        n: controller::BlockNumber,
    ) -> Result<controller::SystemConfig, tonic::Status>;

    async fn get_version(
        &self,
        e: common::Empty,
    ) -> Result<controller::SoftwareVersion, tonic::Status>;

    async fn get_block_hash(
        &self,
        n: controller::BlockNumber,
    ) -> Result<common::Hash, tonic::Status>;

    async fn get_transaction_block_number(
        &self,
        hash: common::Hash,
    ) -> Result<controller::BlockNumber, tonic::Status>;

    async fn get_transaction_index(
        &self,
        hash: common::Hash,
    ) -> Result<controller::TransactionIndex, tonic::Status>;

    async fn get_peer_count(
        &self,
        e: common::Empty,
    ) -> Result<controller::PeerCount, tonic::Status>;

    async fn add_node(
        &self,
        node_info: common::NodeNetInfo,
    ) -> Result<common::StatusCode, tonic::Status>;

    async fn get_peers_info(
        &self,
        e: common::Empty,
    ) -> Result<common::TotalNodeInfo, tonic::Status>;
}

#[async_trait::async_trait]
pub trait EVMClientTrait {
    async fn get_transaction_receipt(
        &self,
        hash: common::Hash,
    ) -> Result<evm::Receipt, tonic::Status>;

    async fn get_code(&self, address: common::Address) -> Result<evm::ByteCode, tonic::Status>;

    async fn get_balance(&self, address: common::Address) -> Result<evm::Balance, tonic::Status>;

    async fn get_transaction_count(
        &self,
        address: common::Address,
    ) -> Result<evm::Nonce, tonic::Status>;

    async fn get_abi(&self, address: common::Address) -> Result<evm::ByteAbi, tonic::Status>;

    async fn estimate_quota(
        &self,
        request: executor::CallRequest,
    ) -> Result<evm::ByteQuota, tonic::Status>;
}

#[async_trait::async_trait]
pub trait ControllerClientTrait {
    async fn get_proposal(
        &self,
        e: common::Empty,
    ) -> Result<common::ProposalResponse, tonic::Status>;

    async fn check_proposal(
        &self,
        proposal: common::Proposal,
    ) -> Result<common::StatusCode, tonic::Status>;

    async fn commit_block(
        &self,
        pp: common::ProposalWithProof,
    ) -> Result<common::ConsensusConfigurationResponse, tonic::Status>;
}

#[async_trait::async_trait]
pub trait NetworkClientTrait {
    async fn send_msg(&self, msg: network::NetworkMsg)
        -> Result<common::StatusCode, tonic::Status>;

    async fn broadcast(
        &self,
        msg: network::NetworkMsg,
    ) -> Result<common::StatusCode, tonic::Status>;

    async fn get_network_status(
        &self,
        e: common::Empty,
    ) -> Result<network::NetworkStatusResponse, tonic::Status>;

    async fn register_network_msg_handler(
        &self,
        info: network::RegisterInfo,
    ) -> Result<common::StatusCode, tonic::Status>;

    async fn add_node(
        &self,
        info: common::NodeNetInfo,
    ) -> Result<common::StatusCode, tonic::Status>;

    async fn get_peers_net_info(
        &self,
        e: common::Empty,
    ) -> Result<common::TotalNodeNetInfo, tonic::Status>;
}

#[async_trait::async_trait]
pub trait NetworkMsgHandlerServiceClientTrait {
    async fn process_network_msg(
        &self,
        msg: network::NetworkMsg,
    ) -> Result<common::StatusCode, tonic::Status>;
}

#[async_trait::async_trait]
pub trait ConsensusClientTrait {
    async fn reconfigure(
        &self,
        config: common::ConsensusConfiguration,
    ) -> Result<common::StatusCode, tonic::Status>;

    async fn check_block(
        &self,
        pp: common::ProposalWithProof,
    ) -> Result<common::StatusCode, tonic::Status>;
}

#[async_trait::async_trait]
pub trait CryptoClientTrait {
    async fn get_crypto_info(
        &self,
        e: common::Empty,
    ) -> Result<crypto::GetCryptoInfoResponse, tonic::Status>;

    async fn hash_data(
        &self,
        data: crypto::HashDataRequest,
    ) -> Result<common::HashResponse, tonic::Status>;

    async fn verify_data_hash(
        &self,
        verify_data: crypto::VerifyDataHashRequest,
    ) -> Result<common::StatusCode, tonic::Status>;

    async fn sign_message(
        &self,
        msg: crypto::SignMessageRequest,
    ) -> Result<crypto::SignMessageResponse, tonic::Status>;

    async fn recover_signature(
        &self,
        sig: crypto::RecoverSignatureRequest,
    ) -> Result<crypto::RecoverSignatureResponse, tonic::Status>;

    async fn check_transactions(
        &self,
        txs: blockchain::RawTransactions,
    ) -> Result<common::StatusCode, tonic::Status>;
}

#[async_trait::async_trait]
pub trait ExecutorClientTrait {
    async fn exec(&self, block: blockchain::Block) -> Result<common::HashResponse, tonic::Status>;

    async fn call(
        &self,
        request: executor::CallRequest,
    ) -> Result<executor::CallResponse, tonic::Status>;
}

#[async_trait::async_trait]
pub trait StorageClientTrait {
    async fn store(&self, content: storage::Content) -> Result<common::StatusCode, tonic::Status>;

    async fn load(&self, key: storage::ExtKey) -> Result<storage::Value, tonic::Status>;

    async fn delete(&self, key: storage::ExtKey) -> Result<common::StatusCode, tonic::Status>;
}
