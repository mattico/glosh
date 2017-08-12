use std::sync::Arc;
use std::thread::{JoinHandle, spawn, sleep};
use config::Config;
use ftp::FtpStream;
use buffered_reader::BufferedReader;
use std::path::Path;
use std::time::Duration;
use rb::Consumer;

pub fn spawn_uploader(config: Arc<Config>, data_rx: Consumer<u8>, key: String) -> JoinHandle<()> {
  let config = config.clone();
  spawn(move || {
    println!("Starting uploader");
    sleep(Duration::from_secs(30));
    println!("The Wait is over");
    let mut ftp_stream = FtpStream::connect(&config.ftp_host).expect("Failed to connect");
    ftp_stream.login(&config.ftp_username, &config.ftp_password).expect("Failed to login");
    println!("Logged into ftp");
    let file_name = Path::new(&key).file_name().unwrap().to_str().unwrap().to_owned();
    ftp_stream.cwd(config.ftp_folder.as_ref()).expect("Failed To change directory");
    println!("Went into destination dir");
    let mut buffered_rx = BufferedReader::new(data_rx);
    ftp_stream.put(file_name.as_ref(), &mut buffered_rx).expect("failed to upload file");
    println!("Finished uploading");
  })
}