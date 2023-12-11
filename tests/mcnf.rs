use orx_closure::Capture;
use orx_funvec::{FunVecD1, FunVecD2, Scalar};
use std::collections::HashMap;

const N: usize = 4;
type Unit = i32;

#[derive(derive_new::new)]
struct FakeResult {
    sum_demands: i32,
    sum_costs: i32,
    sum_capacities: i32,
}

#[derive(derive_new::new)]
struct FakeMcnfSolver<Demands, Costs, Capacities>
where
    Demands: FunVecD1<Unit>,
    Costs: FunVecD2<Unit>,
    Capacities: FunVecD2<Unit>,
{
    demands: Demands,
    costs: Costs,
    capacities: Capacities,
}

impl<Demands, Costs, Capacities> FakeMcnfSolver<Demands, Costs, Capacities>
where
    Demands: FunVecD1<Unit>,
    Costs: FunVecD2<Unit>,
    Capacities: FunVecD2<Unit>,
{
    fn fake_solve(&self) -> FakeResult {
        let sum_demands = self
            .demands
            .val_iter_over(0..N)
            .flatten()
            .filter(|x| x > &0)
            .sum();

        let mut sum_costs = 0;
        let mut sum_capacities = 0;
        for i in 0..N {
            for j in 0..N {
                if let Some(cost) = self.costs.val_at(i, j) {
                    sum_costs += cost;
                }

                if let Some(capacity) = self.capacities.val_at(i, j) {
                    sum_capacities += capacity;
                }
            }
        }
        FakeResult::new(sum_demands, sum_costs, sum_capacities)
    }
}

fn const_if_not_self_edge(ij: (usize, usize), value: i32) -> Option<i32> {
    if ij.0 == ij.1 {
        None
    } else {
        Some(value)
    }
}

fn get_euclidean_distance(location1: (f64, f64), location2: (f64, f64)) -> i32 {
    let (x1, y1) = location1;
    let (x2, y2) = location2;
    (f64::powf(x1 - x2, 2.0) + f64::powf(y1 - y2, 2.0)).sqrt() as i32
}

#[test]
#[cfg(any(feature = "impl_all", feature = "impl_ndarray"))]
fn single_commodity_mcnf() {
    // mcnf problem with a single source and sink
    let source = 0;
    let sink = 2;
    let demand = 10;

    // demands vector as a no-box orx_closure::Closure
    let demands = Capture((source, sink, demand)).fun(|(s, t, d), i| match (i == *s, i == *t) {
        (true, _) => Some(*d),
        (_, true) => Some(-*d),
        _ => None,
    });

    // complete cost matrix using ndarray::Array2
    let mut costs = ndarray::Array2::zeros((N, N));
    costs[[0, 1]] = 1;
    costs[[0, 2]] = 2;
    costs[[0, 3]] = 3;
    costs[[1, 0]] = 2;
    costs[[1, 2]] = 2;
    costs[[1, 3]] = 2;
    costs[[2, 0]] = 7;
    costs[[2, 1]] = 10;
    costs[[2, 3]] = 9;
    costs[[3, 0]] = 1;
    costs[[3, 1]] = 1;
    costs[[3, 2]] = 1;

    // capacities matrix as a box dyn Fn
    let capacities: Box<dyn Fn((usize, usize)) -> Option<i32>> =
        Box::new(|ij: (usize, usize)| const_if_not_self_edge(ij, 100));

    // simulate & assert
    let solver = FakeMcnfSolver::new(demands, costs, capacities);
    let result = solver.fake_solve();

    assert_eq!(10, result.sum_demands);
    assert_eq!(41, result.sum_costs);
    assert_eq!((N * (N - 1) * 100) as i32, result.sum_capacities);
}

#[test]
fn multi_commodity_mcnf() {
    // multi-commodity mcnf problem
    struct Commodity {
        origin: usize,
        destination: usize,
        amount: Unit,
    }
    let commodities = vec![
        Commodity {
            origin: 0,
            destination: 2,
            amount: 10,
        },
        Commodity {
            origin: 1,
            destination: 3,
            amount: 20,
        },
    ];

    // demands vector as a no-box orx_closure::Closure capturing a reference of commodities collection
    let demands = Capture(&commodities).fun(|com, i: usize| {
        Some(
            com.iter()
                .map(|c| {
                    if c.origin == i {
                        c.amount
                    } else if c.destination == i {
                        -c.amount
                    } else {
                        0
                    }
                })
                .sum::<i32>(),
        )
    });

    // costs computed as Euclidean distances of node coordinates
    let locations = vec![(0.0, 3.0), (3.0, 5.0), (7.0, 2.0), (1.0, 1.0)];
    let costs = Capture(locations).fun(|loc, (i, j): (usize, usize)| {
        loc.get(i)
            .and_then(|l1| loc.get(j).map(|l2| (l1, l2)))
            .map(|(l1, l2)| get_euclidean_distance(*l1, *l2))
    });

    // capacities defined as a Vec of HashMap to take advantage of sparsity in the graph
    let capacities = vec![
        HashMap::from_iter([(1, 100), (3, 200)].into_iter()),
        HashMap::from_iter([(3, 300)].into_iter()),
        HashMap::from_iter([(0, 400), (3, 500)].into_iter()),
        HashMap::new(),
    ];

    // simulate & assert
    let solver = FakeMcnfSolver::new(demands, costs, capacities);
    let result = solver.fake_solve();

    assert_eq!(30, result.sum_demands);
    assert_eq!(54, result.sum_costs);
    assert_eq!(1500, result.sum_capacities);
}

#[test]
fn shortest_distance() {
    let source = 3;
    let sink = 1;

    // demands vector as a no-box orx_closure::Closure
    let demands = Capture((source, sink)).fun(|(s, t), i| match (i == *s, i == *t) {
        (true, _) => Some(1),
        (_, true) => Some(-1),
        _ => None,
    });

    // costs computed as Euclidean distances of node coordinates
    let locations = vec![(0.0, 3.0), (3.0, 5.0), (7.0, 2.0), (1.0, 1.0)];
    let costs = Capture(locations).fun(|loc, (i, j): (usize, usize)| {
        loc.get(i)
            .and_then(|l1| loc.get(j).map(|l2| (l1, l2)))
            .map(|(l1, l2)| get_euclidean_distance(*l1, *l2))
    });

    // uniform capacities for all edges: any scalar qualifies as a FunVecD1, FunVecD2, etc.
    let capacities = Scalar::new(1);

    // simulate & assert
    let solver = FakeMcnfSolver::new(demands, costs, capacities);
    let result = solver.fake_solve();

    assert_eq!(1, result.sum_demands);
    assert_eq!(54, result.sum_costs);
    assert_eq!((N * N) as i32, result.sum_capacities);
}

#[test]
fn shortest_num_edges() {
    let source = 3;
    let sink = 1;

    // demands vector as a no-box orx_closure::Closure
    let demands = Capture((source, sink)).fun(|(s, t), i| match (i == *s, i == *t) {
        (true, _) => Some(1),
        (_, true) => Some(-1),
        _ => None,
    });

    // uniform capacities and costs for all edges
    let costs = Scalar::new(1);
    let capacities = Capture(()).fun(|_, ij| const_if_not_self_edge(ij, 1));

    // simulate & assert
    let solver = FakeMcnfSolver::new(demands, costs, capacities);
    let result = solver.fake_solve();

    assert_eq!(1, result.sum_demands);
    assert_eq!((N * N) as i32, result.sum_costs);
    assert_eq!((N * (N - 1)) as i32, result.sum_capacities);
}
