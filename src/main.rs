use serde::{Serialize, Deserialize};


fn main() {
    // Creating string that contains my name
    let name = String::from("Aziz Jalel");
    println!("My name is: {}", name);
    
    let name_bytes = name.as_bytes();
    println!("My name in bytes: {:?}", name_bytes);

    let hashed_name = ring::digest::digest(&ring::digest::SHA256, name_bytes);
    println!("My name hashed: {:?}", hashed_name);

    let hex_name = hex::encode(hashed_name);
    println!("My name hashed in hex: {}", hex_name);

    #[derive(Serialize, Deserialize, Debug)]
    struct NameHash {
        name: String,
        hash: String,
    }

    let Aziz_hash: NameHash = NameHash {
        name: name,
        hash: hex_name,
    };

    println!("Printing struct {:?}", Aziz_hash);

    let serialized: Vec<u8> = bincode::serialize(&Aziz_hash).unwrap();
    println!("{:?}", serialized);

    let deserialized: NameHash = bincode::deserialize(&serialized).unwrap();
    println!("{:?}", deserialized);
}
