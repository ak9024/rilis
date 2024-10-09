use log::info;
use ssh2::Session;
use std::io::Read;

pub fn command(sess: &Session, command: &str) -> String {
    info!("{}", command);

    let mut channel = sess.channel_session().unwrap();

    channel.exec(command).unwrap();

    let mut output = String::new();
    let mut error = String::new();

    channel.read_to_string(&mut output).unwrap();
    channel.stderr().read_to_string(&mut error).unwrap();

    channel.wait_close().unwrap();

    if !output.is_empty() {
        output
    } else {
        error
    }
}
