use std::io::{self, Write};
use libp2p::identity::ed25519::Keypair;
fn main() -> io::Result<()> {
    let raw_key = Keypair::generate().encode();
    io::stdout().write_all(&raw_key)?;
    Ok(())
}