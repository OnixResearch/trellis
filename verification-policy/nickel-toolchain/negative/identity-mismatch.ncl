let expected = "nickel-lang-cli nickel 1.17.0 (rev 1320a98)" in
let actual = "nickel-lang-cli nickel 1.17.0 (rev wrong)" in
if actual == expected then actual else error "identity-mismatch"
