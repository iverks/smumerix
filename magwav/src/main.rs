#![allow(unused)]

mod plotty;
mod system;

use core::num;
use std::{
    collections::HashMap,
    time::{self, Instant},
};

use nalgebra as na;
use ndarray::{s, Array3};
use plotty::{plot_system, PlotDirection};
use rand::{thread_rng, Rng};
use rand_distr::{uniform, Distribution, Normal};
use system::{Magnet, MagneticSystem, E_Z};

fn testy() {
    let straight = {
        let (mx, my) = (0.0, 0.0);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    let tilted = {
        let (mx, my) = (-0.2, 0.0);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    let tilted2 = {
        let (mx, my) = (0.2, 0.0);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    let mut sys = MagneticSystem {
        magnets: Array3::from_elem((1, 1, 10), straight),
        dampening_constant: 0.0,
        coupling_constant: 0.000,
        anisotropy_constant: 100e-3,
        temperature: 0.0e-4,
        magnetic_field: 0.0 * E_Z,
        timestep: 5e-16,
    };

    sys.magnets[(0, 0, 5)] = tilted;
    sys.magnets[(0, 0, 4)] = tilted2;

    let mut states = vec![sys.magnets.clone()];

    for _ in 0..200 {
        for _ in 0..1 {
            sys.step();
        }
        states.push(sys.magnets.clone());
    }

    plot_system(&states, "testplot.gif", 100, PlotDirection::Testy).unwrap();
}

fn task_2_1_1() {
    let start = Instant::now();
    let magnet = {
        let (mx, my) = (0.3, 0.3);
        Magnet::new(mx, my, (1.0 - mx.powi(2) - my.powi(2)).sqrt())
    };

    // let mut sys = MagneticSystem::new_toy(Array3::from_element(1, 1, magnet), 10e-3, 0.0, 1e6);
    let mut sys = MagneticSystem {
        magnets: Array3::from_elem((1, 1, 1), magnet),
        dampening_constant: 0.1,
        coupling_constant: 0.1,
        anisotropy_constant: 1e-3,
        temperature: 0.1 * 1e-3,
        magnetic_field: 0.0 * E_Z,
        timestep: 1e-15,
    };
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..200 {
        for _ in 0..10 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }

    println!("Simulation took {} ms", start.elapsed().as_millis());

    let start = Instant::now();
    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_1).unwrap();
    println!("Plotting took {} s", start.elapsed().as_secs());
}

fn task_2_1_2() {
    let start = Instant::now();

    let magnets = {
        let mut rng = thread_rng();
        // let mut normal = Normal::new(0.0, 1.0).unwrap();
        Array3::from_shape_fn((1, 1, 10), |_| {
            let magnet = Magnet::from_fn(|_, _| rng.gen_range(-1.0..1.0));
            magnet.normalize()
        })
    };

    // let mut sys = MagneticSystem::new_toy(Array3::from_element(1, 1, magnet), 10e-3, 0.0, 1e6);
    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 0.1,
        coupling_constant: -10.0 * 1e-3,
        anisotropy_constant: 3.0 * 1e-3,
        temperature: 0.1 * 1e-3,
        magnetic_field: 0.0 * E_Z,
        timestep: 1e-15,
    };
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..200 {
        for _ in 0..50 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }
    println!("Simulation took {} s", start.elapsed().as_secs());

    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_2).unwrap();
}

fn task_2_1_3() {
    let start = Instant::now();

    let mut magnets = Array3::from_elem((1, 1, 100), Magnet::new(0.0, 0.0, 1.0));

    // Tilt the central magnet slightly less than 45 degrees
    magnets[(0, 0, 50)] = Magnet::new(0.9, 0.0, 1.0).normalize();

    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 0.0,
        coupling_constant: 10.0 * 1e-3,
        anisotropy_constant: 3.0 * 1e-3,
        temperature: 0.0,
        magnetic_field: 0.0 * E_Z,
        timestep: 1e-15,
    };
    let mut states = vec![sys.magnets.clone()];

    for _ in 0..400 {
        for _ in 0..4 {
            sys.step();
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }

    println!("Simulation took {} ms", start.elapsed().as_millis());

    let start = Instant::now();
    plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_1_3).unwrap();
    println!("Simulation took {} ms", start.elapsed().as_millis());
}

