#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationMessage {
    #[prost(string, tag = "1")]
    pub device_address: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationResult {
    #[prost(uint32, tag = "1")]
    pub status_code: u32,
}
#[doc = r" Generated server implementations."]
pub mod register_device_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RegisterDeviceServer."]
    #[async_trait]
    pub trait RegisterDevice: Send + Sync + 'static {
        async fn send(
            &self,
            request: tonic::Request<super::RegistrationMessage>,
        ) -> Result<tonic::Response<super::RegistrationResult>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct RegisterDeviceServer<T: RegisterDevice> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RegisterDevice> RegisterDeviceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for RegisterDeviceServer<T>
    where
        T: RegisterDevice,
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
                "/registration.RegisterDevice/Send" => {
                    #[allow(non_camel_case_types)]
                    struct SendSvc<T: RegisterDevice>(pub Arc<T>);
                    impl<T: RegisterDevice> tonic::server::UnaryService<super::RegistrationMessage> for SendSvc<T> {
                        type Response = super::RegistrationResult;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegistrationMessage>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.send(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendSvc(inner);
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
    impl<T: RegisterDevice> Clone for RegisterDeviceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RegisterDevice> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RegisterDevice> tonic::transport::NamedService for RegisterDeviceServer<T> {
        const NAME: &'static str = "registration.RegisterDevice";
    }
}
