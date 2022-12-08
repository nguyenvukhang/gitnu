mod bench;
mod comparisons;
mod data;
mod test;
use bench::Bench;
use test::Test;

fn main() {
    // comparisons::dev_null_vs_stdout(100);
    // comparisons::buffered_vs_sync(100);
    git_vs_gitnu();
}

#[allow(unused)]
fn git_vs_gitnu() {
    let idle = || {
        let t = Test::new();
        t.setup(100);
        t
    };
    let mut git = Bench::new(100, idle, |t| {
        let mut proc = t.cmd("git").arg("status").spawn().ok()?;
        proc.wait().ok().map(|_| ())
    });
    let mut gitnu = Bench::new(100, idle, |t| {
        let mut proc = t.cmd("gitnu").arg("status").spawn().ok()?;
        proc.wait().ok().map(|_| ())
    });
    git.run();
    gitnu.run();
    println!("git:{}", git);
    println!("gitnu:{}", gitnu);
}
