#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flag {
    #[prost(bool, tag = "1")]
    pub flag: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockNumber {
    #[prost(uint64, tag = "1")]
    pub block_number: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemConfig {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub chain_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "4")]
    pub block_interval: u32,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, tag = "6")]
    pub emergency_brake: bool,
    #[prost(bytes = "vec", tag = "7")]
    pub version_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub chain_id_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "9")]
    pub admin_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "10")]
    pub block_interval_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "11")]
    pub validators_pre_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "12")]
    pub emergency_brake_pre_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareVersion {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerCount {
    #[prost(uint64, tag = "1")]
    pub peer_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionIndex {
    #[prost(uint64, tag = "1")]
    pub tx_index: u64,
}
#[doc = r" Generated client implementations."]
pub mod rpc_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct RpcServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RpcServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RpcServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " flag means latest or pending."]
        #[doc = " true means pending, false means latest."]
        pub async fn get_block_number(
            &mut self,
            request: impl tonic::IntoRequest<super::Flag>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetBlockNumber");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_raw_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::blockchain::RawTransaction>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/SendRawTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_by_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::super::blockchain::CompactBlock>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetBlockByHash");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::blockchain::CompactBlock>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetBlockByNumber");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::super::blockchain::RawTransaction>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_system_config(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::SystemConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetSystemConfig");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::SoftwareVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/controller.RPCService/GetVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/controller.RPCService/GetBlockHash");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_block_number(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.RPCService/GetTransactionBlockNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_index(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::TransactionIndex>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/controller.RPCService/GetTransactionIndex");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_peer_count(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::PeerCount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/controller.RPCService/GetPeerCount");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RpcServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RpcServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RpcServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod consensus2_controller_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct Consensus2ControllerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl Consensus2ControllerServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> Consensus2ControllerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Consensus request a Proposal to start consensus"]
        #[doc = " ret: proposal"]
        pub async fn get_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::super::common::Proposal>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.Consensus2ControllerService/GetProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " when Consensus received a new proposal from other nodes, it will ask controller to check it"]
        #[doc = " args: proposal hash"]
        #[doc = " ret: ok or not"]
        pub async fn check_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Proposal>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.Consensus2ControllerService/CheckProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " after Consensus, tell controller a proposal has committed"]
        pub async fn commit_block(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::ProposalWithProof>,
        ) -> Result<tonic::Response<super::super::common::ConsensusConfiguration>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/controller.Consensus2ControllerService/CommitBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for Consensus2ControllerServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for Consensus2ControllerServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Consensus2ControllerServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod rpc_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RpcServiceServer."]
    #[async_trait]
    pub trait RpcService: Send + Sync + 'static {
        #[doc = " flag means latest or pending."]
        #[doc = " true means pending, false means latest."]
        async fn get_block_number(
            &self,
            request: tonic::Request<super::Flag>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status>;
        async fn send_raw_transaction(
            &self,
            request: tonic::Request<super::super::blockchain::RawTransaction>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status>;
        async fn get_block_by_hash(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::super::blockchain::CompactBlock>, tonic::Status>;
        async fn get_block_by_number(
            &self,
            request: tonic::Request<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::blockchain::CompactBlock>, tonic::Status>;
        async fn get_transaction(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::super::blockchain::RawTransaction>, tonic::Status>;
        async fn get_system_config(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::SystemConfig>, tonic::Status>;
        async fn get_version(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::SoftwareVersion>, tonic::Status>;
        async fn get_block_hash(
            &self,
            request: tonic::Request<super::BlockNumber>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status>;
        async fn get_transaction_block_number(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::BlockNumber>, tonic::Status>;
        async fn get_transaction_index(
            &self,
            request: tonic::Request<super::super::common::Hash>,
        ) -> Result<tonic::Response<super::TransactionIndex>, tonic::Status>;
        async fn get_peer_count(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::PeerCount>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RpcServiceServer<T: RpcService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RpcService> RpcServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for RpcServiceServer<T>
    where
        T: RpcService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/controller.RPCService/GetBlockNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::Flag> for GetBlockNumberSvc<T> {
                        type Response = super::BlockNumber;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Flag>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBlockNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/SendRawTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SendRawTransactionSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService>
                        tonic::server::UnaryService<super::super::blockchain::RawTransaction>
                        for SendRawTransactionSvc<T>
                    {
                        type Response = super::super::common::Hash;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::blockchain::RawTransaction>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_raw_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendRawTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetBlockByHash" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByHashSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetBlockByHashSvc<T>
                    {
                        type Response = super::super::blockchain::CompactBlock;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_by_hash(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBlockByHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetBlockByNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::BlockNumber> for GetBlockByNumberSvc<T> {
                        type Response = super::super::blockchain::CompactBlock;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_by_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBlockByNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetTransactionSvc<T>
                    {
                        type Response = super::super::blockchain::RawTransaction;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetSystemConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetSystemConfigSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Empty>
                        for GetSystemConfigSvc<T>
                    {
                        type Response = super::SystemConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_system_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSystemConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Empty> for GetVersionSvc<T> {
                        type Response = super::SoftwareVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetBlockHash" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockHashSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::BlockNumber> for GetBlockHashSvc<T> {
                        type Response = super::super::common::Hash;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_hash(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBlockHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetTransactionBlockNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionBlockNumberSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetTransactionBlockNumberSvc<T>
                    {
                        type Response = super::BlockNumber;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_transaction_block_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTransactionBlockNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetTransactionIndex" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionIndexSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Hash>
                        for GetTransactionIndexSvc<T>
                    {
                        type Response = super::TransactionIndex;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Hash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction_index(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTransactionIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.RPCService/GetPeerCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetPeerCountSvc<T: RpcService>(pub Arc<T>);
                    impl<T: RpcService> tonic::server::UnaryService<super::super::common::Empty>
                        for GetPeerCountSvc<T>
                    {
                        type Response = super::PeerCount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_peer_count(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetPeerCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RpcService> Clone for RpcServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RpcService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RpcService> tonic::transport::NamedService for RpcServiceServer<T> {
        const NAME: &'static str = "controller.RPCService";
    }
}
#[doc = r" Generated server implementations."]
pub mod consensus2_controller_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with Consensus2ControllerServiceServer."]
    #[async_trait]
    pub trait Consensus2ControllerService: Send + Sync + 'static {
        #[doc = " Consensus request a Proposal to start consensus"]
        #[doc = " ret: proposal"]
        async fn get_proposal(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::super::common::Proposal>, tonic::Status>;
        #[doc = " when Consensus received a new proposal from other nodes, it will ask controller to check it"]
        #[doc = " args: proposal hash"]
        #[doc = " ret: ok or not"]
        async fn check_proposal(
            &self,
            request: tonic::Request<super::super::common::Proposal>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status>;
        #[doc = " after Consensus, tell controller a proposal has committed"]
        async fn commit_block(
            &self,
            request: tonic::Request<super::super::common::ProposalWithProof>,
        ) -> Result<tonic::Response<super::super::common::ConsensusConfiguration>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct Consensus2ControllerServiceServer<T: Consensus2ControllerService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Consensus2ControllerService> Consensus2ControllerServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for Consensus2ControllerServiceServer<T>
    where
        T: Consensus2ControllerService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/controller.Consensus2ControllerService/GetProposal" => {
                    #[allow(non_camel_case_types)]
                    struct GetProposalSvc<T: Consensus2ControllerService>(pub Arc<T>);
                    impl<T: Consensus2ControllerService>
                        tonic::server::UnaryService<super::super::common::Empty>
                        for GetProposalSvc<T>
                    {
                        type Response = super::super::common::Proposal;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_proposal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.Consensus2ControllerService/CheckProposal" => {
                    #[allow(non_camel_case_types)]
                    struct CheckProposalSvc<T: Consensus2ControllerService>(pub Arc<T>);
                    impl<T: Consensus2ControllerService>
                        tonic::server::UnaryService<super::super::common::Proposal>
                        for CheckProposalSvc<T>
                    {
                        type Response = super::super::common::SimpleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Proposal>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check_proposal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CheckProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/controller.Consensus2ControllerService/CommitBlock" => {
                    #[allow(non_camel_case_types)]
                    struct CommitBlockSvc<T: Consensus2ControllerService>(pub Arc<T>);
                    impl<T: Consensus2ControllerService>
                        tonic::server::UnaryService<super::super::common::ProposalWithProof>
                        for CommitBlockSvc<T>
                    {
                        type Response = super::super::common::ConsensusConfiguration;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::ProposalWithProof>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).commit_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CommitBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Consensus2ControllerService> Clone for Consensus2ControllerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Consensus2ControllerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Consensus2ControllerService> tonic::transport::NamedService
        for Consensus2ControllerServiceServer<T>
    {
        const NAME: &'static str = "controller.Consensus2ControllerService";
    }
}
