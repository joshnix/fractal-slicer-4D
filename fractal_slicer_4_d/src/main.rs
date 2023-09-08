use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point3 {
    x: ordered_float::OrderedFloat<f64>,
    y: ordered_float::OrderedFloat<f64>,
    z: ordered_float::OrderedFloat<f64>,
}

fn is_condition_met(
    i: ordered_float::OrderedFloat<f64>,
    j: ordered_float::OrderedFloat<f64>,
    k: ordered_float::OrderedFloat<f64>,
) -> bool {
    let error_margin = f64::EPSILON;
    ((i % 3.0 - 1.0).abs() < error_margin && (j % 3.0 - 1.0).abs() < error_margin)
        || ((i % 3.0 - 1.0).abs() < error_margin && (k % 3.0 - 1.0).abs() < error_margin)
        || ((j % 3.0 - 1.0).abs() < error_margin && (k % 3.0 - 1.0).abs() < error_margin)
}

fn are_at_least_two_positive(
    i: ordered_float::OrderedFloat<f64>,
    j: ordered_float::OrderedFloat<f64>,
    k: ordered_float::OrderedFloat<f64>,
) -> bool {
    (i > ordered_float::OrderedFloat(0.0) && j > ordered_float::OrderedFloat(0.0))
        || (i > ordered_float::OrderedFloat(0.0) && k > ordered_float::OrderedFloat(0.0))
        || (j > ordered_float::OrderedFloat(0.0) && k > ordered_float::OrderedFloat(0.0))
}

fn keep_point(point: &Point3) -> bool {
    let mut i: ordered_float::OrderedFloat<f64> = point.x;
    let mut j: ordered_float::OrderedFloat<f64> = point.y;
    let mut k: ordered_float::OrderedFloat<f64> = point.z;

    while are_at_least_two_positive(i, j, k) {
        if is_condition_met(i, j, k) {
            return false;
        }
        i = ordered_float::OrderedFloat((i / ordered_float::OrderedFloat(3.0)).floor());
        j = ordered_float::OrderedFloat((j / ordered_float::OrderedFloat(3.0)).floor());
        k = ordered_float::OrderedFloat((k / ordered_float::OrderedFloat(3.0)).floor());
    }
    true
}

#[allow(clippy::cast_precision_loss)]
fn generate_lattice_conc(n: u32) -> Vec<Point3> {
    log::info!("Generating lattice with n = {}", n);
    let max_val = 3u64.pow(n);

    log::info!("Number of threads in use: {}", rayon::current_num_threads());

    (0..max_val)
        .into_par_iter()
        .flat_map(|x| {
            (0..max_val).into_par_iter().flat_map(move |y| {
                (0..max_val).into_par_iter().map(move |z| Point3 {
                    x: ordered_float::OrderedFloat(x as f64),
                    y: ordered_float::OrderedFloat(y as f64),
                    z: ordered_float::OrderedFloat(z as f64),
                })
            })
        })
        .filter(|&point| keep_point(&point))
        .collect()
}

fn generate_vertices(lattice: &Vec<Point3>) -> HashSet<Point3> {
    let mut vertices: HashSet<Point3> = HashSet::default();
    for point in lattice {
        vertices.insert(Point3 {
            x: point.x + 0.5,
            y: point.y + 0.5,
            z: point.z + 0.5,
        });
        vertices.insert(Point3 {
            x: point.x + 0.5,
            y: point.y + 0.5,
            z: point.z - 0.5,
        });
        vertices.insert(Point3 {
            x: point.x + 0.5,
            y: point.y - 0.5,
            z: point.z + 0.5,
        });
        vertices.insert(Point3 {
            x: point.x - 0.5,
            y: point.y + 0.5,
            z: point.z + 0.5,
        });
        vertices.insert(Point3 {
            x: point.x + 0.5,
            y: point.y - 0.5,
            z: point.z - 0.5,
        });
        vertices.insert(Point3 {
            x: point.x - 0.5,
            y: point.y + 0.5,
            z: point.z - 0.5,
        });
        vertices.insert(Point3 {
            x: point.x - 0.5,
            y: point.y - 0.5,
            z: point.z + 0.5,
        });
        vertices.insert(Point3 {
            x: point.x - 0.5,
            y: point.y - 0.5,
            z: point.z - 0.5,
        });
    }
    vertices
}

