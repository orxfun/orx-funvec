//! # orx-funvec
//!
//! Traits to unify access to elements of n-dimensional vectors which are particularly useful in algorithms requiring both flexibility through abstraction over inputs and performance through monomorphization.
//!
//! ## A. Traits and Methods
//!
//! **`trait FunVec<const DIM: usize, T>`** represents `DIM` dimensional vectors of `T` requiring only the following method to be implemented:
//!
//! ```rust ignore
//! fn at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<T>
//! ```
//!
//! `FunVec` is different from, a generalization of, multi dimensional vectors due to the following:
//!
//! * The elements are not necessarily contagious or dense. They can rather represent any level of sparsity/density depending on the underlying type.
//! * Assumed to be of infinite length.
//!
//! Additionally, the trait provides with, auto implements, the `iter_over` method which allows to iterate over values of the vector at the given indices.
//!
//! The crate also provides the reference returning counterpart **`FunVecRef<const DIM: usize, T>`** requiring the method `fn ref_at<Idx: IntoIndex<DIM>>(&self, index: Idx) -> Option<&T>`.
//!
//! ## B. Implementations and Features
//!
//! ### B.1. Available Implementations
//!
//! This crate provides implementations for a wide range of types which are useful in algorithms. For instance, the following implementations are available:
//!
//! | **`trait FunVec<1, T>`**                    | **`trait FunVec<2, T>`**                                      |
//! |---------------------------------------------|---------------------------------------------------------------|
//! | `Vec<T>`                                    |                                                               |
//! | `[T; N]`                                    |                                                               |
//! | `HashMap<usize, T>` \| `BTreeMap<usize, T>` | `HashMap<(usize, usize), T>` \| `BTreeMap<[usize, usize], T>` |
//! | `Closure<Capture, usize, T>`                | `Closure<Capture, (usize, usize), T>`                         |
//! | `Box<dyn Fn(usize) -> T>`                   | `Box<dyn Fn([usize, usize] -> T)`                             |
//!
//! You may notice the pattern in the indices; `(usize, usize)` or `[usize, usize]` can be used interchangeable as they both implement `IntoIndex<2>`. And as we move to higher dimensions, only the index dimension changes.
//!
//! However, the recursive implementations to allow for compositisons are also available. For instance all of the following types implement `FunVec<2, T>` for any `V1` provided that `V1` implements `FunVec<1, T>`:
//!
//! * `Vec<V1>`
//! * `[V1; N]`
//! * `HashMap<usize, V1>` | `BTreeMap<usize, V1>`
//! * `Closure<Capture, usize, V1>`
//! * `Box<dyn Fn(usize) -> V1>`
//!
//! Lastly, `ScalarAsVec<T>` and `EmptyVec<T>` implement `FunVec<D, T>` for any dimension `D`. These turn out to be useful common special cases.
//!
//! ### B.2. Optional Implementations by Features
//!
//! Finally, the following implementations are optionally provided through features:
//!
//! * `ndarray` by `impl_ndarray` feature,
//! * `indexmap` by `impl_indexmap` feature,
//! * `smallvec` by `impl_smallvec` feature,
//! * or all implementations by `impl_all` feature.
//!
//! ### B.3. Extension
//!
//! Implementing the trait for a new type is straightforward, requiring only to implement `at` method. Please see section <a href="#c5">C5</a> for an example.
//!
//! ## C. Motivation
//!
//! The motivation of the create is demonstrated by a use case in the following example.
//!
//! Assume we need to solve a network problem, namely minimum cost network flow (mcnf) problem ([wikipedia](https://en.wikipedia.org/wiki/Minimum-cost_flow_problem)). In the example, we ignore the graph input. Instead, we focus on the following inputs:
//!
//! * demands: each node of the graph has an amount of demand, which represents supply when negative.
//! * costs: each arc has an associated unit flow cost.
//! * capacities: each arc has a limited flow capacity.
//!
//! The mcnf problem provides a certain level of abstraction over inputs. For instance:
//! * If demands are non-zero only for two nodes, the problem is a single commodity problem; otherwise, it can represent a multi commodity mcnf problem.
//! * If demands are 1 and -1 for source and sink nodes and zero for all others, and if capacities are irrelevant, the problem becomes a shortest path problem.
//! * If we want to find the least number of arcs rather than the shortest path, we can use a costs matrix whose elements are 1 for all arcs.
//!
//! Abstraction over the inputs is powerful since it allows to implement a generic mcnf solver without the need to make assumptions on the concrete input types.
//!
//! ### C.1. Problem Setup
//!
//! Below we implement our fake solver generic over the input types.
//!
//! ```rust
//! use orx_funvec::*;
//!
//! const N: usize = 4;
//! type Unit = i32;
//!
//! #[derive(derive_new::new)]
//! struct FakeResult {
//!     sum_demands: i32,
//!     sum_costs: i32,
//!     sum_capacities: i32,
//! }
//!
//! #[derive(derive_new::new)]
//! struct FakeMcnfSolver<Demands, Costs, Capacities>
//! where
//!     Demands: FunVec<1, Unit>,
//!     Costs: FunVec<2, Unit>,
//!     Capacities: FunVec<2, Unit>,
//! {
//!     demands: Demands,
//!     costs: Costs,
//!     capacities: Capacities,
//! }
//!
//! impl<Demands, Costs, Capacities> FakeMcnfSolver<Demands, Costs, Capacities>
//! where
//!     Demands: FunVec<1, Unit>,
//!     Costs: FunVec<2, Unit>,
//!     Capacities: FunVec<2, Unit>,
//! {
//!     fn fake_solve(&self) -> FakeResult {
//!         let sum_demands = self
//!             .demands
//!             .iter_over(0..N)
//!             .flatten()
//!             .filter(|x| x > &0)
//!             .sum();
//!
//!         let mut sum_costs = 0;
//!         let mut sum_capacities = 0;
//!         for i in 0..N {
//!             for j in 0..N {
//!                 if let Some(cost) = self.costs.at([i, j]) {
//!                     sum_costs += cost;
//!                 }
//!
//!                 if let Some(capacity) = self.capacities.at((i, j)) {
//!                     sum_capacities += capacity;
//!                 }
//!             }
//!         }
//!         FakeResult::new(sum_demands, sum_costs, sum_capacities)
//!     }
//! }
//! ```
//!
//! ### C.2. Variant - Single Commodity
//!
//! In the below example, we use our generic solver with:
//!
//! * a single commodity problem where demands vector is a cheap closure (`Closure<_, usize, Unit>`),
//! * a complete costs matrix (`Vec<Vec<Unit>>`),
//! * a uniform capacity matrix represented as a cheap closure (`Box<dyn Fn((usize, usize)) -> Unit>`).
//!
//! ```rust
//! # use orx_funvec::*;
//! #
//! # const N: usize = 4;
//! # type Unit = i32;
//! #
//! # #[derive(derive_new::new)]
//! # struct FakeResult {
//! #     sum_demands: i32,
//! #     sum_costs: i32,
//! #     sum_capacities: i32,
//! # }
//!
//! # #[derive(derive_new::new)]
//! # struct FakeMcnfSolver<Demands, Costs, Capacities>
//! # where
//! #     Demands: FunVec<1, Unit>,
//! #     Costs: FunVec<2, Unit>,
//! #     Capacities: FunVec<2, Unit>,
//! # {
//! #     demands: Demands,
//! #     costs: Costs,
//! #     capacities: Capacities,
//! # }
//! #
//! # impl<Demands, Costs, Capacities> FakeMcnfSolver<Demands, Costs, Capacities>
//! # where
//! #     Demands: FunVec<1, Unit>,
//! #     Costs: FunVec<2, Unit>,
//! #     Capacities: FunVec<2, Unit>,
//! # {
//! #     fn fake_solve(&self) -> FakeResult {
//! #         let sum_demands = self
//! #             .demands
//! #             .iter_over(0..N)
//! #             .flatten()
//! #             .filter(|x| x > &0)
//! #             .sum();
//! #
//! #         let mut sum_costs = 0;
//! #         let mut sum_capacities = 0;
//! #         for i in 0..N {
//! #             for j in 0..N {
//! #                 if let Some(cost) = self.costs.at([i, j]) {
//! #                     sum_costs += cost;
//! #                 }
//! #
//! #                 if let Some(capacity) = self.capacities.at((i, j)) {
//! #                     sum_capacities += capacity;
//! #                 }
//! #             }
//! #         }
//! #         FakeResult::new(sum_demands, sum_costs, sum_capacities)
//! #     }
//! # }
//! use orx_closure::Capture;
//!
//! fn some_if_not_self_edge(ij: (usize, usize), value: i32) -> Option<i32> {
//!     if ij.0 == ij.1 {
//!         None
//!     } else {
//!         Some(value)
//!     }
//! }
//!
//! // mcnf problem with a single source and sink
//! let source = 0;
//! let sink = 2;
//! let demand = 10;
//!
//! // demands vector as a no-box orx_closure::Closure
//! let demands =
//!     Capture((source, sink, demand)).fun(|(s, t, d), i: usize| match (i == *s, i == *t) {
//!         (true, _) => Some(*d),
//!         (_, true) => Some(-*d),
//!         _ => None,
//!     });
//!
//! // complete cost matrix
//! let costs = vec![
//!     vec![0, 1, 2, 3],
//!     vec![2, 0, 2, 2],
//!     vec![7, 10, 0, 9],
//!     vec![1, 1, 1, 0],
//! ];
//!
//! // capacities matrix as a box dyn Fn
//! let capacities: Box<dyn Fn((usize, usize)) -> Option<i32>> =
//!     Box::new(|ij: (usize, usize)| some_if_not_self_edge(ij, 100));
//!
//! // simulate & assert
//! let solver = FakeMcnfSolver::new(demands, costs, capacities);
//! let result = solver.fake_solve();
//!
//! assert_eq!(10, result.sum_demands);
//! assert_eq!(41, result.sum_costs);
//! assert_eq!((N * (N - 1) * 100) as i32, result.sum_capacities);
//! ```
//!
//! ### C.3. Variant - Multi Commodity
//!
//! In the second example variant, we use our solver with:
//!
//! * a multi commodity problem where demands vector is computed by a closure on the fly (`Closure<_, usize, Unit>`),
//! * arc costs are computed as Euclidean distances by a closure using captured node locations (`Closure<_, (usize, usize), Unit>`),
//! * a sparse capacity matrix using hash map (`Vec<HashMap<usize, Unit>>`).
//!
//! ```rust
//! # use orx_funvec::*;
//! #
//! # const N: usize = 4;
//! # type Unit = i32;
//! #
//! # #[derive(derive_new::new)]
//! # struct FakeResult {
//! #     sum_demands: i32,
//! #     sum_costs: i32,
//! #     sum_capacities: i32,
//! # }
//!
//! # #[derive(derive_new::new)]
//! # struct FakeMcnfSolver<Demands, Costs, Capacities>
//! # where
//! #     Demands: FunVec<1, Unit>,
//! #     Costs: FunVec<2, Unit>,
//! #     Capacities: FunVec<2, Unit>,
//! # {
//! #     demands: Demands,
//! #     costs: Costs,
//! #     capacities: Capacities,
//! # }
//! #
//! # impl<Demands, Costs, Capacities> FakeMcnfSolver<Demands, Costs, Capacities>
//! # where
//! #     Demands: FunVec<1, Unit>,
//! #     Costs: FunVec<2, Unit>,
//! #     Capacities: FunVec<2, Unit>,
//! # {
//! #     fn fake_solve(&self) -> FakeResult {
//! #         let sum_demands = self
//! #             .demands
//! #             .iter_over(0..N)
//! #             .flatten()
//! #             .filter(|x| x > &0)
//! #             .sum();
//! #
//! #         let mut sum_costs = 0;
//! #         let mut sum_capacities = 0;
//! #         for i in 0..N {
//! #             for j in 0..N {
//! #                 if let Some(cost) = self.costs.at([i, j]) {
//! #                     sum_costs += cost;
//! #                 }
//! #
//! #                 if let Some(capacity) = self.capacities.at((i, j)) {
//! #                     sum_capacities += capacity;
//! #                 }
//! #             }
//! #         }
//! #         FakeResult::new(sum_demands, sum_costs, sum_capacities)
//! #     }
//! # }
//! use orx_closure::Capture;
//! use std::collections::HashMap;
//!
//! fn get_euclidean_distance(location1: (f64, f64), location2: (f64, f64)) -> i32 {
//!     let (x1, y1) = location1;
//!     let (x2, y2) = location2;
//!     (f64::powf(x1 - x2, 2.0) + f64::powf(y1 - y2, 2.0)).sqrt() as i32
//! }
//!
//! // multi-commodity mcnf problem
//! struct Commodity {
//!     origin: usize,
//!     destination: usize,
//!     amount: Unit,
//! }
//! let commodities = vec![
//!     Commodity {
//!         origin: 0,
//!         destination: 2,
//!         amount: 10,
//!     },
//!     Commodity {
//!         origin: 1,
//!         destination: 3,
//!         amount: 20,
//!     },
//! ];
//!
//! // demands vector as a no-box orx_closure::Closure capturing a reference of commodities collection
//! let demands = Capture(&commodities).fun(|com, i: usize| {
//!     Some(
//!         com.iter()
//!             .map(|c| {
//!                 if c.origin == i {
//!                     c.amount
//!                 } else if c.destination == i {
//!                     -c.amount
//!                 } else {
//!                     0
//!                 }
//!             })
//!             .sum::<i32>(),
//!     )
//! });
//!
//! // costs computed as Euclidean distances of node coordinates
//! let locations = vec![(0.0, 3.0), (3.0, 5.0), (7.0, 2.0), (1.0, 1.0)];
//! let costs = Capture(locations).fun(|loc, (i, j): (usize, usize)| {
//!     loc.get(i)
//!         .and_then(|l1| loc.get(j).map(|l2| (l1, l2)))
//!         .map(|(l1, l2)| get_euclidean_distance(*l1, *l2))
//! });
//!
//! // capacities defined as a Vec of HashMap to take advantage of sparsity in the graph
//! let capacities = vec![
//!     HashMap::from_iter([(1, 100), (3, 200)].into_iter()),
//!     HashMap::from_iter([(3, 300)].into_iter()),
//!     HashMap::from_iter([(0, 400), (3, 500)].into_iter()),
//!     HashMap::new(),
//! ];
//!
//! // simulate & assert
//! let solver = FakeMcnfSolver::new(demands, costs, capacities);
//! let result = solver.fake_solve();
//!
//! assert_eq!(30, result.sum_demands);
//! assert_eq!(54, result.sum_costs);
//! assert_eq!(1500, result.sum_capacities);
//! ```
//!
//! ### C.4. Variant - Shortest Distance
//!
//! Next, we solve a shortest distance problem with the generic solver:
//!
//! * demands vector is all zeros except for the source and sink represented as a cheap closure (`Closure<_, usize, Unit>`),
//! * arc costs are computed as Euclidean distances by a closure using captured node locations (`Closure<_, (usize, usize), Unit>`),
//! * capacities are all-ones-matrix represented by a scalar value which has the memory size of a number and `at` calls will be replaced by the inlined value (`ScalarAsVec<Unit>`).
//!
//! ```rust
//! # use orx_closure::*;
//! # use orx_funvec::*;
//! #
//! # const N: usize = 4;
//! # type Unit = i32;
//! #
//! # #[derive(derive_new::new)]
//! # struct FakeResult {
//! #     sum_demands: i32,
//! #     sum_costs: i32,
//! #     sum_capacities: i32,
//! # }
//! #
//! # #[derive(derive_new::new)]
//! # struct FakeMcnfSolver<Demands, Costs, Capacities>
//! # where
//! #     Demands: FunVec<1, Unit>,
//! #     Costs: FunVec<2, Unit>,
//! #     Capacities: FunVec<2, Unit>,
//! # {
//! #     demands: Demands,
//! #     costs: Costs,
//! #     capacities: Capacities,
//! # }
//! #
//! # impl<Demands, Costs, Capacities> FakeMcnfSolver<Demands, Costs, Capacities>
//! # where
//! #     Demands: FunVec<1, Unit>,
//! #     Costs: FunVec<2, Unit>,
//! #     Capacities: FunVec<2, Unit>,
//! # {
//! #     fn fake_solve(&self) -> FakeResult {
//! #         let sum_demands = self
//! #             .demands
//! #             .iter_over(0..N)
//! #             .flatten()
//! #             .filter(|x| x > &0)
//! #             .sum();
//! #
//! #         let mut sum_costs = 0;
//! #         let mut sum_capacities = 0;
//! #         for i in 0..N {
//! #             for j in 0..N {
//! #                 if let Some(cost) = self.costs.at([i, j]) {
//! #                     sum_costs += cost;
//! #                 }
//! #
//! #                 if let Some(capacity) = self.capacities.at((i, j)) {
//! #                     sum_capacities += capacity;
//! #                 }
//! #             }
//! #         }
//! #         FakeResult::new(sum_demands, sum_costs, sum_capacities)
//! #     }
//! # }
//! # fn get_euclidean_distance(location1: (f64, f64), location2: (f64, f64)) -> i32 {
//! #     let (x1, y1) = location1;
//! #     let (x2, y2) = location2;
//! #     (f64::powf(x1 - x2, 2.0) + f64::powf(y1 - y2, 2.0)).sqrt() as i32
//! # }
//! let source = 3;
//! let sink = 1;
//!
//! // demands vector as a no-box orx_closure::Closure
//! let demands = Capture((source, sink)).fun(|(s, t), i: usize| match (i == *s, i == *t) {
//!     (true, _) => Some(1),
//!     (_, true) => Some(-1),
//!     _ => None,
//! });
//!
//! // costs computed as Euclidean distances of node coordinates
//! let locations = vec![(0.0, 3.0), (3.0, 5.0), (7.0, 2.0), (1.0, 1.0)];
//! let costs = Capture(locations).fun(|loc, (i, j): (usize, usize)| {
//!     loc.get(i)
//!         .and_then(|l1| loc.get(j).map(|l2| (l1, l2)))
//!         .map(|(l1, l2)| get_euclidean_distance(*l1, *l2))
//! });
//!
//! // uniform capacities for all edges
//! let capacities = ScalarAsVec(1);
//!
//! // simulate & assert
//! let solver = FakeMcnfSolver::new(demands, costs, capacities);
//! let result = solver.fake_solve();
//!
//! assert_eq!(1, result.sum_demands);
//! assert_eq!(54, result.sum_costs);
//! assert_eq!((N * N) as i32, result.sum_capacities);
//! ```
//!
//! <div id="c5"></div>
//!
//! ### C.5. Variant - Special Costs Matrix
//!
//! So far in the above examples, we have made use of the available implementations. Here, we will see an example where we implement `FunVec` for our own type.
//!
//! Consider a case where computing the cost between two nodes is expensive. We might be required to solve shortest path problem for each pair. We want to be lazy in this computation because the solver will not need the cost between each pair. Therefore, we decide to compute the costs on the fly when requested. On the other hand, we do not want to repeat the computation when the solver requests the same pair multiple times, and hence, we will cache already computed costs. In brief, we want a costs matrix which is a shortest path algorithm combined with an internal cache.
//!
//! ```rust
//! # type Unit = i32;
//! use std::{cell::RefCell, collections::HashMap};
//!
//! #[derive(Default)]
//! struct DistanceProvider {
//!     // graph: skipping for the brevity, but we'd probably hold a graph ref to compute the shortest distances
//!     cached: RefCell<HashMap<(usize, usize), Option<Unit>>>,
//! }
//! impl DistanceProvider {
//!     fn distance(&self, from: usize, to: usize) -> Option<Unit> {
//!         if let Some(cached) = self.cached.borrow().get(&(from, to)) {
//!             return *cached;
//!         }
//!
//!         let distance = self.compute_shortest_distance(from, to);
//!         self.cached.borrow_mut().insert((from, to), distance);
//!         distance
//!     }
//!
//!     /// Computes shortest distance between `from` and `to` returns `None` if `to` is unreachable from `from`.
//!     fn compute_shortest_distance(&self, _from: usize, _to: usize) -> Option<Unit> {
//!         Some(1) // expensive computation!
//!     }
//! }
//! ```
//!
//! Now, can we use our `DistanceProvider` as the 'costs' input of the `FakeMcnfSolver`? Yes. All we need is to implement the `at` method of `FunVec<2, Unit>` as below.
//!
//! ```rust
//! # use orx_funvec::*;
//! # type Unit = i32;
//! # use std::{cell::RefCell, collections::HashMap};
//! #
//! # #[derive(Default)]
//! # struct DistanceProvider {
//! #     // graph: skipping for the brevity, but we'd probably hold a graph ref to compute the shortest distances
//! #     cached: RefCell<HashMap<(usize, usize), Option<Unit>>>,
//! # }
//! # impl DistanceProvider {
//! #     fn distance(&self, from: usize, to: usize) -> Option<Unit> {
//! #         if let Some(cached) = self.cached.borrow().get(&(from, to)) {
//! #             return *cached;
//! #         }
//! #
//! #         let distance = self.compute_shortest_distance(from, to);
//! #         self.cached.borrow_mut().insert((from, to), distance);
//! #         distance
//! #     }
//! #
//! #     /// Computes shortest distance between `from` and `to` returns `None` if `to` is unreachable from `from`.
//! #     fn compute_shortest_distance(&self, _from: usize, _to: usize) -> Option<Unit> {
//! #         Some(1) // expensive computation!
//! #     }
//! # }
//! impl FunVec<2, Unit> for DistanceProvider {
//!     fn at<Idx: IntoIndex<2>>(&self, index: Idx) -> Option<Unit> {
//!         let [from, to] = index.into_index();
//!         self.distance(from, to)
//!     }
//! }
//! ```
//!
//! This abstraction allows us to keep the algorithm intact while we can make significant changes on how we represent the inputs.
//!
//! ```rust
//! # use orx_closure::*;
//! # use orx_funvec::*;
//! #
//! # const N: usize = 4;
//! # type Unit = i32;
//! #
//! # #[derive(derive_new::new)]
//! # struct FakeResult {
//! #     sum_demands: i32,
//! #     sum_costs: i32,
//! #     sum_capacities: i32,
//! # }
//! # #[derive(derive_new::new)]
//! # struct FakeMcnfSolver<Demands, Costs, Capacities>
//! # where
//! #     Demands: FunVec<1, Unit>,
//! #     Costs: FunVec<2, Unit>,
//! #     Capacities: FunVec<2, Unit>,
//! # {
//! #     demands: Demands,
//! #     costs: Costs,
//! #     capacities: Capacities,
//! # }
//! # impl<Demands, Costs, Capacities> FakeMcnfSolver<Demands, Costs, Capacities>
//! # where
//! #     Demands: FunVec<1, Unit>,
//! #     Costs: FunVec<2, Unit>,
//! #     Capacities: FunVec<2, Unit>,
//! # {
//! #     fn fake_solve(&self) -> FakeResult {
//! #         let sum_demands = self
//! #             .demands
//! #             .iter_over(0..N)
//! #             .flatten()
//! #             .filter(|x| x > &0)
//! #             .sum();
//! #
//! #         let mut sum_costs = 0;
//! #         let mut sum_capacities = 0;
//! #         for i in 0..N {
//! #             for j in 0..N {
//! #                 if let Some(cost) = self.costs.at([i, j]) {
//! #                     sum_costs += cost;
//! #                 }
//! #
//! #                 if let Some(capacity) = self.capacities.at((i, j)) {
//! #                     sum_capacities += capacity;
//! #                 }
//! #             }
//! #         }
//! #         FakeResult::new(sum_demands, sum_costs, sum_capacities)
//! #     }
//! # }
//! # use std::{cell::RefCell, collections::HashMap};
//! # #[derive(Default)]
//! # struct DistanceProvider {
//! #     // graph: skipping for the brevity, but we'd probably hold a graph ref to compute the shortest distances
//! #     cached: RefCell<HashMap<(usize, usize), Option<Unit>>>,
//! # }
//! # impl DistanceProvider {
//! #     fn distance(&self, from: usize, to: usize) -> Option<Unit> {
//! #         if let Some(cached) = self.cached.borrow().get(&(from, to)) {
//! #             return *cached;
//! #         }
//! #
//! #         let distance = self.compute_shortest_distance(from, to);
//! #         self.cached.borrow_mut().insert((from, to), distance);
//! #         distance
//! #     }
//! #
//! #     /// Computes shortest distance between `from` and `to` returns `None` if `to` is unreachable from `from`.
//! #     fn compute_shortest_distance(&self, _from: usize, _to: usize) -> Option<Unit> {
//! #         Some(1) // expensive computation!
//! #     }
//! # }
//! # use orx_funvec::*;
//! # impl FunVec<2, Unit> for DistanceProvider {
//! #     fn at<Idx: IntoIndex<2>>(&self, index: Idx) -> Option<Unit> {
//! #         let [from, to] = index.into_index();
//! #         self.distance(from, to)
//! #     }
//! # }
//! let source = 3;
//! let sink = 1;
//!
//! // demands vector as a no-box orx_closure::Closure
//! let demands = Capture((source, sink)).fun(|(s, t), i: usize| match (i == *s, i == *t) {
//!     (true, _) => Some(1),
//!     (_, true) => Some(-1),
//!     _ => None,
//! });
//!
//! // our custom caching distance provider
//! let costs = DistanceProvider::default();
//!
//! // uniform capacities for all edges
//! let capacities = ScalarAsVec(1);
//!
//! // simulate & assert
//! let solver = FakeMcnfSolver::new(demands, costs, capacities);
//! let result = solver.fake_solve();
//!
//! assert_eq!(1, result.sum_demands);
//! assert_eq!(4 * 4, result.sum_costs);
//! assert_eq!((N * N) as i32, result.sum_capacities);
//! ```

#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

mod d1;
mod d2;
mod d3;
mod d4;
mod d_any;
mod empty_vec;
mod funvec_ref;
mod funvec_val;
mod index;
mod iter_over_ref;
mod iter_over_val;
mod scalar_as_vec;

pub use empty_vec::EmptyVec;
pub use funvec_ref::FunVecRef;
pub use funvec_val::FunVec;
pub use index::{FromIndex, IntoIndex};
pub use scalar_as_vec::ScalarAsVec;