fn task_2_2_2() {
    let start = Instant::now();

    let mut magnets = Array3::from_elem((1, 1, 50), Magnet::new(0.0, 0.0, 1.0));

    let j = -10.0;
    let dz = 20.0;
    let a = 0.01;

    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 0.01,
        coupling_constant: j * 1e-3,
        anisotropy_constant: dz * 1e-3,
        temperature: 0.5 * 1e-3,
        magnetic_field: 0.0 * E_Z,
        timestep: 0.1 * 1e-15,
    };
    let mut states = vec![sys.magnets.clone()];
    let mut x_components: Vec<Vec<f64>> = vec![sys.magnets.iter().map(|mag| mag.x).collect()];

    // 60 000 * 0.5 fs = 30 ps
    // 300 000 * 0.1 fs = 30 ps
    for _ in 0..300_000 {
        for _ in 0..1 {
            sys.step();
            x_components.push(sys.magnets.iter().map(|mag| mag.x).collect());
        }
        states.push(sys.magnets.clone());
        // dbg!(&states);
    }
    println!("Took {:.2} s", start.elapsed().as_millis() as f64 / 1000.0);

    // plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_2_2_1).unwrap();
    std::fs::write(
        &format!("plots/x_components_J{j:.0}_dz{dz:.0}_a{a}.json"),
        serde_json::to_string_pretty(&x_components).expect("Cant jsonify"),
    )
    .expect("cant write json to file");
}

fn task_2_2_5() {
    let start = Instant::now();

    let j = 10.0;
    for b in [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0] {
        let mut magnets = Array3::from_elem((1, 1, 50), Magnet::new(0.0, 0.0, 1.0));
        let mut sys = MagneticSystem {
            magnets,
            dampening_constant: 0.01,
            coupling_constant: j * 1e-3,
            anisotropy_constant: 3.0 * 1e-3,
            temperature: 0.5 * 1e-3,
            magnetic_field: b * 1e-3 * E_Z,
            timestep: 0.1 * 1e-15,
        };
        let mut x_components: Vec<Vec<f64>> = vec![sys.magnets.iter().map(|mag| mag.x).collect()];

        // 60 000 * 0.5 fs = 30 ps
        // 300 000 * 0.1 fs = 30 ps
        for _ in 0..300_000 {
            sys.step();
            x_components.push(sys.magnets.iter().map(|mag| mag.x).collect());
        }

        println!(
            "finished {b} after {:.2}",
            start.elapsed().as_millis() as f64 / 1000.0
        );

        // plot_system(&states, "testplot.gif", 100, PlotDirection::Task2_2_2_1).unwrap();
        std::fs::write(
            &format!("plots/x_w_mag_j{j}_b{b:.1}.json"),
            serde_json::to_string_pretty(&x_components).expect("Cant jsonify"),
        )
        .expect("cant write json to file");
    }
}

fn task_2_2_6() {
    let start = Instant::now();

    let j = -30.0;
    for b in [0.0, 0.05] {
        let mut magnets = Array3::from_elem((1, 1, 50), Magnet::new(0.0, 0.0, 1.0));

        for (idx, mut magnet) in magnets.iter_mut().enumerate() {
            if idx % 2 == 0 {
                continue;
            }
            magnet.z *= -1.0;
        }

        let mut sys = MagneticSystem {
            magnets,
            dampening_constant: 0.01,
            coupling_constant: j * 1e-3,
            anisotropy_constant: 0.01 * 1e-3,
            temperature: 0.005 * 1e-3,
            magnetic_field: b * j * E_Z,
            timestep: 0.1 * 1e-15,
        };
        let mut states = vec![sys.magnets.clone()];
        let mut x_components: Vec<Vec<f64>> = vec![sys.magnets.iter().map(|mag| mag.x).collect()];

        // 300 000 * 0.1 fs = 30 ps
        for _ in 0..300_000 {
            sys.step();
            x_components.push(sys.magnets.iter().map(|mag| mag.x).collect());
        }

        println!(
            "finished {b} after {:.2}",
            start.elapsed().as_millis() as f64 / 1000.0
        );

        std::fs::write(
            &format!("plots/x_afm_j{j}_b{b}j.json"),
            serde_json::to_string_pretty(&x_components).expect("Cant jsonify"),
        )
        .expect("cant write json to file");
    }
}

