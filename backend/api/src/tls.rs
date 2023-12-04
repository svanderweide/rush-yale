use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader};

pub fn load_rustls_config() -> ServerConfig {
    // load TLS files
    let cert_file =
        &mut BufReader::new(File::open("cert.pem").expect("'cert.pem' should be available"));
    let key_file =
        &mut BufReader::new(File::open("key.pem").expect("'key.pem' should be available"));

    // convert files into Rust objects
    // (to be updated when actix-web allows rustls v0.22 and rustls-pemfile v2)
    let certs = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // ERROR: empty or invalid private keys
    if keys.is_empty() {
        panic!("Could not locate private keys");
    }

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .expect("unable to configure TLS")
}
