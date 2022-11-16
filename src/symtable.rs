use std::collections::hash_map::HashMap;

pub struct Symt<'a> {
    local: HashMap<&'a str, SymtInfo>,
    global: HashMap<&'a str, SymtInfo>,
}

pub enum SymtType {
    Var,
    Func { params: i32 },
}

pub struct SymtInfo {
    symt_type: SymtType,
}
