// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_ZOTH_CORE_EXEC: ::grpcio::Method<super::zothcore::ExecRequest, super::zothcore::ExecResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/zothcore.ZothCore/Exec",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ZOTH_CORE_CLEAR_CACHE: ::grpcio::Method<super::zothcore::ClearCacheRequest, super::zothcore::ClearCacheResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/zothcore.ZothCore/ClearCache",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ZOTH_CORE_HEALTH_CHECK: ::grpcio::Method<super::zothcore::HealthCheckRequest, super::zothcore::HealthCheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/zothcore.ZothCore/HealthCheck",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ZOTH_CORE_SHUTDOWN: ::grpcio::Method<super::zothcore::ShutdownRequest, super::zothcore::ShutdownResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/zothcore.ZothCore/Shutdown",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ZothCoreClient {
    client: ::grpcio::Client,
}

impl ZothCoreClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ZothCoreClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn exec_opt(&self, req: super::zothcore::ExecRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::zothcore::ExecResponse> {
        self.client.unary_call(&METHOD_ZOTH_CORE_EXEC, req, opt)
    }

    pub fn exec(&self, req: super::zothcore::ExecRequest) -> ::grpcio::Result<super::zothcore::ExecResponse> {
        self.exec_opt(req, ::grpcio::CallOption::default())
    }

    pub fn exec_async_opt(&self, req: super::zothcore::ExecRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::zothcore::ExecResponse> {
        self.client.unary_call_async(&METHOD_ZOTH_CORE_EXEC, req, opt)
    }

    pub fn exec_async(&self, req: super::zothcore::ExecRequest) -> ::grpcio::ClientUnaryReceiver<super::zothcore::ExecResponse> {
        self.exec_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn clear_cache_opt(&self, req: super::zothcore::ClearCacheRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::zothcore::ClearCacheResponse> {
        self.client.unary_call(&METHOD_ZOTH_CORE_CLEAR_CACHE, req, opt)
    }

    pub fn clear_cache(&self, req: super::zothcore::ClearCacheRequest) -> ::grpcio::Result<super::zothcore::ClearCacheResponse> {
        self.clear_cache_opt(req, ::grpcio::CallOption::default())
    }

    pub fn clear_cache_async_opt(&self, req: super::zothcore::ClearCacheRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::zothcore::ClearCacheResponse> {
        self.client.unary_call_async(&METHOD_ZOTH_CORE_CLEAR_CACHE, req, opt)
    }

    pub fn clear_cache_async(&self, req: super::zothcore::ClearCacheRequest) -> ::grpcio::ClientUnaryReceiver<super::zothcore::ClearCacheResponse> {
        self.clear_cache_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn health_check_opt(&self, req: super::zothcore::HealthCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::zothcore::HealthCheckResponse> {
        self.client.unary_call(&METHOD_ZOTH_CORE_HEALTH_CHECK, req, opt)
    }

    pub fn health_check(&self, req: super::zothcore::HealthCheckRequest) -> ::grpcio::Result<super::zothcore::HealthCheckResponse> {
        self.health_check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn health_check_async_opt(&self, req: super::zothcore::HealthCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::zothcore::HealthCheckResponse> {
        self.client.unary_call_async(&METHOD_ZOTH_CORE_HEALTH_CHECK, req, opt)
    }

    pub fn health_check_async(&self, req: super::zothcore::HealthCheckRequest) -> ::grpcio::ClientUnaryReceiver<super::zothcore::HealthCheckResponse> {
        self.health_check_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn shutdown_opt(&self, req: super::zothcore::ShutdownRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::zothcore::ShutdownResponse> {
        self.client.unary_call(&METHOD_ZOTH_CORE_SHUTDOWN, req, opt)
    }

    pub fn shutdown(&self, req: super::zothcore::ShutdownRequest) -> ::grpcio::Result<super::zothcore::ShutdownResponse> {
        self.shutdown_opt(req, ::grpcio::CallOption::default())
    }

    pub fn shutdown_async_opt(&self, req: super::zothcore::ShutdownRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::zothcore::ShutdownResponse> {
        self.client.unary_call_async(&METHOD_ZOTH_CORE_SHUTDOWN, req, opt)
    }

    pub fn shutdown_async(&self, req: super::zothcore::ShutdownRequest) -> ::grpcio::ClientUnaryReceiver<super::zothcore::ShutdownResponse> {
        self.shutdown_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ZothCore {
    fn exec(&self, ctx: ::grpcio::RpcContext, req: super::zothcore::ExecRequest, sink: ::grpcio::UnarySink<super::zothcore::ExecResponse>);
    fn clear_cache(&self, ctx: ::grpcio::RpcContext, req: super::zothcore::ClearCacheRequest, sink: ::grpcio::UnarySink<super::zothcore::ClearCacheResponse>);
    fn health_check(&self, ctx: ::grpcio::RpcContext, req: super::zothcore::HealthCheckRequest, sink: ::grpcio::UnarySink<super::zothcore::HealthCheckResponse>);
    fn shutdown(&self, ctx: ::grpcio::RpcContext, req: super::zothcore::ShutdownRequest, sink: ::grpcio::UnarySink<super::zothcore::ShutdownResponse>);
}

pub fn create_zoth_core<S: ZothCore + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ZOTH_CORE_EXEC, move |ctx, req, resp| {
        instance.exec(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ZOTH_CORE_CLEAR_CACHE, move |ctx, req, resp| {
        instance.clear_cache(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ZOTH_CORE_HEALTH_CHECK, move |ctx, req, resp| {
        instance.health_check(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ZOTH_CORE_SHUTDOWN, move |ctx, req, resp| {
        instance.shutdown(ctx, req, resp)
    });
    builder.build()
}
