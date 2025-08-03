pub fn u8_slice_to_hex(slice: &[u8]) -> String {
    slice
        .to_vec()
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("")
}
