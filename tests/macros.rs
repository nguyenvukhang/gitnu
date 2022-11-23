#[macro_export]
macro_rules! gitnu_test {
    ($name:ident, $fun:expr) => {
        #[test]
        fn $name() {
            use crate::test::{Test, TestInterface};
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
macro_rules! assert_eq_pretty {
    ($expected:expr, $received:expr) => {
        let expected = &*$expected;
        let received = &*$received;
        if expected != received {
            panic!(
                "
Printed outputs differ!

expected:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
{}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

received:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
{}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
",
                expected, received
            );
        }
    };
}

pub mod status {
    macro_rules! short {
        ($test:expr, $expected:expr) => {
            crate::test::Test::assert_short(&mut $test, "", $expected);
        };
        ($test:expr, $path:expr, $expected:expr) => {
            crate::test::Test::assert_short(&mut $test, $path, $expected);
        };
    }

    macro_rules! normal {
        ($test:expr, $expected:expr) => {
            crate::test::Test::assert_normal(&mut $test, "", $expected);
        };
        ($test:expr, $path:expr, $expected:expr) => {
            crate::test::Test::assert_normal(&mut $test, $path, $expected);
        };
    }

    macro_rules! general {
        ($test:expr, $command:expr, $expected:expr) => {
            crate::test::Test::assert_general(
                &mut $test, "", $expected, $command,
            );
        };
    }

    pub(crate) use {general, normal, short};
}
