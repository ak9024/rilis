use log::info;
use ssh2::Session;
use std::{fs::File, io::copy, path::Path};

pub fn scp(sess: &Session, local_path: &str, remote_path: &str) {
    info!("Uploading file: {} to {}", local_path, remote_path);

    let mut local_file = File::open(local_path).unwrap();
    let mut remote_file = sess
        .scp_send(
            Path::new(remote_path),
            0o644,
            local_file.metadata().unwrap().len(),
            None,
        )
        .unwrap();

    copy(&mut local_file, &mut remote_file).unwrap();
}
