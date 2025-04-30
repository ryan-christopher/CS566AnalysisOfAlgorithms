use std::time::Instant;
use rand;
use ml_kem::*;
use kem::{Decapsulate, Encapsulate};
use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Key};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::fs;
use std::fs::read_to_string;
use std::io::Write;

fn kyber(message: String) -> String {
    let mut results: String = "".to_owned();
    // timestamps for total_time and stopwatch
    let total_time = Instant::now();
    let mut stopwatch = Instant::now();

    // instance of random number generator
    let mut rng = rand::thread_rng();

    // encapsulate = public key, decapsulate = private key
    let (dk, ek) = MlKem768::generate(&mut rng);
    results.push_str(&format!("Key generation time - {:?}\n", stopwatch.elapsed()));
    stopwatch = Instant::now();

    // encapsulate shared key to the holder of the decapsulation key, receive the shared secret 'k_send'
    // and the encapsulated form 'ct'
    let (ct, k_send) = ek.encapsulate(&mut rng).unwrap();
    results.push_str(&format!("Encapsulation time - {:?}\n", stopwatch.elapsed()));
    stopwatch = Instant::now();

    // decapsulate the shared key
    let k_recv = dk.decapsulate(&ct).unwrap();
    results.push_str(&format!("Decapsulation time - {:?}\n", stopwatch.elapsed()));

    // verify shared keys are identical
    assert_eq!(k_send, k_recv);
    stopwatch = Instant::now();

    // take symmetric key generated from kyber, pass as GenericArray
    let key = Key::<Aes256Gcm>::from_slice(&k_send);

    // create cipher from key
    let cipher = Aes256Gcm::new(&key);
    results.push_str(&format!("Key and cipher generation time - {:?}\n", stopwatch.elapsed()));
    stopwatch = Instant::now();

    // initialize nonce/IV to be used in combination with encryption key 
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // encrypt message
    let ciphertext = cipher.encrypt(&nonce, message.as_bytes().as_ref()).unwrap();
    results.push_str(&format!("Encryption time - {:?}\n", stopwatch.elapsed()));
    stopwatch = Instant::now();

    // decrypt message
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    results.push_str(&format!("Decryption time - {:?}\n", stopwatch.elapsed()));

    // verify initial message and decrypted message are the same
    assert_eq!(&plaintext, message.as_bytes());
    results.push_str(&format!("Total time - {:?}\n", total_time.elapsed()));

    results
}


fn rsa(message: String) -> String {
    let mut results: String = "".to_owned();
    // timestamps for total_time and stopwatch
    let total_time = Instant::now();
    let mut stopwatch = Instant::now();

    // instance of random number generator
    let mut rng = rand::thread_rng();

    // set bit size (therefore key size will be 256 bytes)
    let bits = 2048;

    // generate private and public keys
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    results.push_str(&format!("Key generation time - {:?}\n", stopwatch.elapsed()));
    stopwatch = Instant::now();
    
    // encrypt message
    let data = message.as_bytes();
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    results.push_str(&format!("Encryption time - {:?}\n", stopwatch.elapsed()));
    stopwatch = Instant::now();

    // decrypt message
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    results.push_str(&format!("Decryption time - {:?}\n", stopwatch.elapsed()));
    
    // verify initial message and decrypted message are the same
    assert_eq!(&data[..], &dec_data[..]);
    results.push_str(&format!("Total time - {:?}\n", total_time.elapsed()));

    results
}

fn test_on_quotes(algorithm: &str){
    // get file name
    let mut stats_file_name: String = "./".to_owned();
    stats_file_name.push_str(&algorithm.to_string());
    stats_file_name.push_str("_stats.txt");

    // open file, create if not found
    let mut stats_file = fs::OpenOptions::new().append(true).create(true).open(&stats_file_name).expect("Can not open file.");

    // for each line in quotes, call either kyber or rsa dependent on what the parameter was
    for line in read_to_string("./quotes.txt").unwrap().lines() {
        // show for progress
        println!("{}", line);
        if algorithm == "kyber" {
            stats_file.write_all(line.as_bytes()).expect("Unable to read line.");
            stats_file.write_all("\n".as_bytes()).expect("Error writing new line");
            stats_file.write_all(kyber(line.to_string()).as_bytes()).expect("Error on - kyber");
        }
        else if algorithm == "rsa" {
            stats_file.write_all(line.as_bytes()).expect("Unable to read line.");
            stats_file.write_all("\n".as_bytes()).expect("Error writing new line");
            stats_file.write_all(rsa(line.to_string()).as_bytes()).expect("Error on - rsa");
        }
    }
}


fn main(){
    // println!(" ===== Kyber =====");
    // println!("{}", kyber("this thing on?".to_string()));
    // println!(" =====  RSA  =====");
    // println!("{}", rsa("this thing on?".to_string()));

    // ===== Uncomment to run tests =====
    //test_on_quotes("kyber");
    //test_on_quotes("rsa");
}