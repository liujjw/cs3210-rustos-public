// FIXME: Make me pass! Diff budget: 2 lines.

struct Dummy;

pub trait Foo {
    fn foo(&self) -> usize {
        1
    }
}

pub trait FooToo {
    fn foob(&self) -> usize {
        2
    }
}

impl Foo for Dummy {}

impl FooToo for Dummy {}

#[test]
fn ufcs() {
    let dummy = Dummy;

    let x = dummy.foo();
    let y = dummy.foob();

    // Values for `x` and `y` must come from calling `foo()` methods.
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}
