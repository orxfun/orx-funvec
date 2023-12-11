use orx_closure::Capture;
use orx_funvec::*;

#[derive(derive_new::new)]
struct Person {
    id: usize,
    age: u32,
}

#[test]
fn at() {
    let fun = Capture(42).fun(|val, _: usize| Some(*val));
    assert_eq!(Some(42), fun.at(13));

    let fun = Capture(vec![1, 2, 3]).fun(|vec, i: usize| vec.get(i).copied());
    assert_eq!(Some(2), fun.at(1));
    assert_eq!(None, fun.at(3));

    let people = [Person::new(42, 74), Person::new(3, 19)];

    let fun = Capture(&people).fun(|ppl, id: usize| ppl.iter().find(|p| p.id == id).map(|p| p.age));
    assert_eq!(Some(19), fun.at(3));
    assert_eq!(None, fun.at(7));

    let fun = Capture(people).fun(|ppl, id: usize| ppl.iter().find(|p| p.id == id).map(|p| p.age));
    assert_eq!(Some(74), fun.at(42));
    assert_eq!(None, fun.at(1));
}

#[test]
fn iter() {
    let fun = Capture(1).fun(|val, _: usize| Some(*val));
    assert_eq!(12, fun.iter_over(0..12).flatten().sum());
    assert_eq!(12, fun.iter_over(100..112).flatten().sum());

    let fun = Capture(vec![1, 2, 3]).fun(|vec, i: usize| vec.get(i).copied());
    assert_eq!(6, fun.iter_over(0..3).flatten().sum());
    assert_eq!(5, fun.iter_over(1..4).flatten().sum());
}
