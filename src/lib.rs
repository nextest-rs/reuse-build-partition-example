// SPDX-License-Identifier: CC0

//! This is an example showing how to reuse builds and partition test runs in nextest.
//!
//! The example contains 32 tests that each take 20 seconds.

macro_rules! define_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    };
}

define_test!(test_sleep0);
define_test!(test_sleep1);
define_test!(test_sleep2);
define_test!(test_sleep3);
define_test!(test_sleep4);
define_test!(test_sleep5);
define_test!(test_sleep6);
define_test!(test_sleep7);
define_test!(test_sleep8);
define_test!(test_sleep9);
define_test!(test_sleep10);
define_test!(test_sleep11);
define_test!(test_sleep12);
define_test!(test_sleep13);
define_test!(test_sleep14);
define_test!(test_sleep15);
define_test!(test_sleep16);
define_test!(test_sleep17);
define_test!(test_sleep18);
define_test!(test_sleep19);
define_test!(test_sleep20);
define_test!(test_sleep21);
define_test!(test_sleep22);
define_test!(test_sleep23);
define_test!(test_sleep24);
define_test!(test_sleep25);
define_test!(test_sleep26);
define_test!(test_sleep27);
define_test!(test_sleep28);
define_test!(test_sleep29);
define_test!(test_sleep30);
define_test!(test_sleep31);
