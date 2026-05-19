fn target(value: u64) -> u64 {
    value
}

fn caller(value: u64) -> u64 {
    self::target(value)
}
