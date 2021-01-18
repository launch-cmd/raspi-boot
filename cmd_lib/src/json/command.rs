use serde::Deserialize;
/// Command struct maps directly to commands.json
///
/// * `message` - maps to 'message' key in commands.json
/// * `version` - maps to 'version' key in commands.json
#[derive(Clone, Deserialize)]
pub struct Command {
    pub message: String,
    pub command: String,
}
