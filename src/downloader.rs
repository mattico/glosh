use std::sync::Arc;
use std::thread::{JoinHandle, spawn};
use std::sync::mpsc::{SyncSender};
use config::Config;
use range::Range;

use aws_sdk_rust::aws::common::credentials::{ParametersProvider, DefaultCredentialsProvider};
use aws_sdk_rust::aws::s3::endpoint::{Endpoint,Signature};
use aws_sdk_rust::aws::s3::s3client::S3Client;
use aws_sdk_rust::aws::s3::object::GetObjectRequest;

pub fn spawn_downloader(config: Arc<Config>, data_tx: SyncSender<u8>, key: String) -> JoinHandle<()> {
  let config = config.clone();
  spawn(move || {
    println!("Starting downloader");
    let parameters = ParametersProvider::with_parameters(
      config.aws_access_key_id.clone(),
      config.aws_secret_key.clone(),
      None
    ).unwrap();
    let mut range = Range::default();

    let provider = DefaultCredentialsProvider::new(Some(parameters)).expect("Failed to create AWS Provider");
    let endpoint = Endpoint::new(config.s3_region, Signature::V4, None, None, None, None);
    let client = S3Client::new(provider, endpoint);
    println!("S3 client is ready");
    loop {
      let step = range.step();
      println!("Started work on chunk: {}", step.clone());
      let mut request = GetObjectRequest::default();
      request.bucket = config.s3_bucket.clone();
      request.key = key.clone().into();
      request.range = Some(step.clone());
      println!("Downloading file {}", key.clone());
      let resp_result = client.get_object(&request, None);
      let resp = resp_result.expect("Failed to download chunk");
      println!("Downloaded chunk: {}", step.clone());
      let mut body: Vec<u8> = resp.body.clone();
      body.reverse();
      println!("Feeding bytes from chunk: {}", step.clone());
      while let Some(byte) = body.pop() {
        data_tx.send(byte).expect("failed to push bytes");
      }
      if resp.body.len() < range.step {
        break;
      }
    }
  })
}