#[cfg(feature = "logging")]
fn init_logger() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();
}

fn main() {
    init_logger();
    let n = 3;
    let lattice = generate_lattice_conc(n);

    // log::info!("Lattice: {:#?}", lattice);
    log::info!("Lattice size: {}", lattice.len());

    let vertices: HashSet<Point3> = generate_vertices(&lattice);
    log::info!("Vertices size: {}", vertices.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lattice_size() {
        let c: u16 = 20;
        let max_val = 3;
        for n in 1..max_val {
            assert_eq!(generate_lattice_conc(n).len(), c.pow(n).into());
        }
    }

    #[test]
    fn keep_point_true() {
        let test_point = Point3 {
            x: ordered_float::OrderedFloat(2.0),
            y: ordered_float::OrderedFloat(2.0),
            z: ordered_float::OrderedFloat(2.0),
        };
        assert!(keep_point(&test_point));
    }

    #[test]
    fn keep_point_false() {
        let test_point = Point3 {
            x: ordered_float::OrderedFloat(4.0),
            y: ordered_float::OrderedFloat(5.0),
            z: ordered_float::OrderedFloat(3.0),
        };
        assert!(!keep_point(&test_point));
    }

    #[test]
    fn are_at_least_two_positive_true() {
        assert!(are_at_least_two_positive(
            ordered_float::OrderedFloat(1.0),
            ordered_float::OrderedFloat(3.0),
            ordered_float::OrderedFloat(4.0)
        ));
        assert!(are_at_least_two_positive(
            ordered_float::OrderedFloat(0.0),
            ordered_float::OrderedFloat(3.0),
            ordered_float::OrderedFloat(4.0)
        ));
        assert!(are_at_least_two_positive(
            ordered_float::OrderedFloat(-1.0),
            ordered_float::OrderedFloat(3.0),
            ordered_float::OrderedFloat(4.0)
        ));
        assert!(are_at_least_two_positive(
            ordered_float::OrderedFloat(1.0),
            ordered_float::OrderedFloat(-3.0),
            ordered_float::OrderedFloat(4.0)
        ));
        assert!(are_at_least_two_positive(
            ordered_float::OrderedFloat(1.0),
            ordered_float::OrderedFloat(3.0),
            ordered_float::OrderedFloat(-4.0)
        ));
    }

    #[test]
    fn are_at_least_two_positive_false() {
        assert!(!are_at_least_two_positive(
            ordered_float::OrderedFloat(0.0),
            ordered_float::OrderedFloat(0.0),
            ordered_float::OrderedFloat(0.0)
        ));
        assert!(!are_at_least_two_positive(
            ordered_float::OrderedFloat(-1.0),
            ordered_float::OrderedFloat(-4.0),
            ordered_float::OrderedFloat(7.0)
        ));
        assert!(!are_at_least_two_positive(
            ordered_float::OrderedFloat(1.0),
            ordered_float::OrderedFloat(-4.0),
            ordered_float::OrderedFloat(-7.0)
        ));
        assert!(!are_at_least_two_positive(
            ordered_float::OrderedFloat(-1.0),
            ordered_float::OrderedFloat(4.0),
            ordered_float::OrderedFloat(-7.0)
        ));
    }

    #[test]
    fn generate_vertices_test() {
        let lattice = vec![Point3 {
            x: ordered_float::OrderedFloat(1.0),
            y: ordered_float::OrderedFloat(1.0),
            z: ordered_float::OrderedFloat(1.0),
        }];
        let vertices = generate_vertices(&lattice);
        assert!(vertices.len() == lattice.len() * 8);
    }
}
