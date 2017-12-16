#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate protobuf;
extern crate grpcio;
extern crate futures;
extern crate crypto;
extern crate chan_signal;

mod cli;
mod zothcore;
mod zothcore_grpc;
mod server;
mod cache;

use std::thread;
use std::time::Duration;
use std::io::*;
use std::sync::Arc;
use chan_signal::Signal;
use grpcio::{Environment, ChannelBuilder, CallOption};
use futures::Future;
use zothcore::{ExecRequest, HealthCheckRequest, ShutdownRequest, ClearCacheRequest};
use zothcore_grpc::ZothCoreClient;

fn start_zothd() {
    debug!("start zothd with thread");
    thread::spawn(|| {
        let sig = chan_signal::notify(&[Signal::TERM, Signal::USR1]);
        let mut sv = server::start_server();
        for &(ref host, port) in sv.bind_addrs() {
            info!("listening on {}:{}", host, port);
        }

        match sig.recv() {
            Some(_) => info!("bye"),
            _ => { println!("bb"); }
        }
        let _ = sv.shutdown().wait();
    });
}

fn check_alive_zothd(client: &ZothCoreClient) -> bool {
    let req = HealthCheckRequest::new();
    let option = CallOption::default().timeout(Duration::from_millis(500));
    match client.health_check_opt(req, option) {
        Ok(reply) => {
            debug!("health check ok: {:?}", reply);
            true
        },
        Err(e) => {
            info!("health check error: {}", e);
            false
        }
    }
}

fn dispatch_exec_subcommand(command_name: &str, args: &str, client: ZothCoreClient) {
    let s = stdin();
    let mut cmd_input = String::new();
    let _ = s.lock().read_to_string(&mut cmd_input);

    let mut req = ExecRequest::new();
    req.set_command(command_name.to_owned());
    req.set_args(args.to_string());
    req.set_input(cmd_input);
    let reply = client.exec(req).expect("fail rpc request");
    println!("{}", reply.get_result());
}

fn dispatch_clear_subcommand(command_name: &str, args: &str, client: ZothCoreClient, is_all: bool) {
    let mut req = ClearCacheRequest::new();
    req.set_command(command_name.to_owned());
    req.set_args(args.to_string());
    req.set_all(is_all);
    let _ = client.clear_cache(req).expect("fail rpc request");
}

fn main() {
    let _ = env_logger::init();

    let app = cli::build_zothcl();
    let matches = app.get_matches();

    let env = Arc::new(Environment::new(1));
    let mut ch = ChannelBuilder::new(env.clone()).connect("127.0.0.1:50051");
    let mut core_client = ZothCoreClient::new(ch);

    if !check_alive_zothd(&core_client) {
        start_zothd();
        // re-connect
        ch = ChannelBuilder::new(env.clone()).connect("127.0.0.1:50051");
        core_client = ZothCoreClient::new(ch);
        std::thread::sleep(Duration::from_millis(500));
    }

    debug!("execute subcommand");

    match matches.subcommand_name() {
        Some("exec") => {
            let sub_matches = matches.subcommand_matches("exec");
            let input_cmd = sub_matches.unwrap().values_of("cmd");
            if input_cmd.is_none() {
                println!("usage: {}", matches.usage());
                return;
            }
            let cmd = input_cmd.map(|vals| vals.collect::<Vec<_>>()).unwrap();
            let command_name = cmd[0];
            let args = cmd[1..].join(" ");
            dispatch_exec_subcommand(command_name, args.as_str(), core_client);
        },
        Some("shutdown") => {
            let mut req = ShutdownRequest::new();
            req.set_msec_after(0);
            let option = CallOption::default().timeout(Duration::from_millis(500));
            let _ = core_client.shutdown_opt(req, option);
        },
        Some("clear") => {
            let is_all = false;
            let sub_matches = matches.subcommand_matches("clear");
            let input_cmd = sub_matches.unwrap().values_of("cmd");
            if input_cmd.is_none() {
                println!("usage: {}", matches.usage());
                return;
            }
            let cmd = input_cmd.map(|vals| vals.collect::<Vec<_>>()).unwrap();
            let command_name = cmd[0];
            let args = cmd[1..].join(" ");
            dispatch_clear_subcommand(command_name, args.as_str(), core_client, is_all);
        },
        _ => {
            println!("{}", matches.usage());
            return;
        }
    };

}
