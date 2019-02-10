use moqt_core::{BasicClass, BasicClassField};
use std::ops::Deref;

#[test]
fn basic_class() {
    unsafe {
        let mut v = BasicClass::new(1);
        assert_eq!(v.foo(), 1);
        v.set_foo(5);
        assert_eq!(v.foo(), 5);

        assert_eq!(v.int_field(), 1);
        v.set_int_field(3);
        assert_eq!(v.int_field(), 3);

        assert!(v.int_pointer_field().is_null());
        let p = v.int_reference_field() as *mut i32;
        v.set_int_pointer_field(p);
        v.set_int_field(4);
        assert_eq!(v.int_pointer_field().deref(), &4);

        assert_eq!(v.int_reference_field(), &4);
        v.set_int_field(7);
        assert_eq!(v.int_reference_field(), &7);
        *v.int_reference_field() = 8;
        assert_eq!(v.int_field(), 8);

        // TODO: set_int_reference_field should have int arg
        v.set_int_reference_field(&mut 9);
        assert_eq!(v.int_field(), 9);

        assert_eq!(v.class_field().get(), 42);
        assert_eq!(v.class_field_mut().set(43), 42);
        assert_eq!(v.class_field().get(), 43);

        let c = BasicClassField::new();
        v.set_class_field(&c);
        assert_eq!(v.class_field().get(), 42);
        drop(c);
        assert_eq!(v.class_field().get(), 42);
    }
}