use std::io;

use libp2p::identity::ed25519::Keypair;
fn main() -> io::Result<()> {
    let raw_key = Keypair::generate().encode();

    let raw_key64 = base64::encode(raw_key);
    println!("{}", raw_key64);
    Ok(())
}
