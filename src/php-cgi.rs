use std::process::Command;

pub struct PhpCgi {
    pub script: String,
    pub script_name: String,
    pub script_filename: String,
}
