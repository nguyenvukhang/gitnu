use crate::status;
use crate::test::{Test, TestInterface};

#[macro_export]
macro_rules! test {
    ($name:ident, $fun:expr) => {
        gitnu_test!($name, |mut t: Test| {
            setup(&mut t);
            $fun(t)
        });
    };
}

fn setup(t: &mut Test) {
    t.gitnu("", "init");
    for file in "A B C D E F G H I".split(' ') {
        t.shell("", &format!("touch {}", file));
        t.write_file(file, file);
    }
    t.gitnu("", "add B C D E G H I")
        .gitnu("", "commit -m pre")
        // modify
        .write_file("B", "B_")
        .write_file("G", "G_")
        // remove
        .remove("C")
        // rename
        .rename("E", "_E")
        .rename("I", "_I")
        // typechange
        .shell("", "ln -sf . D")
        .shell("", "ln -sf . H")
        .gitnu("", "add A B C D E _E");
}

test!(base, |mut t: Test| {
    status::normal!(
        t,
        "
---
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

"
    );
    status::short!(
        t,
        "
---
1  A  A
2  M  B
3  D  C
4  T  D
5   M G
6   T H
7   D I
8  R  E -> _E
9  ?? F
10 ?? _I
"
    );
});

test!(cache_state, |mut t: Test| {
    let test_dir = t.get_test_dir();
    let cache = cache!(
        test_dir,
        "
---
A
B
C
D
_E
G
H
I
F
_I

---
"
    );
    t.gitnu("", "status").shell("", "cat .git/gitnu.txt");
    status::general!(t, "cat .git/gitnu.txt", &cache);
});

test!(renamed_add, |mut t: Test| {
    let test_dir = t.get_test_dir();
    t.gitnu("", "status").gitnu("", "add 8 10");
    status::normal!(
        t,
        "
---
On branch main
Changes to be committed:
1	new file:   A
2	modified:   B
3	deleted:    C
4	typechange: D
5	renamed:    E -> _E
6	renamed:    I -> _I

Changes not staged for commit:
7	modified:   G
8	typechange: H

Untracked files:
9	F

"
    );
    status::short!(
        t,
        "
---
1  A  A
2  M  B
3  D  C
4  T  D
5   M G
6   T H
7  R  E -> _E
8  R  I -> _I
9  ?? F
"
    );
});
