use saelient::{Id, Pgn};

pub fn main() {
    let id_builder = Id::builder()
        .sa(0x55)
        .pgn(Pgn::ProprietaryA)
        .da(0x00)
        .build()
        .unwrap();
    println!("Built id: {:x}", id_builder.as_raw());

    let id_from_raw = Id::new(418316373);
    println!(
        "Id from raw, pgn: {:?}, sa: {}, da: {:?}",
        id_from_raw.pgn(),
        id_from_raw.sa(),
        id_from_raw.da(),
    );
}
