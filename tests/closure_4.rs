use orx_closure::{Capture, ClosureOneOf4};
use orx_funvec::*;

struct MyStruct {
    min_index: usize,
}
impl MyStruct {
    fn get(&self, i: usize) -> Option<i32> {
        if i < self.min_index {
            None
        } else {
            Some(42)
        }
    }
}

#[test]
fn at() {
    let fun1 = Capture(vec![1, 2, 3]).fun(|vec, i: usize| vec.get(i).cloned());
    let fun2 = Capture(42).fun(|scalar, _: usize| Some(*scalar));
    let fun3 = Capture(MyStruct { min_index: 4 }).fun(|s, i| s.get(i));
    let fun4 = Capture(()).fun(|_, _: usize| Some(7));

    let fun: ClosureOneOf4<Vec<i32>, i32, MyStruct, (), usize, Option<i32>> =
        fun1.into_oneof4_var1();
    assert_eq!(Some(2), fun.val_at(1));
    assert_eq!(None, fun.val_at(7));

    let fun: ClosureOneOf4<Vec<i32>, i32, MyStruct, (), usize, Option<i32>> =
        fun2.into_oneof4_var2();
    assert_eq!(Some(42), fun.val_at(100));
    assert_eq!(Some(42), fun.val_at(7));

    let fun: ClosureOneOf4<Vec<i32>, i32, MyStruct, (), usize, Option<i32>> =
        fun3.into_oneof4_var3();
    assert_eq!(Some(42), fun.val_at(100));
    assert_eq!(None, fun.val_at(2));

    let fun: ClosureOneOf4<Vec<i32>, i32, MyStruct, (), usize, Option<i32>> =
        fun4.into_oneof4_var4();
    assert_eq!(Some(7), fun.val_at(100));
    assert_eq!(Some(7), fun.val_at(7));
}

#[test]
fn iter() {
    let fun1 = Capture(vec![1, 2, 3]).fun(|vec, i: usize| vec.get(i).cloned());
    let fun2 = Capture(42).fun(|scalar, _: usize| Some(*scalar));
    let fun3 = Capture(MyStruct { min_index: 4 }).fun(|s, i| s.get(i));
    let fun4 = Capture(()).fun(|_, _: usize| Some(7));

    let fun: ClosureOneOf4<Vec<i32>, i32, MyStruct, (), usize, Option<i32>> =
        fun1.into_oneof4_var1();
    assert_eq!(
        vec![Some(1), Some(2), Some(3), None],
        fun.val_iter_over(0..4).collect::<Vec<_>>()
    );

    let fun: ClosureOneOf4<Vec<i32>, i32, MyStruct, (), usize, Option<i32>> =
        fun2.into_oneof4_var2();
    assert_eq!(
        vec![Some(42), Some(42)],
        fun.val_iter_over(0..2).collect::<Vec<_>>()
    );

    let fun: ClosureOneOf4<Vec<i32>, i32, MyStruct, (), usize, Option<i32>> =
        fun3.into_oneof4_var3();
    assert_eq!(
        vec![None, None, Some(42)],
        fun.val_iter_over(2..5).collect::<Vec<_>>()
    );

    let fun: ClosureOneOf4<Vec<i32>, i32, MyStruct, (), usize, Option<i32>> =
        fun4.into_oneof4_var4();
    assert_eq!(
        vec![Some(7), Some(7)],
        fun.val_iter_over(0..2).collect::<Vec<_>>()
    );
}
