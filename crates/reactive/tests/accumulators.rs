use std::mem;

use silkenweb_reactive::{
    accumulators::{SumElement, SumTotal},
    signal::{ReadSignal, SignalReceiver},
};

fn check_total(expected: usize, actual: &ReadSignal<usize>) {
    assert_eq!(expected, *actual.current());
}

#[test]
fn sum() {
    let total = SumTotal::<usize>::default();
    let first_digit = SumElement::new(&total);
    let second_digit = SumElement::new(&total);
    let third_digit = SumElement::new(&total);
    let total = total.read();

    assert_eq!(*total.current(), 0, "Intial total should be zero");

    // Test basic updates
    first_digit.receive(&1);
    check_total(1, &total);
    second_digit.receive(&10);
    check_total(11, &total);
    third_digit.receive(&100);
    check_total(111, &total);

    // Test delta updates
    first_digit.receive(&2);
    check_total(112, &total);
    second_digit.receive(&20);
    check_total(122, &total);
    third_digit.receive(&200);
    check_total(222, &total);

    // Test wrapping delta updates
    first_digit.receive(&1);
    check_total(221, &total);
    second_digit.receive(&10);
    check_total(211, &total);
    third_digit.receive(&100);
    check_total(111, &total);

    // Test dropping
    mem::drop(first_digit);
    check_total(110, &total);
    mem::drop(second_digit);
    check_total(100, &total);
    mem::drop(third_digit);
    check_total(0, &total);
}
