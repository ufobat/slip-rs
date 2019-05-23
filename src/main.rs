fn main() {
    let data: Vec<u8> = gen_random_data();
    let packet = encode_data(&data);
    let data2 = decode_packet(&packet);
    println!("vec_compare: {} ", vec_compare(&data, &data2));
}

fn vec_compare(va: &[u8], vb: &[u8]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
        .all(|(a,b)| a == b)
}

fn gen_random_data() -> Vec<u8> {
    let random_bytes: Vec<u8> = (0..1024).map(|_| { rand::random::<u8>() }).collect();
    return random_bytes
}


fn encode_data(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.push(0xC0);
    for byte in data.iter() {
        match byte {
            0xC0 => { result.push(0xDB); result.push(0xDC) },
            0xDB => { result.push(0xDB); result.push(0xDD) },
            x    => result.push((*x).clone()),
        }
    }
    result.push(0xC0);
    return result;
}

fn decode_packet(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut packet_started  = true;  // because we could start with random data, in the middle of the converstation
    let mut sob             = false; // skip one byte
    for (i, byte) in data.iter().enumerate() {
        match (byte, sob) {
            (0xC0, false)                          => packet_started = !packet_started,
            (0xDB, false) if peek(data, i+1, 0xDC) => if !packet_started { sob = true; result.push(0xC0) },
            (0xDB, false) if peek(data, i+1, 0xDD) => if !packet_started { sob = true; result.push(0xDB) },
            (0xDB, false)                          => panic!("data wrong: {} - {}", byte, sob),
            (_   , false)                          => result.push(*byte),
            (_   , true )                          => sob = false,
        }
    }
    return result;
}

fn peek(data: &Vec<u8>, pos: usize, expectation: u8) -> bool {
    data.len() >= pos && data[pos] == expectation
}
