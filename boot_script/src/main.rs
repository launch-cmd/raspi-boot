use crate::opt::Opt;
use cmd_lib::{
    cmd_manager::CommandManager, cmd_wrapper::CmdWrapper, json::command_file::CommandFile,
};
use structopt::StructOpt;
const COMMAND_JSON: &str = include_str!("../commands.json");

mod opt;

fn main() {
    let opt = Opt::from_args();
    let command_file: CommandFile = serde_json::from_str(&COMMAND_JSON).unwrap();
    let mut commands: Vec<CmdWrapper> = Vec::new();

    substitute_cmd_args(&opt, command_file, &mut commands);

    let mut manager = CommandManager::new(commands);
    manager.execute_all(opt.verbose);
}

fn substitute_cmd_args(opt: &Opt, cmd_file: CommandFile, commands: &mut Vec<CmdWrapper>) {
    cmd_file.commands.into_iter().for_each(|mut cmd| {
        //change occurences of the arguments in the COMMAND_JSON string
        let mut new_cmd = str::replace(cmd.command.as_str(), "$CLIENT_MAC", &opt.mac);
        new_cmd = str::replace(new_cmd.as_str(), "$WIFI_REGION", &opt.wifi_region);
        new_cmd = str::replace(new_cmd.as_str(), "$WIFI_SSID", &opt.ssid);
        new_cmd = str::replace(new_cmd.as_str(), "$WIFI_PASSWORD", &opt.password);
        new_cmd = str::replace(new_cmd.as_str(), "$SERVER_HOSTNAME", &opt.server_hostname);
        new_cmd = str::replace(new_cmd.as_str(), "$CLIENT_HOSTNAME", &opt.client_hostname);
        new_cmd = str::replace(new_cmd.as_str(), "$SERVER_IP", &opt.server_ip);
        new_cmd = str::replace(new_cmd.as_str(), "$CLIENT_IP", &opt.client_ip);
        cmd.command = new_cmd;
        //create CmdWrappers for all of the commands in commands.json
        commands.push(CmdWrapper::new(cmd))
    });
}
