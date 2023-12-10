use orx_closure::Capture;
use orx_funvec::*;

#[derive(derive_new::new)]
struct Person {
    id: usize,
    name: String,
}

#[test]
fn at() {
    let fun = Capture(42).fun_option_ref(|val, _: usize| Some(val));
    assert_eq!(Some(&42), fun.ref_at(13));

    let fun = Capture(vec![1, 2, 3]).fun_option_ref(|vec, i: usize| vec.get(i));
    assert_eq!(Some(&2), fun.ref_at(1));
    assert_eq!(None, fun.ref_at(3));

    let people = [
        Person::new(42, "john".to_string()),
        Person::new(3, "doe".to_string()),
    ];

    let fun = Capture(&people)
        .fun_option_ref(|ppl, id: usize| ppl.iter().find(|p| p.id == id).map(|p| p.name.as_str()));
    assert_eq!(Some("doe"), fun.ref_at(3));
    assert_eq!(None, fun.ref_at(7));

    let fun = Capture(people)
        .fun_option_ref(|ppl, id: usize| ppl.iter().find(|p| p.id == id).map(|p| p.name.as_str()));
    assert_eq!(Some("john"), fun.ref_at(42));
    assert_eq!(None, fun.ref_at(1));
}

#[test]
fn iter() {
    let fun = Capture(1).fun_option_ref(|val, _: usize| Some(val));
    assert_eq!(12, fun.ref_iter_over(0..12).flatten().sum());
    assert_eq!(12, fun.ref_iter_over(100..112).flatten().sum());

    let fun = Capture(vec![1, 2, 3]).fun_option_ref(|vec, i: usize| vec.get(i));
    assert_eq!(6, fun.ref_iter_over(0..3).flatten().sum());
    assert_eq!(5, fun.ref_iter_over(1..4).flatten().sum());

    let people = [
        Person::new(42, "john".to_string()),
        Person::new(3, "doe".to_string()),
    ];
    let ids_to_test = [3, 12, 42];

    let fun = Capture(&people)
        .fun_option_ref(|ppl, id: usize| ppl.iter().find(|p| p.id == id).map(|p| p.name.as_str()));
    let result: Vec<_> = fun.ref_iter_over(ids_to_test.iter().cloned()).collect();
    assert_eq!(vec![Some("doe"), None, Some("john")], result);

    let fun = Capture(people)
        .fun_option_ref(|ppl, id: usize| ppl.iter().find(|p| p.id == id).map(|p| p.name.as_str()));
    let result: Vec<_> = fun.ref_iter_over(ids_to_test.into_iter()).collect();
    assert_eq!(vec![Some("doe"), None, Some("john")], result);
}