fn task_2_3_1() {
    let start = Instant::now();
    let magnets = {
        let normal = Normal::new(0.0, 1.0).unwrap();
        Array3::from_shape_fn((20, 20, 20), |_| {
            let magnet = Magnet::from_fn(|_, _| normal.sample(&mut thread_rng()));
            magnet.normalize()
        })
    };

    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 1.0,
        coupling_constant: 10.0 * 1e-3,
        anisotropy_constant: 1.0 * 1e-3,
        temperature: 0.0 * 1e-3,
        magnetic_field: 0.0 * E_Z,
        timestep: 1.0 * 1e-15,
    };

    let mut states = vec![];
    let num_mags = sys.magnets.len() as f64;
    let summy: f64 = sys.magnets.iter().map(|mag| mag.z).sum();
    let mut ms: Vec<f64> = vec![summy / num_mags];

    // 60 000 * 0.5 fs = 30 ps
    for _ in 0..100 {
        for _ in 0..100 {
            sys.step();
            let summy: f64 = sys.magnets.iter().map(|mag| mag.z).sum();
            ms.push(summy / num_mags);
        }
        states.push(sys.magnets.clone());
    }

    println!("Finished sim in {} sec", start.elapsed().as_secs());

    plot_system(&states, "task_2_3_1.gif", 100, PlotDirection::Task2_3_1).unwrap();

    std::fs::write(
        "plots/avgs_2_3_1_tmp.json",
        serde_json::to_string(&ms).expect("Cant jsonify"),
    )
    .expect("cant write json to file");
}

fn task_2_3_2() {
    let start = Instant::now();
    let magnets = Array3::from_elem((20, 20, 20), Magnet::new(0.0, 0.0, 1.0));

    let mut sys = MagneticSystem {
        magnets,
        dampening_constant: 1.0,
        coupling_constant: 10.0 * 1e-3,
        anisotropy_constant: 10.0 * 1e-3,
        temperature: 1.0 * 1e-3,
        magnetic_field: 0.0 * E_Z,
        timestep: 1.0 * 1e-15,
    };
    let num_mags = sys.magnets.len() as f64;
    let summy: f64 = sys.magnets.iter().map(|mag| mag.z).sum();
    let mut ms: Vec<f64> = vec![summy / num_mags];

    // 10 000 * 1.0 fs = 10 ps
    for _ in 0..10_000 {
        sys.step();
        let summy: f64 = sys.magnets.iter().map(|mag| mag.z).sum();
        ms.push(summy / num_mags);
    }

    println!("Finished sim in {} sec", start.elapsed().as_secs());

    std::fs::write(
        "plots/avgs_2_3_2_tmp.json",
        serde_json::to_string(&ms).expect("Cant jsonify"),
    )
    .expect("cant write json to file");
}

fn task_2_3_3() {
    let start = Instant::now();
    let mut states = HashMap::new();
    for i in 0..=20 {
        // set kbt relative J using the i loop (it is impossible to loop over a range of floats in rust)
        let kbt = i as f64 * 1.0; // 20.0 * 0.1 = 2.0 J
        let magnets = Array3::from_elem((20, 20, 20), Magnet::new(0.0, 0.0, 1.0));

        let j = 10.0;

        let mut sys = MagneticSystem {
            magnets,
            dampening_constant: 1.0,
            coupling_constant: j * 1e-3,
            anisotropy_constant: 10.0 * 1e-3,
            temperature: kbt * j * 1e-3,
            magnetic_field: 0.0 * E_Z,
            timestep: 1.0 * 1e-15,
        };
        let num_mags = sys.magnets.len() as f64;
        let summy: f64 = sys.magnets.iter().map(|mag| mag.z).sum();
        let mut ms: Vec<f64> = vec![summy / num_mags];

        for _ in 0..10_000 {
            sys.step();
            let summy: f64 = sys.magnets.iter().map(|mag| mag.z).sum();
            ms.push(summy / num_mags);
        }

        states.insert(format!("{kbt:.2}"), ms);

        println!("Finished kBt {kbt:.2} in {} sec", start.elapsed().as_secs());
    }

    std::fs::write(
        "plots/avgs_2_3_3_10k.json",
        serde_json::to_string(&states).expect("Cant jsonify"),
    )
    .expect("cant write json to file");
}

fn main() {
    // testy();
    // task_2_1_1();
    // task_2_1_2();
    // task_2_1_3();
    // task_2_2_2();
    // task_2_2_5();
    // task_2_2_6();
    // task_2_3_1();
    // task_2_3_2();
    task_2_3_3();
}
