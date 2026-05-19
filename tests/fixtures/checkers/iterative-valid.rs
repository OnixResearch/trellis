fn leaf(value: u64) -> u64 {
    value
}

fn root(value: u64) -> u64 {
    leaf(value)
}

spec fn ignored_spec(value: u64) -> u64 {
    ignored_spec(value)
}

proof fn ignored_proof() {
}
