use std::env;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use std::fs::File;
use std::io::*;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

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
pub struct ZothCache {
    cmd_key: String,            // sha256 for cmd
    output: Option<String>,
}

impl ZothCache {
    pub fn new(cmdline: &str) -> ZothCache {
        let mut hasher = Sha256::new();
        hasher.input(cmdline.as_bytes());
        let cmd_key = hasher.result_str();

        ZothCache {
            cmd_key: cmd_key,
            output: None,
        }
    }

    pub fn exist_cache(&mut self, input: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.input(input.as_bytes());
        let input_key = hasher.result_str();

        let cache_root = get_cache_root();
        let cache_cmd_dir = Path::new(cache_root.as_str())
            .join(self.cmd_key.clone());
        if !cache_cmd_dir.exists() {
            create_dir(cache_cmd_dir).expect("failed to create cmd cache directory");
            return false;
        }

        let cache_path = cache_cmd_dir.join(input_key);
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

    pub fn set_cache(&mut self, input: &str, output: &str) {
        let mut hasher = Sha256::new();
        hasher.input(input.as_bytes());
        let input_key = hasher.result_str();

        let cache_root = get_cache_root();
        let cache_path = Path::new(cache_root.as_str())
            .join(self.cmd_key.clone())
            .join(input_key);
        let mut f = BufWriter::new(
                File::create(cache_path.to_str().unwrap()).expect("file open error"));
        let _ = f.write_all(output.as_bytes());
    }

    pub fn clear_cache(&self, is_all: bool) {
        let cache_root_dir = get_cache_root();
        let target_path = if is_all {
            Path::new(get_cache_root().as_str()).to_path_buf()
        } else {
            Path::new(cache_root_dir.as_str()).join(self.cmd_key.clone())
        };
        match remove_dir_all(target_path.to_str().expect("path to_str() error")) {
            Ok(_) => {},
            Err(e) => error!("remove_dir_all() error: {}", e),
        }
    }
}

pub fn get_cache_root() -> String {
    let home = dirs::home_dir().expect("fail get home dir");
    let root = format!("{}/.zoth_cache", home.to_str().unwrap());
    root
}

pub fn create_cache_root_dir() {
    // NOTE: crate directory, if not exists cache directory
    let root = get_cache_root();
    let cache_dir = Path::new(root.as_str());
    if !cache_dir.exists() {
        create_dir(cache_dir).expect("failed to create cache directory");
    }
}
