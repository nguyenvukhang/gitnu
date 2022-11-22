macro_rules! test {
    ($name:ident, $fun:expr) => {
        gitnu_test!($name, $fun, "--porcelain");
    };
}

test!(many_files, |mut t: Test| {
    use crate::data::LONG_EXPECT_SHORT_FLAG;
    t.shell(
        "",
        &(1..1000)
            .map(|v| format!(" {:0width$}", v, width = 5))
            .fold(String::from("touch"), |a, v| a + &v),
    )
    .gitnu("", "init")
    .gitnu("", "status")
    .gitnu("", "add 69-420")
    .gitnu("", "status")
    .expect_stderr("")
    .expect_stdout(LONG_EXPECT_SHORT_FLAG);
});
