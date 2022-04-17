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

define_test!(test_sleep_0);
define_test!(test_sleep_1);
define_test!(test_sleep_2);
define_test!(test_sleep_3);
define_test!(test_sleep_4);
define_test!(test_sleep_5);
define_test!(test_sleep_6);
define_test!(test_sleep_7);
define_test!(test_sleep_8);
define_test!(test_sleep_9);
define_test!(test_sleep_10);
define_test!(test_sleep_11);
define_test!(test_sleep_12);
define_test!(test_sleep_13);
define_test!(test_sleep_14);
define_test!(test_sleep_15);
define_test!(test_sleep_16);
define_test!(test_sleep_17);
define_test!(test_sleep_18);
define_test!(test_sleep_19);
define_test!(test_sleep_20);
define_test!(test_sleep_21);
define_test!(test_sleep_22);
define_test!(test_sleep_23);
define_test!(test_sleep_24);
define_test!(test_sleep_25);
define_test!(test_sleep_26);
define_test!(test_sleep_27);
define_test!(test_sleep_28);
define_test!(test_sleep_29);
define_test!(test_sleep_30);
define_test!(test_sleep_31);
