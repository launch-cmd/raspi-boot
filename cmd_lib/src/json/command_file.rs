use super::command::Command;
use serde::Deserialize;

/// CommandFile struct maps directly to commands.json
///
/// * `commands` - Vector of Command
/// * `version` - Version of commands.json
#[derive(Clone, Deserialize)]
pub struct CommandFile {
    pub commands: Vec<Command>,
    pub version: i32,
}
