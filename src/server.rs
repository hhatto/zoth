use std::thread::sleep;
use std::time::Duration;
use std::io::*;
use std::process::{Command, Stdio};
use std::sync::Arc;
use futures::Future;
use grpcio::{EnvBuilder, RpcContext, ServerBuilder, UnarySink, Server};

use crate::zothcore::*;
use crate::zothcore_grpc::{ZothCore, create_zoth_core};
use crate::cache::{ZothCache, create_cache_root_dir};
use chan_signal::{kill_this, Signal};


#[derive(Clone)]
struct ZothService;

impl ZothCore for ZothService {
    fn exec(&self, ctx: RpcContext, req: ExecRequest, sink: UnarySink<ExecResponse>) {
        create_cache_root_dir();

        let command = req.get_command();
        let args = req.get_args();
        let input = req.get_input();
        let mut resp = ExecResponse::new();

        let mut cache_info = ZothCache::new(format!("{}{}", command, args).as_str());
        if cache_info.exist_cache(input) {
            // cache hit!!
            resp.set_result(cache_info.get_output().unwrap());
        } else {
            // not hit cache, execute command
            let mut cmdproc = Command::new(command);
            if !args.is_empty() {
                let vec_args: Vec<&str> = args.split(' ').collect();
                cmdproc.args(&vec_args);
            }
            let mut process = cmdproc
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn().expect("spawn fail");
            {
                let cmd_in = process.stdin.as_mut().unwrap();
                match cmd_in.write_all(input.as_bytes()) {
                    Err(e) => panic!("piped error: {}", e),
                    Ok(_) => {
                    },
                }
            }

            let proc_out = process.wait_with_output().expect("failed to wait on child proc");
            let output = String::from_utf8(proc_out.stdout).expect("get string error");

            if !proc_out.status.success() {
                println!("process error. exit code: {:?}", proc_out.status.code());
                return;
            }
            cache_info.set_cache(input, output.as_str());
            resp.set_result(output);
        }

        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply: {:?}", e));
        ctx.spawn(f)
    }

    fn clear_cache(&self, ctx: RpcContext, req: ClearCacheRequest, sink: UnarySink<ClearCacheResponse>) {
        let command = req.get_command();
        let args = req.get_args();
        let is_all = req.get_all();
        let resp = ClearCacheResponse::new();

        let cache_info = ZothCache::new(format!("{}{}", command, args).as_str());
        cache_info.clear_cache(is_all);
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply: {:?}", e));
        ctx.spawn(f)
    }

    fn health_check(&self, ctx: RpcContext, req: HealthCheckRequest, sink: UnarySink<HealthCheckResponse>) {
        let mut resp = HealthCheckResponse::new();
        resp.set_status(ServerStatus::ALIVE);
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn shutdown(&self, ctx: RpcContext, req: ShutdownRequest, sink: UnarySink<ShutdownResponse>) {
        println!("shutdonw!!");
        let msec = req.get_msec_after();
        sleep(Duration::from_millis(msec as u64));
        let resp = ShutdownResponse::new();
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        kill_this(Signal::TERM);
        ctx.spawn(f)
    }
}

pub fn start_server() -> Server {
    let env = Arc::new(EnvBuilder::new().build());
    let core_service = create_zoth_core(ZothService);
    let mut server = ServerBuilder::new(env)
        .register_service(core_service)
        .bind("127.0.0.1", 50051)
        .build()
        .expect("fail build server");
    server.start();
    server
}

