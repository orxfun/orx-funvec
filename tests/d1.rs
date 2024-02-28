use orx_closure::{
    Capture, ClosureOneOf2, ClosureOneOf3, ClosureOneOf4, ClosureOptRefOneOf2, ClosureOptRefOneOf3,
    ClosureOptRefOneOf4,
};
use orx_funvec::*;
use std::collections::HashSet;

// scalars
#[test]
fn scalars_as_vec() {
    let vec = ScalarAsVec(12);

    assert_eq!(Some(12), vec.at(3));
    assert_eq!(Some(&12), vec.ref_at(8));

    assert_eq!(4 * 12, vec.iter_over(0..4).flatten().sum());
    assert_eq!(4 * 12, vec.ref_iter_over(0..4).flatten().sum());
}

#[test]
fn empty_vec() {
    let vec = EmptyVec::new();

    assert_eq!(None, vec.at(0));
    assert_eq!(None, vec.at(3));

    assert_eq!(0, vec.iter_over(0..4).flatten().sum());
    assert_eq!(0, vec.ref_iter_over(0..4).flatten().sum());
}

// arrays
fn val_assert_contagious<V: FunVec<1, i32>>(vec: &V) {
    // [1, 2, 3]

    assert_eq!(Some(2), vec.at(1));
    assert_eq!(None, vec.at(3));

    assert_eq!(6, vec.iter_over(0..3).flatten().sum());
    assert_eq!(6, vec.iter_over(0..10).flatten().sum());
    assert_eq!(5, vec.iter_over([1, 2].into_iter()).flatten().sum());
}
fn ref_assert_contagious<V: FunVecRef<1, i32>>(vec: &V) {
    // [1, 2, 3]

    assert_eq!(Some(&2), vec.ref_at(1));
    assert_eq!(None, vec.ref_at(3));

    assert_eq!(6, vec.ref_iter_over(0..3).flatten().sum());
    assert_eq!(6, vec.ref_iter_over(0..10).flatten().sum());
    assert_eq!(5, vec.ref_iter_over([1, 2].into_iter()).flatten().sum());
}

#[test]
fn vec() {
    let vec = vec![1, 2, 3];
    val_assert_contagious(&vec);
    ref_assert_contagious(&vec);
}

#[test]
fn array() {
    let arr = [1, 2, 3];
    val_assert_contagious(&arr);
    ref_assert_contagious(&arr);
}

#[cfg(any(feature = "impl_all", feature = "impl_ndarray"))]
#[test]
fn ndarray() {
    use ndarray::Array1;
    let arr = Array1::from_vec(vec![1, 2, 3]);
    val_assert_contagious(&arr);
    ref_assert_contagious(&arr);
}

#[cfg(any(feature = "impl_all", feature = "impl_smallvec"))]
#[test]
fn smallvec() {
    use smallvec::SmallVec;

    let vec: SmallVec<[i32; 2]> = smallvec::smallvec![1, 2, 3];
    val_assert_contagious(&vec);
    ref_assert_contagious(&vec);

    let vec: SmallVec<[i32; 4]> = smallvec::smallvec![1, 2, 3];
    val_assert_contagious(&vec);
    ref_assert_contagious(&vec);
}

// maps
fn val_assert_maps<V: FunVec<1, i32>>(vec: &V) {
    // 1->10 ; 2->20 ; 7->70

    assert_eq!(Some(70), vec.at(7));
    assert_eq!(None, vec.at(0));

    assert_eq!(100, vec.iter_over(0..10).flatten().sum());
    assert_eq!(
        100,
        vec.iter_over([1, 2, 7].iter().cloned()).flatten().sum()
    );
    assert_eq!(80, vec.iter_over([1, 7].iter().cloned()).flatten().sum());
}
fn ref_assert_maps<V: FunVecRef<1, i32>>(vec: &V) {
    // 1->10 ; 2->20 ; 7->70

    assert_eq!(Some(&70), vec.ref_at(7));
    assert_eq!(None, vec.ref_at(0));

    assert_eq!(100, vec.ref_iter_over(0..10).flatten().sum());
    assert_eq!(
        100,
        vec.ref_iter_over([1, 2, 7].iter().cloned()).flatten().sum()
    );
    assert_eq!(
        80,
        vec.ref_iter_over([1, 7].iter().cloned()).flatten().sum()
    );
}

