use mapped_command::{
    Command, CommandExecutionWithStringOutputError as Error, ExecResult, MapStdoutString,
    ReturnStdoutString,
};

// Usage: `ls_command().run()`.
// "git -C ~/repos/git-number/tests/repo -c color.status=always status",
fn git_status() -> Command<Vec<String>, Error> {
    Command::new(
        "git status",
        MapStdoutString(|out| {
            let lines = out.lines().map(Into::into).collect::<Vec<_>>();
            Ok(lines)
        }),
    )
}

fn test() {
    println!("hello world");
    let data = git_status().run().expect("command failed to run.");
    for i in data {
        println!("got here");
        println!("{}", i);
    }
}
