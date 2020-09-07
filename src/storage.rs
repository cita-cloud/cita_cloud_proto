#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    #[prost(uint32, tag = "1")]
    pub region: u32,
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtKey {
    #[prost(uint32, tag = "1")]
    pub region: u32,
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Regions {
    Global = 0,
    Transactions = 1,
    Headers = 2,
    Bodies = 3,
    BlockHash = 4,
    Proof = 5,
    Result = 6,
    Button = 7,
}
#[doc = r" Generated client implementations."]
pub mod storage_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct StorageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageServiceClient<tonic::transport::Channel> {
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
    impl<T> StorageServiceClient<T>
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
        #[doc = " store key/value"]
        pub async fn store(
            &mut self,
            request: impl tonic::IntoRequest<super::Content>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.StorageService/Store");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " given a ext key return value"]
        pub async fn load(
            &mut self,
            request: impl tonic::IntoRequest<super::ExtKey>,
        ) -> Result<tonic::Response<super::Value>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.StorageService/Load");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " given a ext key delete it"]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::ExtKey>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.StorageService/Delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for StorageServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for StorageServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StorageServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod storage_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with StorageServiceServer."]
    #[async_trait]
    pub trait StorageService: Send + Sync + 'static {
        #[doc = " store key/value"]
        async fn store(
            &self,
            request: tonic::Request<super::Content>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status>;
        #[doc = " given a ext key return value"]
        async fn load(
            &self,
            request: tonic::Request<super::ExtKey>,
        ) -> Result<tonic::Response<super::Value>, tonic::Status>;
        #[doc = " given a ext key delete it"]
        async fn delete(
            &self,
            request: tonic::Request<super::ExtKey>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct StorageServiceServer<T: StorageService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: StorageService> StorageServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for StorageServiceServer<T>
    where
        T: StorageService,
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
                "/storage.StorageService/Store" => {
                    #[allow(non_camel_case_types)]
                    struct StoreSvc<T: StorageService>(pub Arc<T>);
                    impl<T: StorageService> tonic::server::UnaryService<super::Content> for StoreSvc<T> {
                        type Response = super::super::common::SimpleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Content>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.store(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StoreSvc(inner);
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
                "/storage.StorageService/Load" => {
                    #[allow(non_camel_case_types)]
                    struct LoadSvc<T: StorageService>(pub Arc<T>);
                    impl<T: StorageService> tonic::server::UnaryService<super::ExtKey> for LoadSvc<T> {
                        type Response = super::Value;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::ExtKey>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.load(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LoadSvc(inner);
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
                "/storage.StorageService/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: StorageService>(pub Arc<T>);
                    impl<T: StorageService> tonic::server::UnaryService<super::ExtKey> for DeleteSvc<T> {
                        type Response = super::super::common::SimpleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::ExtKey>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
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
    impl<T: StorageService> Clone for StorageServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: StorageService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StorageService> tonic::transport::NamedService for StorageServiceServer<T> {
        const NAME: &'static str = "storage.StorageService";
    }
}