#[test]
fn hashmap() {
    use std::collections::HashMap;

    let map = HashMap::from_iter([(1, 10), (2, 20), (7, 70)]);
    val_assert_maps(&map);
    ref_assert_maps(&map);
}
#[test]
fn btreemap() {
    use std::collections::BTreeMap;

    let map = BTreeMap::from_iter([(1, 10), (2, 20), (7, 70)]);
    val_assert_maps(&map);
    ref_assert_maps(&map);
}
#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
#[test]
fn indexmap() {
    use indexmap::IndexMap;

    let map = IndexMap::from_iter([(1, 10), (2, 20), (7, 70)]);
    val_assert_maps(&map);
    ref_assert_maps(&map);
}

// closures
#[test]
fn closure_no_capture() {
    let closure = Capture(()).fun(|_, i: usize| match i {
        1 => Some(10),
        2 => Some(20),
        7 => Some(70),
        _ => None,
    });
    val_assert_maps(&closure);

    let closure_oneof2: ClosureOneOf2<(), usize, _, _> = closure.clone().into_oneof2_var1();
    val_assert_maps(&closure_oneof2);

    let closure_oneof3: ClosureOneOf3<(), usize, Vec<usize>, _, _> =
        closure.clone().into_oneof3_var1();
    val_assert_maps(&closure_oneof3);

    let closure_oneof4: ClosureOneOf4<(), usize, Vec<usize>, String, _, _> =
        closure.clone().into_oneof4_var1();
    val_assert_maps(&closure_oneof4);

    let closure = Capture(()).fun_option_ref(|_, i: usize| match i {
        1 => Some(&10),
        2 => Some(&20),
        7 => Some(&70),
        _ => None,
    });
    ref_assert_maps(&closure);

    let closure_oneof2: ClosureOptRefOneOf2<(), usize, _, _> = closure.clone().into_oneof2_var1();
    ref_assert_maps(&closure_oneof2);

    let closure_oneof3: ClosureOptRefOneOf3<(), usize, Vec<usize>, _, _> =
        closure.clone().into_oneof3_var1();
    ref_assert_maps(&closure_oneof3);

    let closure_oneof4: ClosureOptRefOneOf4<(), usize, Vec<usize>, String, _, _> =
        closure.clone().into_oneof4_var1();
    ref_assert_maps(&closure_oneof4);
}

#[test]
fn closure_capture() {
    let valid_indices: HashSet<usize> = HashSet::from_iter([1, 2, 7]);

    let closure = Capture(valid_indices).fun(|v, i: usize| {
        if v.contains(&i) {
            Some((i * 10) as i32)
        } else {
            None
        }
    });
    val_assert_maps(&closure);

    let closure_oneof2: ClosureOneOf2<HashSet<usize>, usize, _, _> =
        closure.clone().into_oneof2_var1();
    val_assert_maps(&closure_oneof2);

    let closure_oneof3: ClosureOneOf3<HashSet<usize>, usize, Vec<usize>, _, _> =
        closure.clone().into_oneof3_var1();
    val_assert_maps(&closure_oneof3);

    let closure_oneof4: ClosureOneOf4<HashSet<usize>, usize, Vec<usize>, String, _, _> =
        closure.clone().into_oneof4_var1();
    val_assert_maps(&closure_oneof4);
}

// box_dyn_fn
#[test]
fn box_dyn_fn() {
    let closure: Box<dyn Fn(usize) -> Option<i32>> = Box::new(|i: usize| match i {
        1 => Some(10),
        2 => Some(20),
        7 => Some(70),
        _ => None,
    });
    val_assert_maps(&closure);

    let valid_indices: HashSet<usize> = HashSet::from_iter([1, 2, 7]);
    let closure: Box<dyn Fn(usize) -> Option<i32>> = Box::new(move |i: usize| {
        if valid_indices.contains(&i) {
            Some((i * 10) as i32)
        } else {
            None
        }
    });
    val_assert_maps(&closure);
}
