# Shared Nickel contracts for repository-owned policy and artifact data.
# Runtime Rust remains responsible for filesystem scanning and command execution.

let non_empty_string_contract = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in

let non_empty_array_contract = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in

let non_negative_number_contract = std.contract.from_predicate (fun value =>
  std.is_number value && value >= 0
) in

let positive_number_contract = std.contract.from_predicate (fun value =>
  std.is_number value && value > 0
) in

let boolean_contract = std.contract.from_predicate (fun value =>
  std.is_bool value
) in

let stable_id_contract = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[A-Za-z0-9_.-]+$" value
) in

let safe_relative_path_contract = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && !std.string.is_match "^/" value
  && !std.string.is_match "(^|/)\\.\\.(/|$)" value
  && !std.string.is_match "\\\\" value
) in

let blake3_hex_contract = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-fA-F]{64}$" value
) in

let blake3_digest_text_contract = std.contract.from_predicate (fun value =>
  std.is_string value && (
    std.string.is_match "^[0-9a-fA-F]{64}$" value
    || std.string.is_match "^b3:[0-9a-fA-F]{64}$" value
  )
) in

let short_blake3_digest_text_contract = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^b3:[0-9a-fA-F]{16,64}$" value
) in

let contains_value = fun values value =>
  std.array.any (fun candidate => candidate == value) values in

let count_value = fun values value =>
  std.array.length (std.array.filter (fun candidate => candidate == value) values) in

let unique_values = fun values =>
  std.array.all (fun value => count_value values value == 1) values in

let all_values_in = fun values allowed =>
  std.array.all (fun value => contains_value allowed value) values in

let entry_ids = fun entries =>
  std.array.map (fun entry => entry.id) entries in

let entry_ids_unique = fun entries =>
  unique_values (entry_ids entries) in

let enum_contract = fun allowed => std.contract.from_predicate (fun value =>
  std.is_string value && contains_value allowed value
) in

{
  NonEmptyString = non_empty_string_contract,
  NonEmptyArray = non_empty_array_contract,
  NonNegativeNumber = non_negative_number_contract,
  PositiveNumber = positive_number_contract,
  Boolean = boolean_contract,
  StableId = stable_id_contract,
  SafeRelativePath = safe_relative_path_contract,
  Blake3Hex = blake3_hex_contract,
  Blake3DigestText = blake3_digest_text_contract,
  ShortBlake3DigestText = short_blake3_digest_text_contract,
  contains = contains_value,
  count = count_value,
  unique = unique_values,
  all_in = all_values_in,
  ids = entry_ids,
  ids_unique = entry_ids_unique,
  enum = enum_contract,
}
