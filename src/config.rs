use std::net::SocketAddrV4;
use std::env::var;
use std::str::FromStr;
use regex::Regex;
use aws_sdk_rust::aws::common::region::Region;

/// Configuration that will be shared across threads. It's read-only and verified at startup.
#[derive(Clone)]
pub struct Config {
  pub aws_access_key_id: String,
  pub aws_secret_key: String,
  pub s3_bucket: String,
  pub s3_region: Region,
  pub ftp_host: SocketAddrV4,
  pub ftp_folder: String,
  pub ftp_username: String,
  pub ftp_password: String,
  pub regex: Regex
}

impl Config {
  /// Easy way to construct configration.
  pub fn new<S: Into<String>>(aws_access_key_id: S,
  aws_secret_key: S,
  s3_bucket: S,
  s3_region: S,
  ftp_host: S,
  ftp_folder: S,
  ftp_username: S,
  ftp_password: S,
  regex: S) -> Config {
    Config {
      aws_access_key_id: aws_access_key_id.into(),
      aws_secret_key: aws_secret_key.into(),
      s3_bucket: s3_bucket.into(),
      s3_region: Region::from_str(s3_region.into().as_ref()).expect("Failed to parse S3 Region"),
      ftp_host: ftp_host.into().parse().expect("Failed tp parse ftp host"),
      ftp_folder: ftp_folder.into(),
      ftp_username: ftp_username.into(),
      ftp_password: ftp_password.into(),
      regex: Regex::from_str(regex.into().as_ref()).expect("Failed to parse regex")
    }
  }
    /// Build configration from ENV
    pub fn from_env() -> Config {
      let aws_access_key_id = var("S3_ACCESS_KEY_ID").expect("Missing AWS_ACCESS_KEY_ID!");
      let aws_secret_key = var("S3_SECRET_KEY").expect("Missing AWS_SECRET_KEY");
      let s3_bucket = var("S3_BUCKET").expect("Missing S3_BUCKET");
      let s3_region = var("S3_REGION").expect("MISSING S3_REGION");
      let ftp_host = var("FTP_HOST").expect("Missing FTP_HOST");
      let ftp_folder = var("FTP_FOLDER").expect("Missing FTP_FOLDER");
      let ftp_username = var("FTP_USERNAME").expect("Missing FTP_USERNAME");
      let ftp_password = var("FTP_PASSWORD").expect("Missing FTP_PASSWORD");
      let regex = var("REGEX").expect("Missing REGEX");
      Config::new(
        aws_access_key_id,
        aws_secret_key,
        s3_bucket,
        s3_region,
        ftp_host,
        ftp_folder,
        ftp_username,
        ftp_password,
        regex
      )
    }
}