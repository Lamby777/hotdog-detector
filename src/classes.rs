// typedefs, structs, impls, etc.
// stuff that would take up too much space in main

pub type IDFC<T> = Result<T, Box<dyn std::error::Error>>;
