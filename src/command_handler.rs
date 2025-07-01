const LOGO_STR: &str = r#"
     _______________________________________________________________________________________________________________
    |  ___                  _            _   ___                                 _   _                              | 
    | |_ _|___  _ _ ._ _ _ <_>._ _  ___ | | |  _> ___ ._ _ _ ._ _ _  ___ ._ _  _| | | |   ___  ___  ___  ___  _ _   |
    |  | |/ ._>| '_>| ' ' || || ' |<_> || | | <__/ . \| ' ' || ' ' |<_> || ' |/ . | | |_ / . \/ . |/ . |/ ._>| '_>  |
    |  |_|\___.|_|  |_|_|_||_||_|_|<___||_| `___/\___/|_|_|_||_|_|_|<___||_|_|\___| |___|\___/\_. |\_. |\___.|_|    | 
    |                                                                                         <___'<___'            |
    |_______________________________________________________________________________________________________________|
                                            | Made By: Muhammad Muazen |
                                             ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾                                       
"#;

fn get_exe_file_name() -> String {

    let exe_path: std::path::PathBuf = std::env::current_exe().expect("Failed to get current executable path");
    let exe_name: &std::ffi::OsStr = exe_path.file_name().expect("Failed to get executable name");
    let str_exe_name: &str = exe_name.to_str().expect("Failed to convert executable name to string");

    return str_exe_name.to_owned();
}

// TODO
pub fn help_message() {

    let executable_file_name = get_exe_file_name();

    println!(r#"{0}

    Log all your terminal commands in a simple json file either directly by piping any command to tcl or 
    by running tcl as a background service.

    [i] Usage:

        $ {1} [options]
        $ tcl [options] cmd <command>

    [i] Options:

        -h,    --help               Print this help message
        -pl,   --print-log          Show all the logged commands from the output file
        -csh,  --config-show        Show all the setup configurations for tcl (This will print the file tcl_config.json)
        -sx,   --show-execlude      Show all the execluded commands (This will print the file execluded_commands.json)
        -svc,  --service            Run TCL as a background service (if the needed arguments are not set it will ask some questions)
        -o,    --output-file        Set the output file for tcl
        -x,    --execlude-command   Execlude a command from being logged

        //TODO
"#, LOGO_STR, executable_file_name);

}