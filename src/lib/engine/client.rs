pub type data_chunk = Vec<u8>;
pub type ec_compressed = Vec<usize>;

const EC_COMPRESSED_SIZE: usize = 33;

fn u32_to_data_chunk(value: u32) -> data_chunk {
    let mut return_value: data_chunk = Vec::new();
    for x in (0..3).rev() {
        return_value.push((value >> (8 * x)) as u8);
    }
    println!("return_value in u32_to_data_chunk: {:?}\n", &return_value);

    return return_value;
}

// побитовое ИСКЛЮЧАЮЩЕЕ ИЛИ
fn xor(point1: ec_compressed, point2: ec_compressed) -> ec_compressed {
    let mut return_value: ec_compressed = Vec::new();

    for x in (0..EC_COMPRESSED_SIZE) {
        return_value[x] = point1[x] ^ point2[x];
    }

    println!("return_value in XOR: {:?}\n", &return_value);

    return return_value;
}

fn send_transaction() {}

fn receive_transaction() {}

fn create_opening_tx() -> bool {
    return true;
}

fn create_commitment_tx() -> bool {
    return true;
}

pub fn client_handler() {
    println!("Client ...");
}
