use std::sync::Arc;
use std::thread::{JoinHandle, spawn, sleep};
use std::sync::mpsc::{Receiver};
use config::Config;
use ftp::FtpStream;
use channel_reader::ChannelReader;
use std::path::Path;
use std::time::Duration;

pub fn spawn_uploader(config: Arc<Config>, data_rx: Receiver<u8>, key: String) -> JoinHandle<()> {
  let config = config.clone();
  spawn(move || {
    println!("Starting uploader");
    sleep(Duration::from_secs(30));
    println!("The Wait is over");
    let mut ftp_stream = FtpStream::connect(&config.ftp_host).expect("Failed to connect");
    ftp_stream.login(&config.ftp_username, &config.ftp_password).expect("Failed to login");
    println!("Logged into ftp");
    let mut reader = ChannelReader::new(data_rx);
    let file_name = Path::new(&key).file_name().unwrap().to_str().unwrap().to_owned();
    ftp_stream.cwd(config.ftp_folder.as_ref()).expect("Failed To change directory");
    println!("Went into destination dir");
    ftp_stream.put(file_name.as_ref(), &mut reader).expect("failed to upload file");
    println!("Finished uploading");
  })
}