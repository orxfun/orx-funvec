use orx_funvec::*;

type BoxDynFn = Box<dyn Fn(usize) -> Option<i32>>;

#[derive(derive_new::new)]
struct Person {
    id: usize,
    age: i32,
}

#[test]
fn at() {
    let value = 42;
    let fun: BoxDynFn = Box::new(move |_: usize| Some(value));
    assert_eq!(Some(42), fun.val_at(13));

    let vec = vec![1, 2, 3];
    let fun: BoxDynFn = Box::new(move |i: usize| vec.get(i).copied());
    assert_eq!(Some(2), fun.val_at(1));
    assert_eq!(None, fun.val_at(3));

    let people = [Person::new(42, 74), Person::new(3, 19)];

    let fun: BoxDynFn =
        Box::new(move |id: usize| people.iter().find(|p| p.id == id).map(|p| p.age));
    assert_eq!(Some(19), fun.val_at(3));
    assert_eq!(None, fun.val_at(7));
}

#[test]
fn iter() {
    let value = 1;
    let fun: BoxDynFn = Box::new(move |_: usize| Some(value));
    assert_eq!(12, fun.val_iter_over(0..12).flatten().sum());
    assert_eq!(12, fun.val_iter_over(100..112).flatten().sum());

    let vec = vec![1, 2, 3];
    let fun: BoxDynFn = Box::new(move |i| vec.get(i).copied());
    assert_eq!(6, fun.val_iter_over(0..3).flatten().sum());
    assert_eq!(5, fun.val_iter_over(1..4).flatten().sum());
}
