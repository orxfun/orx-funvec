use orx_closure::{Capture, ClosureOptRefOneOf4};
use orx_funvec::*;

struct MyStruct {
    min_index: usize,
}
impl MyStruct {
    fn get(&self, i: usize) -> Option<&i32> {
        if i < self.min_index {
            None
        } else {
            Some(&42)
        }
    }
}

#[test]
fn at() {
    let fun1 = Capture(vec![1, 2, 3]).fun_option_ref(|vec, i: usize| vec.get(i));
    let fun2 = Capture(42).fun_option_ref(|scalar, _: usize| Some(scalar));
    let fun3 = Capture(MyStruct { min_index: 4 }).fun_option_ref(|s, i| s.get(i));
    let fun4 = Capture(()).fun_option_ref(|_, _: usize| Some(&7));

    let fun: ClosureOptRefOneOf4<Vec<i32>, i32, MyStruct, (), usize, i32> = fun1.into_oneof4_var1();
    assert_eq!(Some(&2), fun.ref_at(1));
    assert_eq!(None, fun.ref_at(7));

    let fun: ClosureOptRefOneOf4<Vec<i32>, i32, MyStruct, (), usize, i32> = fun2.into_oneof4_var2();
    assert_eq!(Some(&42), fun.ref_at(100));
    assert_eq!(Some(&42), fun.ref_at(7));

    let fun: ClosureOptRefOneOf4<Vec<i32>, i32, MyStruct, (), usize, i32> = fun3.into_oneof4_var3();
    assert_eq!(Some(&42), fun.ref_at(100));
    assert_eq!(None, fun.ref_at(2));

    let fun: ClosureOptRefOneOf4<Vec<i32>, i32, MyStruct, (), usize, i32> = fun4.into_oneof4_var4();
    assert_eq!(Some(&7), fun.ref_at(100));
    assert_eq!(Some(&7), fun.ref_at(7));
}

#[test]
fn iter() {
    let fun1 = Capture(vec![1, 2, 3]).fun_option_ref(|vec, i: usize| vec.get(i));
    let fun2 = Capture(42).fun_option_ref(|scalar, _: usize| Some(scalar));
    let fun3 = Capture(MyStruct { min_index: 4 }).fun_option_ref(|s, i| s.get(i));
    let fun4 = Capture(()).fun_option_ref(|_, _: usize| Some(&7));

    let fun: ClosureOptRefOneOf4<Vec<i32>, i32, MyStruct, (), usize, i32> = fun1.into_oneof4_var1();
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

    let fun: ClosureOptRefOneOf4<Vec<i32>, i32, MyStruct, (), usize, i32> = fun2.into_oneof4_var2();
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

    let fun: ClosureOptRefOneOf4<Vec<i32>, i32, MyStruct, (), usize, i32> = fun3.into_oneof4_var3();
    assert_eq!(
        vec![None, None, Some(&42)],
        fun.ref_iter_over(2..5).collect::<Vec<_>>()
    );

    let fun: ClosureOptRefOneOf4<Vec<i32>, i32, MyStruct, (), usize, i32> = fun4.into_oneof4_var4();
    assert_eq!(
        vec![Some(&7), Some(&7)],
        fun.ref_iter_over(0..2).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![7, 7, 7],
        fun.ref_iter_over(0..3)
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
    );
}
