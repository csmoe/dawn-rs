pub(crate) fn parse_numeric_string(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix("0x") {
        u32::from_str_radix(hex, 16).ok()
    } else if let Some(hex) = trimmed.strip_prefix("0X") {
        u32::from_str_radix(hex, 16).ok()
    } else {
        trimmed.parse::<u32>().ok()
    }
}
pub(crate) fn parse_numeric_string_u64(value: &str) -> Option<u64> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).ok()
    } else if let Some(hex) = trimmed.strip_prefix("0X") {
        u64::from_str_radix(hex, 16).ok()
    } else {
        trimmed.parse::<u64>().ok()
    }
}
