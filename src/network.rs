#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkStatusResponse {
    #[prost(uint64, tag = "1")]
    pub peer_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkMsg {
    #[prost(string, tag = "1")]
    pub module: std::string::String,
    #[prost(string, tag = "2")]
    pub r#type: std::string::String,
    #[prost(uint64, tag = "3")]
    pub origin: u64,
    #[prost(bytes, tag = "4")]
    pub msg: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterInfo {
    #[prost(string, tag = "1")]
    pub module_name: std::string::String,
    #[prost(string, tag = "2")]
    pub hostname: std::string::String,
    #[prost(string, tag = "3")]
    pub port: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod network_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct NetworkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NetworkServiceClient<tonic::transport::Channel> {
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
    impl<T> NetworkServiceClient<T>
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
        #[doc = " send message to a single peer."]
        pub async fn send_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::NetworkMsg>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/network.NetworkService/SendMsg");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " broadcast message to all peers."]
        pub async fn broadcast(
            &mut self,
            request: impl tonic::IntoRequest<super::NetworkMsg>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/network.NetworkService/Broadcast");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_network_status(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::NetworkStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/network.NetworkService/GetNetworkStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " modules which need network register its msg handler."]
        #[doc = " args: module name; hostname and port of handler"]
        pub async fn register_network_msg_handler(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterInfo>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/network.NetworkService/RegisterNetworkMsgHandler",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for NetworkServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for NetworkServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "NetworkServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod network_msg_handler_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " modules which need network must implement this service"]
    pub struct NetworkMsgHandlerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NetworkMsgHandlerServiceClient<tonic::transport::Channel> {
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
    impl<T> NetworkMsgHandlerServiceClient<T>
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
        pub async fn process_network_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::NetworkMsg>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/network.NetworkMsgHandlerService/ProcessNetworkMsg",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for NetworkMsgHandlerServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for NetworkMsgHandlerServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "NetworkMsgHandlerServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod network_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with NetworkServiceServer."]
    #[async_trait]
    pub trait NetworkService: Send + Sync + 'static {
        #[doc = " send message to a single peer."]
        async fn send_msg(
            &self,
            request: tonic::Request<super::NetworkMsg>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status>;
        #[doc = " broadcast message to all peers."]
        async fn broadcast(
            &self,
            request: tonic::Request<super::NetworkMsg>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status>;
        async fn get_network_status(
            &self,
            request: tonic::Request<super::super::common::Empty>,
        ) -> Result<tonic::Response<super::NetworkStatusResponse>, tonic::Status>;
        #[doc = " modules which need network register its msg handler."]
        #[doc = " args: module name; hostname and port of handler"]
        async fn register_network_msg_handler(
            &self,
            request: tonic::Request<super::RegisterInfo>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct NetworkServiceServer<T: NetworkService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: NetworkService> NetworkServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for NetworkServiceServer<T>
    where
        T: NetworkService,
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
                "/network.NetworkService/SendMsg" => {
                    #[allow(non_camel_case_types)]
                    struct SendMsgSvc<T: NetworkService>(pub Arc<T>);
                    impl<T: NetworkService> tonic::server::UnaryService<super::NetworkMsg> for SendMsgSvc<T> {
                        type Response = super::super::common::SimpleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NetworkMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.send_msg(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendMsgSvc(inner);
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
                "/network.NetworkService/Broadcast" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastSvc<T: NetworkService>(pub Arc<T>);
                    impl<T: NetworkService> tonic::server::UnaryService<super::NetworkMsg> for BroadcastSvc<T> {
                        type Response = super::super::common::SimpleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NetworkMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.broadcast(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BroadcastSvc(inner);
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
                "/network.NetworkService/GetNetworkStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetNetworkStatusSvc<T: NetworkService>(pub Arc<T>);
                    impl<T: NetworkService> tonic::server::UnaryService<super::super::common::Empty>
                        for GetNetworkStatusSvc<T>
                    {
                        type Response = super::NetworkStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::common::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_network_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetNetworkStatusSvc(inner);
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
                "/network.NetworkService/RegisterNetworkMsgHandler" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterNetworkMsgHandlerSvc<T: NetworkService>(pub Arc<T>);
                    impl<T: NetworkService> tonic::server::UnaryService<super::RegisterInfo>
                        for RegisterNetworkMsgHandlerSvc<T>
                    {
                        type Response = super::super::common::SimpleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.register_network_msg_handler(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RegisterNetworkMsgHandlerSvc(inner);
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
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: NetworkService> Clone for NetworkServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: NetworkService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: NetworkService> tonic::transport::NamedService for NetworkServiceServer<T> {
        const NAME: &'static str = "network.NetworkService";
    }
}
#[doc = r" Generated server implementations."]
pub mod network_msg_handler_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with NetworkMsgHandlerServiceServer."]
    #[async_trait]
    pub trait NetworkMsgHandlerService: Send + Sync + 'static {
        async fn process_network_msg(
            &self,
            request: tonic::Request<super::NetworkMsg>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status>;
    }
    #[doc = " modules which need network must implement this service"]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct NetworkMsgHandlerServiceServer<T: NetworkMsgHandlerService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: NetworkMsgHandlerService> NetworkMsgHandlerServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for NetworkMsgHandlerServiceServer<T>
    where
        T: NetworkMsgHandlerService,
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
                "/network.NetworkMsgHandlerService/ProcessNetworkMsg" => {
                    #[allow(non_camel_case_types)]
                    struct ProcessNetworkMsgSvc<T: NetworkMsgHandlerService>(pub Arc<T>);
                    impl<T: NetworkMsgHandlerService> tonic::server::UnaryService<super::NetworkMsg>
                        for ProcessNetworkMsgSvc<T>
                    {
                        type Response = super::super::common::SimpleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NetworkMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.process_network_msg(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ProcessNetworkMsgSvc(inner);
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
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: NetworkMsgHandlerService> Clone for NetworkMsgHandlerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: NetworkMsgHandlerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: NetworkMsgHandlerService> tonic::transport::NamedService
        for NetworkMsgHandlerServiceServer<T>
    {
        const NAME: &'static str = "network.NetworkMsgHandlerService";
    }
}
