use base64::Engine;
use base64::engine::general_purpose;

use rand::Rng;

pub fn generate_better_auth_secret() -> String {
    // generate byte array of 32 bytes
    let mut rng = rand::rng();
    let mut random_data: [u8; 32] = [0; 32];
    // fill the array with random data
    rng.fill(&mut random_data);
    // encode the array to base64
    general_purpose::STANDARD.encode(random_data)
}
