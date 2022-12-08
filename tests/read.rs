use crate::status;

const EVERY_STATE: &str = "\
On branch main
Changes to be committed:
1	new file:   A
2	modified:   B
3	deleted:    C
4	typechange: D
5	renamed:    E -> _E

Changes not staged for commit:
6	modified:   G
7	typechange: H
8	deleted:    I

Untracked files:
9	F
10	_I

";

#[macro_export]
macro_rules! test {
    ($name:ident, $fun:expr) => {
        gitnu_test!($name, |mut t: Test| {
            t.gitnu("", "init");
            t.shell("", "touch A B C D E F G H I");
            "A B C D E F G H I".split(' ').for_each(|v| {
                t.write_file(v, v);
            });
            t.gitnu("", "add B C D E G H I").gitnu("", "commit -m pre");
            t.write_file("B", "B_").write_file("G", "G_").remove("C");
            t.rename("E", "_E").rename("I", "_I").shell("", "ln -sf . D");
            t.shell("", "ln -sf . H").gitnu("", "add A B C D E _E");
            $fun(t)
        });
    };
}

test!(base, |mut t: Test| {
    status::normal!(t, EVERY_STATE);
    status::short!(
        t,
        lines!(
            "1  A  A",
            "2  M  B",
            "3  D  C",
            "4  T  D",
            "5   M G",
            "6   T H",
            "7   D I",
            "8  R  E -> _E",
            "9  ?? F",
            "10 ?? _I",
            ""
        )
    );
});

test!(cache_state, |mut t: Test| {
    let mut cache =
        t.mock_cache(vec!["A", "B", "C", "D", "_E", "G", "H", "I", "F", "_I"]);
    cache.push('\n');
    t.gitnu("", "status");
    t.assert("", "cat .git/gitnu.txt", &cache);
    t.mark_as_checked();
});

test!(renamed_add, |mut t: Test| {
    t.gitnu("", "status").gitnu("", "add 8 10");
    status::normal!(
        t,
        lines!(
            "On branch main",
            "Changes to be committed:",
            "1	new file:   A",
            "2	modified:   B",
            "3	deleted:    C",
            "4	typechange: D",
            "5	renamed:    E -> _E",
            "6	renamed:    I -> _I",
            "",
            "Changes not staged for commit:",
            "7	modified:   G",
            "8	typechange: H",
            "",
            "Untracked files:",
            "9	F",
            "",
            ""
        )
    );
    status::short!(
        t,
        lines!(
            "1  A  A",
            "2  M  B",
            "3  D  C",
            "4  T  D",
            "5   M G",
            "6   T H",
            "7  R  E -> _E",
            "8  R  I -> _I",
            "9  ?? F",
            ""
        )
    );
});

// git status shows both filenames of the rename (before and after),
// gitnu picks the second one to replace the number because it's the
// one that still exists as a file
test!(renamed_reset, |mut t: Test| {
    t.gitnu("", "status").gitnu("", "reset 5");
    status::normal!(
        t,
        lines!(
            "On branch main",
            "Changes to be committed:",
            "1	new file:   A",
            "2	modified:   B",
            "3	deleted:    C",
            "4	typechange: D",
            "5	deleted:    E",
            "",
            "Changes not staged for commit:",
            "6	modified:   G",
            "7	typechange: H",
            "8	deleted:    I",
            "",
            "Untracked files:",
            "9	F",
            "10	_E",
            "11	_I",
            "",
            ""
        )
    );
    status::short!(
        t,
        lines!(
            "1  A  A", "2  M  B", "3  D  C", "4  T  D", "5  D  E", "6   M G",
            "7   T H", "8   D I", "9  ?? F", "10 ?? _E", "11 ?? _I", ""
        )
    );
});
