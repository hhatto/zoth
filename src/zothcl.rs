extern crate crypto;
extern crate clap;

use std::env;
use std::fs::{create_dir, File};
use std::path::Path;
use std::io::*;
use std::process::{Command, Stdio};
use clap::{Arg, App};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// structure of cache
//   $HOME/.zoth_cache
//     - cmdkey1_sha256/
//       - cmdkey1_input1_sha256 ... into cmd output
//       - cmdkey1_input2_sha256
//             :
//     - cmdkey2_sha256/
//       - cmdkey2_input1_sha256
//           :
//
struct ZothCache {
    cmd_key: String,    // sha256 for cmd
    input_key: String,  // sha256
    output: Option<String>,
}

impl ZothCache {
    pub fn new(cmdline: &str, input: &str) -> ZothCache {
        let mut hasher = Sha256::new();
        hasher.input(cmdline.as_bytes());
        let cmd_key = hasher.result_str();

        let mut hasher = Sha256::new();
        hasher.input(input.as_bytes());
        let input_key = hasher.result_str();

        ZothCache {
            cmd_key: cmd_key,
            input_key: input_key,
            output: None,
        }
    }

    pub fn exist_cache(&mut self) -> bool {
        let cache_root = get_cache_root();
        let cache_cmd_dir = Path::new(cache_root.as_str())
            .join(self.cmd_key.clone());
        if !cache_cmd_dir.exists() {
            create_dir(cache_cmd_dir).expect("failed to create cmd cache directory");
            return false;
        }

        let cache_path = cache_cmd_dir.join(self.input_key.clone());
        if cache_path.exists() {
            // load output when hit cache data
            let mut f = BufReader::new(
                File::open(cache_path.to_str().unwrap()).expect("file open error"));
            let mut b = String::new();
            let _ = f.read_to_string(&mut b);
            self.output = Some(b);
            return true;
        }
        false
    }

    pub fn get_output(self) -> Option<String> {
        self.output
    }

    pub fn set_cache(&mut self, output: &str) {
        let cache_root = get_cache_root();
        let cache_path = Path::new(cache_root.as_str())
            .join(self.cmd_key.clone())
            .join(self.input_key.clone());
        let mut f = BufWriter::new(
                File::create(cache_path.to_str().unwrap()).expect("file open error"));
        let _ = f.write_all(output.as_bytes());
    }
}

fn get_cache_root() -> String {
    let home = env::home_dir().expect("fail get home dir");
    let root = format!("{}/.zoth_cache", home.to_str().unwrap());
    root
}

fn create_cache_root_dir() {
    // NOTE: crate directory, if not exists cache directory
    let root = get_cache_root();
    let cache_dir = Path::new(root.as_str());
    if !cache_dir.exists() {
        create_dir(cache_dir).expect("failed to create cache directory");
    }
}

fn main() {
    let matches = App::new("zothcl")
        .version(VERSION)
        .about("CLI for zothd")
        .arg(Arg::with_name("cmd")
             .multiple(true)
             .last(true))
        .get_matches();

    create_cache_root_dir();

    let cmdkey = matches.values_of("cmd");
    if cmdkey.is_none() {
        println!("{}", matches.usage());
        return;
    }

    let s = stdin();
    let mut cmd_input = String::new();
    let _ = s.lock().read_to_string(&mut cmd_input);

    let cmd = cmdkey.map(|vals| vals.collect::<Vec<_>>()).unwrap();
    let mut cache_info = ZothCache::new(cmd.join(" ").as_str(), cmd_input.as_str());
    if cache_info.exist_cache() {
        // cache hit!!
        println!("{}", cache_info.get_output().unwrap());
        return;
    }

    // not hit cache, execute command
    let cmdname = cmd[0];
    let mut cmdproc = Command::new(cmdname);
    if cmd.len() > 1 {
        cmdproc.args(&cmd[1..]);
    }
    let mut process = cmdproc
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().expect("spawn fail");
    {
        let cmd_in = process.stdin.as_mut().unwrap();
        match cmd_in.write_all(cmd_input.as_bytes()) {
            Err(e) => panic!("piped error: {}", e),
            Ok(_) => {},
        }
    }

    let proc_out = process.wait_with_output().expect("failed to wait on child proc");
    let output = String::from_utf8(proc_out.stdout).expect("get string error");

    if !proc_out.status.success() {
        println!("process error. exit code: {:?}", proc_out.status.code());
        return;
    }

    cache_info.set_cache(output.as_str());

    println!("{}", output);
}
