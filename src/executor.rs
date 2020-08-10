// executor won't define structure of executed block/receipt/log etc.
// implement can customize it.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallRequest {
    /// 1. length is 20 bytes for evm.
    /// 2. if executor is multi-vm, it will be a path.
    #[prost(bytes, tag = "1")]
    pub to: std::vec::Vec<u8>,
    /// 4 bytes for evm.
    #[prost(bytes, tag = "2")]
    pub method: std::vec::Vec<u8>,
    #[prost(bytes, repeated, tag = "3")]
    pub args: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallResponse {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod executor_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ExecutorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExecutorServiceClient<tonic::transport::Channel> {
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
    impl<T> ExecutorServiceClient<T>
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
        #[doc = " exec a block return executed_block_hash"]
        pub async fn exec(
            &mut self,
            request: impl tonic::IntoRequest<super::super::blockchain::CompactBlock>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/executor.ExecutorService/Exec");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn call(
            &mut self,
            request: impl tonic::IntoRequest<super::CallRequest>,
        ) -> Result<tonic::Response<super::CallResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/executor.ExecutorService/Call");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ExecutorServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ExecutorServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ExecutorServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod executor_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ExecutorServiceServer."]
    #[async_trait]
    pub trait ExecutorService: Send + Sync + 'static {
        #[doc = " exec a block return executed_block_hash"]
        async fn exec(
            &self,
            request: tonic::Request<super::super::blockchain::CompactBlock>,
        ) -> Result<tonic::Response<super::super::common::Hash>, tonic::Status>;
        async fn call(
            &self,
            request: tonic::Request<super::CallRequest>,
        ) -> Result<tonic::Response<super::CallResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ExecutorServiceServer<T: ExecutorService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ExecutorService> ExecutorServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ExecutorServiceServer<T>
    where
        T: ExecutorService,
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
                "/executor.ExecutorService/Exec" => {
                    #[allow(non_camel_case_types)]
                    struct ExecSvc<T: ExecutorService>(pub Arc<T>);
                    impl<T: ExecutorService>
                        tonic::server::UnaryService<super::super::blockchain::CompactBlock>
                        for ExecSvc<T>
                    {
                        type Response = super::super::common::Hash;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::blockchain::CompactBlock>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.exec(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ExecSvc(inner);
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
                "/executor.ExecutorService/Call" => {
                    #[allow(non_camel_case_types)]
                    struct CallSvc<T: ExecutorService>(pub Arc<T>);
                    impl<T: ExecutorService> tonic::server::UnaryService<super::CallRequest> for CallSvc<T> {
                        type Response = super::CallResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CallRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.call(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CallSvc(inner);
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
    impl<T: ExecutorService> Clone for ExecutorServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ExecutorService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ExecutorService> tonic::transport::NamedService for ExecutorServiceServer<T> {
        const NAME: &'static str = "executor.ExecutorService";
    }
}
