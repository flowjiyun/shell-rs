use std::{collections::HashSet, sync::LazyLock};

pub mod bt_exit;
pub mod bt_echo;
pub mod bt_type;

static BUILTIN_SET: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut builtin_set: HashSet<String> = HashSet::new();
    builtin_set.insert("exit".to_string());
    builtin_set.insert("echo".to_string());
    builtin_set.insert("type".to_string());
    builtin_set
});
