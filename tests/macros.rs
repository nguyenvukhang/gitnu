#[macro_export]
macro_rules! gitnu_test {
    ($name:ident, $fun:expr) => {
        #[test]
        fn $name() {
            use crate::test::Test;
            fn f() {}
            fn type_name_of<'a, T>(_: T) -> &'a str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            // pop off the "::f" behind
            $fun(Test::new(&name[..name.len() - 3]));
        }
    };
}

#[macro_export]
macro_rules! lines {
    ($( $x:expr ),*) => {{
        let mut t = Vec::new();
        $(t.push($x);)*
        &t.join("\n")
    }};
}

#[macro_export]
macro_rules! assert_eq_pretty {
    ($received:expr, $expected:expr) => {
        let expected = $expected;
        let received = $received;
        if received != expected {
            panic!(
                "
Printed outputs differ!

received:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
{received}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

expected:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
{expected}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
",
            );
        }
    };
}

pub mod status {
    macro_rules! short {
        ($test:expr, $expected:expr) => {
            $test.assert("", "gitnu status --short", $expected);
            $test.mark_status(false);
        };
        ($test:expr, $path:expr, $expected:expr) => {
            $test.assert($path, "gitnu status --short", $expected);
            $test.mark_status(false);
        };
    }

    macro_rules! normal {
        ($test:expr, $expected:expr) => {
            $test.assert("", "gitnu status", $expected);
            $test.mark_status(true);
        };
        ($test:expr, $path:expr, $expected:expr) => {
            $test.assert($path, "gitnu status", $expected);
            $test.mark_status(true);
        };
    }
    pub(crate) use {normal, short};
}
