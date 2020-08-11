// crypt_type:
// 0: mock for test
// 1: secp256k1 + keccak
// 2: sm2 + sm3

// key_id index of the key in kms
// reverse 0/1/2 .. 256 for each crypt_type
// For cases without key_id, for example:
// calc tx_hash of UnverifiedTransaction or other only need hash data

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeyPairRequest {
    #[prost(uint32, tag = "1")]
    pub crypt_type: u32,
    #[prost(string, tag = "2")]
    pub description: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeyPairResponse {
    #[prost(uint64, tag = "1")]
    pub key_id: u64,
    #[prost(bytes, tag = "2")]
    pub address: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashDataRequest {
    #[prost(uint64, tag = "1")]
    pub key_id: u64,
    #[prost(bytes, tag = "2")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashDataResponse {
    #[prost(bytes, tag = "1")]
    pub hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyDataHashRequest {
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageRequest {
    #[prost(uint64, tag = "1")]
    pub key_id: u64,
    #[prost(bytes, tag = "2")]
    pub msg: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageResponse {
    #[prost(bytes, tag = "1")]
    pub signature: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverSignatureRequest {
    #[prost(bytes, tag = "1")]
    pub msg: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub signature: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverSignatureResponse {
    #[prost(bytes, tag = "1")]
    pub address: std::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod kms_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct KmsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl KmsServiceClient<tonic::transport::Channel> {
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
    impl<T> KmsServiceClient<T>
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
        #[doc = " Generate a KeyPair"]
        pub async fn generate_key_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateKeyPairRequest>,
        ) -> Result<tonic::Response<super::GenerateKeyPairResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kms.KmsService/GenerateKeyPair");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Hash data"]
        pub async fn hash_data(
            &mut self,
            request: impl tonic::IntoRequest<super::HashDataRequest>,
        ) -> Result<tonic::Response<super::HashDataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kms.KmsService/HashData");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Verify hash of data"]
        pub async fn verify_data_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyDataHashRequest>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kms.KmsService/VerifyDataHash");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sign a message"]
        pub async fn sign_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessageRequest>,
        ) -> Result<tonic::Response<super::SignMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kms.KmsService/SignMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Recover signature"]
        pub async fn recover_signature(
            &mut self,
            request: impl tonic::IntoRequest<super::RecoverSignatureRequest>,
        ) -> Result<tonic::Response<super::RecoverSignatureResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kms.KmsService/RecoverSignature");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for KmsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KmsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KmsServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod kms_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with KmsServiceServer."]
    #[async_trait]
    pub trait KmsService: Send + Sync + 'static {
        #[doc = " Generate a KeyPair"]
        async fn generate_key_pair(
            &self,
            request: tonic::Request<super::GenerateKeyPairRequest>,
        ) -> Result<tonic::Response<super::GenerateKeyPairResponse>, tonic::Status>;
        #[doc = " Hash data"]
        async fn hash_data(
            &self,
            request: tonic::Request<super::HashDataRequest>,
        ) -> Result<tonic::Response<super::HashDataResponse>, tonic::Status>;
        #[doc = " Verify hash of data"]
        async fn verify_data_hash(
            &self,
            request: tonic::Request<super::VerifyDataHashRequest>,
        ) -> Result<tonic::Response<super::super::common::SimpleResponse>, tonic::Status>;
        #[doc = " Sign a message"]
        async fn sign_message(
            &self,
            request: tonic::Request<super::SignMessageRequest>,
        ) -> Result<tonic::Response<super::SignMessageResponse>, tonic::Status>;
        #[doc = " Recover signature"]
        async fn recover_signature(
            &self,
            request: tonic::Request<super::RecoverSignatureRequest>,
        ) -> Result<tonic::Response<super::RecoverSignatureResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct KmsServiceServer<T: KmsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: KmsService> KmsServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for KmsServiceServer<T>
    where
        T: KmsService,
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
                "/kms.KmsService/GenerateKeyPair" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateKeyPairSvc<T: KmsService>(pub Arc<T>);
                    impl<T: KmsService> tonic::server::UnaryService<super::GenerateKeyPairRequest>
                        for GenerateKeyPairSvc<T>
                    {
                        type Response = super::GenerateKeyPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateKeyPairRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.generate_key_pair(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GenerateKeyPairSvc(inner);
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
                "/kms.KmsService/HashData" => {
                    #[allow(non_camel_case_types)]
                    struct HashDataSvc<T: KmsService>(pub Arc<T>);
                    impl<T: KmsService> tonic::server::UnaryService<super::HashDataRequest> for HashDataSvc<T> {
                        type Response = super::HashDataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HashDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.hash_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = HashDataSvc(inner);
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
                "/kms.KmsService/VerifyDataHash" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyDataHashSvc<T: KmsService>(pub Arc<T>);
                    impl<T: KmsService> tonic::server::UnaryService<super::VerifyDataHashRequest>
                        for VerifyDataHashSvc<T>
                    {
                        type Response = super::super::common::SimpleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyDataHashRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.verify_data_hash(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = VerifyDataHashSvc(inner);
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
                "/kms.KmsService/SignMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SignMessageSvc<T: KmsService>(pub Arc<T>);
                    impl<T: KmsService> tonic::server::UnaryService<super::SignMessageRequest> for SignMessageSvc<T> {
                        type Response = super::SignMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.sign_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SignMessageSvc(inner);
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
                "/kms.KmsService/RecoverSignature" => {
                    #[allow(non_camel_case_types)]
                    struct RecoverSignatureSvc<T: KmsService>(pub Arc<T>);
                    impl<T: KmsService> tonic::server::UnaryService<super::RecoverSignatureRequest>
                        for RecoverSignatureSvc<T>
                    {
                        type Response = super::RecoverSignatureResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecoverSignatureRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.recover_signature(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RecoverSignatureSvc(inner);
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
    impl<T: KmsService> Clone for KmsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: KmsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: KmsService> tonic::transport::NamedService for KmsServiceServer<T> {
        const NAME: &'static str = "kms.KmsService";
    }
}
