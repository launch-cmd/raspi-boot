use crate::cmd_wrapper::CmdWrapper;

/// CommandManager struct
///
/// * `commands` - vector containing all commands ```Vec<CmdWrapper>```
pub struct CommandManager {
    commands: Vec<CmdWrapper>,
}

impl CommandManager {
    /// Constructs CommandManager struct
    ///
    /// * `commands` - vector containing all commands ```Vec<CmdWrapper>```
    /// * `version` - Version of commands.json
    pub fn new(commands: Vec<CmdWrapper>) -> Self {
        CommandManager { commands }
    }

    /// Executes all commands in CommandManager
    ///
    /// * `show_verbose` - true to show verbose output of command
    pub fn execute_all(&mut self, show_verbose: bool) -> bool {
        for command in self.commands.iter_mut() {
            if !command.execute(show_verbose).success() {
                //command failed!
                return false;
            }
        }
        //all commands successfully executed
        true
    }
}
