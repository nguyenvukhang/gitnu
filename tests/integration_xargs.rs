mod utils;
use utils::Git;

#[test]
fn xargs_pure() {
    let (mut git, mut gitnu) = Git::gpair("tmp/x1");
    {
        git.init();
        git.touch("gold");
        git.append("gold", "__contents__");
        gitnu.status();
    }
    let received = gitnu.xargs(&["-c", "cat", "1"]);
    assert_eq!(received, "\n__contents__");
}

#[test]
fn xargs_with_range() {
    let (mut git, mut gitnu) = Git::gpair("tmp/x2");
    {
        git.init();
        for i in 1..8 {
            let filename = format!("file_{}", i);
            git.touch(&filename);
            git.append(&filename, &format!("__contents__{}", i));
        }
        gitnu.status();
    }
    let received = gitnu.xargs(&["-c", "cat", "2-5"]);
    assert_eq!(
        received,
        "\n__contents__2\n__contents__3\n__contents__4\n__contents__5"
    );
}
