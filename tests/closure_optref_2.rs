use orx_closure::{Capture, ClosureOptRefOneOf2};
use orx_funvec::*;

#[test]
fn at() {
    let fun1 = Capture(vec![1, 2, 3]).fun_option_ref(|vec, i: usize| vec.get(i));
    let fun2 = Capture(42).fun_option_ref(|scalar, _: usize| Some(scalar));

    let fun: ClosureOptRefOneOf2<Vec<i32>, i32, usize, i32> = fun1.into_oneof2_var1();
    assert_eq!(Some(&2), fun.ref_at(1));
    assert_eq!(None, fun.ref_at(7));

    let fun: ClosureOptRefOneOf2<Vec<i32>, i32, usize, i32> = fun2.into_oneof2_var2();
    assert_eq!(Some(&42), fun.ref_at(100));
    assert_eq!(Some(&42), fun.ref_at(7));
}

#[test]
fn iter() {
    let fun1 = Capture(vec![1, 2, 3]).fun_option_ref(|vec, i: usize| vec.get(i));
    let fun2 = Capture(42).fun_option_ref(|scalar, _: usize| Some(scalar));

    let fun: ClosureOptRefOneOf2<Vec<i32>, i32, usize, i32> = fun1.into_oneof2_var1();
    assert_eq!(
        vec![Some(&1), Some(&2), Some(&3), None],
        fun.ref_iter_over(0..4).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![1, 2, 3],
        fun.ref_iter_over(0..3)
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
    );

    let fun: ClosureOptRefOneOf2<Vec<i32>, i32, usize, i32> = fun2.into_oneof2_var2();
    assert_eq!(
        vec![Some(&42), Some(&42)],
        fun.ref_iter_over(0..2).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![42, 42, 42],
        fun.ref_iter_over(0..3)
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
    );
}
