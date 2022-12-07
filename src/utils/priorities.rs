pub fn priority(x: &char) -> u32 {
    let p = if x.is_ascii_lowercase() {
        (*x as u32) % 96
    } else {
        (*x as u32) % 64 + 26
    };
    log::debug!("Char {} has priority {}",x,p);
    p
}