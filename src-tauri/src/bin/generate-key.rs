use libp2p::identity::ed25519::Keypair;
use std::io::{self, Write};
fn main() -> io::Result<()> {
    let raw_key = Keypair::generate().encode();
    io::stdout().write_all(&raw_key)?;
    Ok(())
}
