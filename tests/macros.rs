#[macro_export]
macro_rules! gitnu_test {
    ($name:ident, $fun:expr, $status_flag:expr) => {
        #[test]
        fn $name() {
            use crate::test::{test, Test};
            fn f() {}
            fn type_name_of<'a, T>(_: T) -> &'a str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            // pop off the "::f" behind
            $fun(test(&name[..name.len() - 3], $status_flag));
        }
    };
    ($name:ident, $fun:expr) => {
        #[test]
        fn $name() {
            use crate::test::{test, Test};
            fn f() {}
            fn type_name_of<'a, T>(_: T) -> &'a str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            // pop off the "::f" behind
            $fun(test(&name[..name.len() - 3], ""));
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
