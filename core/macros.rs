#[macro_export]
macro_rules! assert_eq {
    ($received:expr, $expected:expr) => {
        if $received != $expected {
            panic!(
                "
\x1b[0;31mReceived:\x1b[0m
{received:?}
─────────────────────────────────────────────────────────────
\x1b[0;32mExpected:\x1b[0m
{expected:?}
",
                received = $received,
                expected = $expected
            );
        }
    };
}
