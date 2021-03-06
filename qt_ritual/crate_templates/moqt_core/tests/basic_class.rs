use moqt_core::basic_class::{inner_struct::InnerEnum, UpdateType};
use moqt_core::{BasicClass, BasicClassField};

#[test]
fn basic_class() {
    unsafe {
        let v = BasicClass::new(1);
        assert_eq!(v.foo(), 1);
        v.set_foo(5);
        assert_eq!(v.foo(), 5);

        assert_eq!(v.int_field(), 1);
        v.set_int_field(3);
        assert_eq!(v.int_field(), 3);

        assert!(v.int_pointer_field().is_null());
        let p = v.int_reference_field();
        v.set_int_pointer_field(p);
        v.set_int_field(4);
        assert_eq!(*v.int_pointer_field(), 4);

        assert_eq!(*v.int_reference_field(), 4);
        v.set_int_field(7);
        assert_eq!(*v.int_reference_field(), 7);
        *v.int_reference_field() = 8;
        assert_eq!(v.int_field(), 8);

        // TODO: set_int_reference_field should have int arg
        v.set_int_reference_field(&mut 9);
        assert_eq!(v.int_field(), 9);

        assert_eq!(v.class_field().get(), 42);
        assert_eq!(v.class_field_mut().set(43), 42);
        assert_eq!(v.class_field().get(), 43);

        let c = BasicClassField::new();
        v.set_class_field(c.as_ref());
        assert_eq!(v.class_field().get(), 42);
        drop(c);
        assert_eq!(v.class_field().get(), 42);

        assert_eq!(v.to_int(), 3);
        let converted = v.to_q_vector_of_int();
        assert_eq!(converted.count(), 1);
        assert_eq!(*converted.at(0), 7);

        v.set_ref(&11);
    }
}

#[test]
fn nested_enum() {
    let x: UpdateType = UpdateType::Add2;
    assert_eq!(x.to_int(), 1);
    assert_eq!(UpdateType::Mul3.to_int(), 2);
    assert_eq!(UpdateType::Div5.to_int(), 4);

    unsafe {
        let v = BasicClass::new(1);
        v.set_foo(1);
        v.update_foo(UpdateType::Mul3.into());
        assert_eq!(v.foo(), 3);
        v.update_foo(UpdateType::Mul3 | UpdateType::Div5);
        assert_eq!(v.foo(), 1);
    }

    let x: InnerEnum = InnerEnum::Something;
    assert_eq!(x.to_int(), 42);
}

#[test]
fn vector_getters() {
    unsafe {
        let v = BasicClass::new(2);
        let vec = v.get_vector_int();
        assert_eq!(vec.count(), 3);
        assert_eq!(*vec.at(0), 1);
        assert_eq!(*vec.at(1), 3);
        assert_eq!(*vec.at(2), 5);

        let vec2 = v.get_vector_class();
        assert_eq!(vec2.count(), 3);
        assert_eq!(vec2.at(0).get(), 2);
        assert_eq!(vec2.at(1).get(), 4);
        assert_eq!(vec2.at(2).get(), 6);
    }
}
