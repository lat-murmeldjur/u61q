use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::thread;

use crate::f32_3::dd_f32_3;
use crate::f64_3::mltply_f64_3;
use crate::magma_ocean::{magma, petrify, Stone};
use crate::positions::move_positions;
use crate::u_modular::modular_offset_in_range;

pub static TS_F64: f64 = 5.391247 * 1e-44;
pub static LS_F64: f64 = 299792458.0 * 1000000000.0 * 6.1879273537329 * 1e+25;

pub struct Anomaly {
    pub anomaly: Vec<Anomaly>,
    pub component: Vec<Component>,
    pub force: Vec<Force>,
}

pub struct Composition {
    pub space: Vec<[f32; 3]>,
    pub distribution: Vec<fn(Vec<[f32; 3]>) -> Vec<[f32; 3]>>,
}

pub struct Component {
    pub component: Vec<Component>,
    pub composition: Vec<Composition>,
    pub property: Vec<Property>,
}

pub struct Property {
    pub name: f64,
    pub value: f64,
}

pub struct Force {
    pub force: Vec<Force>,
    pub range: Vec<f64>,
    pub domain: Vec<Component>,
}

pub fn interact(anom: &mut Anomaly) {
    thread::scope(|s| {
        let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
        for mut a in anom.anomaly.iter_mut() {
            let handle = s.spawn(move || interact(&mut a));
            handles.push(handle);
        }
        for h in handles {
            h.join().unwrap();
        }
    });

    // access non-overlapping disjoint pairs of vector elements concurrently with social golfer problem solution
    // since interaction is bidirectional, ordering is irrelevant

    let klen = anom.anomaly.len();
    let kl2 = klen / 2;
    let mut sym = false;
    if klen % 2 == 0 {
        sym = true;
    }
    for i in 1..=kl2 {
        let mut done: HashSet<usize> = HashSet::new();
        let mut exit_level = false;

        while !exit_level {
            let mut pull: HashMap<usize, usize> = HashMap::new();
            let mut skip: HashSet<usize> = HashSet::new();

            let mut firstopen: usize = 0;
            let mut instructions_chan: Vec<mpsc::Sender<&mut Anomaly>> = vec![];

            for k in 0..anom.anomaly.len() {
                let pair = modular_offset_in_range(k as u32, i as u32, 0, (klen - 1) as u32);
                if !done.contains(&k) && !skip.contains(&k) && !skip.contains(&(pair as usize)) {
                    pull.insert(k, firstopen);
                    pull.insert(pair as usize, firstopen);
                    firstopen += 1;
                    done.insert(k);
                    skip.insert(k);
                    skip.insert(pair as usize);
                    if sym && i == kl2 {
                        done.insert(pair as usize);
                    }
                }
            }

            thread::scope(|s| {
                let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
                for _ in 0..pull.len() / 2 {
                    let (tx, rx) = mpsc::channel();
                    instructions_chan.push(tx);
                    let handle = s.spawn(move || {
                        let mut a = rx.recv().unwrap();
                        let mut b = rx.recv().unwrap();
                        anomaly_2_interact(&mut a, &mut b);
                    });
                    handles.push(handle);
                }

                //

                for (k, a) in anom.anomaly.iter_mut().enumerate() {
                    if pull.contains_key(&k) {
                        instructions_chan[pull[&k]].send(a).unwrap();
                    }
                }

                for h in handles {
                    h.join().unwrap();
                }
            });

            if done.len() == klen {
                exit_level = true;
            }
        }
    }

    component_interact(anom);
}

//fn iter_chunks<T, const CHUNK_SIZE: usize>(
//    slice: &mut [T],
//) -> impl Iterator<Item = [&mut T; CHUNK_SIZE]> + '_ {
//    assert_eq!(slice.len() % CHUNK_SIZE, 0);
//    let len = slice.len();
//    let mut a: [_; CHUNK_SIZE] = array_collect(
//        slice
//            .chunks_mut(len / CHUNK_SIZE)
//            .map(|iter| iter.iter_mut()),
//    );
//    (0..len / CHUNK_SIZE).map(move |_| array_collect(a.iter_mut().map(|i| i.next().unwrap())))
//}

fn array_collect<T, const N: usize>(mut iter: impl Iterator<Item = T>) -> [T; N] {
    let a: [(); N] = [(); N];
    a.map(|_| iter.next().unwrap())
}

pub fn anomaly_2_interact(a: &mut Anomaly, b: &mut Anomaly) {
    for i in a.anomaly.iter_mut() {
        for j in b.anomaly.iter_mut() {
            anomaly_2_interact(i, j);
        }
    }

    for df in &a.force {
        for i in 0..a.component.len() {
            for j in 0..b.component.len() {
                component_2_interact(df, &mut a.component[i], &mut b.component[j]);
            }
        }
    }
}

pub fn component_interact(_anom: &mut Anomaly) {
    for df in &_anom.force {
        for i in 0.._anom.component.len() {
            for j in 0.._anom.component.len() {
                let (e, f) = if i < j {
                    let (left, right) = _anom.component.split_at_mut(j);
                    (&mut left[i], &mut right[0])
                } else if i == j {
                    continue;
                } else {
                    let (left, right) = _anom.component.split_at_mut(i);
                    (&mut right[0], &mut left[j])
                };
                component_2_interact(df, e, f);
            }
        }
    }
}

pub fn component_2_interact(df: &Force, a: &mut Component, b: &mut Component) {
    for i in a.component.iter_mut() {
        for j in b.component.iter_mut() {
            component_2_interact(df, i, j);
        }
    }

    force_apply(df, a, b);
}

pub fn force_apply(_f: &Force, a: &mut Component, b: &mut Component) {
    set_inertia([0.0, 0.0, 0.0], a);

    set_inertia([0.0, 0.0, 0.0], b);
}

pub fn progress(anom: &mut Anomaly, time: f64) {
    thread::scope(|s| {
        let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
        for mut a in anom.anomaly.iter_mut() {
            let handle = s.spawn(move || {
                interact(&mut a);
                progress(&mut a, time);
            });
            handles.push(handle);
        }
        for h in handles {
            h.join().unwrap();
        }
    });

    for i in 0..anom.anomaly.len() {
        for j in 0..anom.anomaly.len() {
            let (e, f) = if i < j {
                let (left, right) = anom.anomaly.split_at_mut(j);
                (&mut left[i], &mut right[0])
            } else if i == j {
                continue;
            } else {
                let (left, right) = anom.anomaly.split_at_mut(i);
                (&mut right[0], &mut left[j])
            };
            anomaly_2_interact(e, f);
        }
    }

    let steps = (time / TS_F64) as u64;
    for _ in 0..steps {
        thread::scope(|s| {
            let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
            for mut c in anom.component.iter_mut() {
                let handle = s.spawn(move || {
                    component_progress(&mut c, TS_F64);
                });
                handles.push(handle);
            }
            for h in handles {
                h.join().unwrap();
            }
        });
    }
}

pub fn component_property(component: &mut Component, name: f64) -> f64 {
    let prop: Vec<&mut Property> = component
        .property
        .iter_mut()
        .filter(|c| c.name == name)
        .collect();

    return prop[0].value;
}

pub fn set_component_property(n: f64, s: f64, component: &mut Component) {
    for p in component.property.iter_mut() {
        if n == p.name {
            *p = Property { name: n, value: s };
        }
    }
}

pub fn set_inertia(in0: [f64; 3], c: &mut Component) {
    set_component_property(IN0, in0[0], c);
    set_component_property(IN1, in0[1], c);
    set_component_property(IN2, in0[2], c);
}

pub fn component_progress(component: &mut Component, time: f64) {
    for mut c in component.component.iter_mut() {
        component_progress(&mut c, time);
    }

    let inertia_0 = component_property(component, IN0);
    let inertia_1 = component_property(component, IN1);
    let inertia_2 = component_property(component, IN2);

    for c in &mut component.composition {
        for s in c.space.iter_mut() {
            let mov0 = mltply_f64_3([inertia_0, inertia_1, inertia_2], TS_F64);
            *s = dd_f32_3(*s, [mov0[0] as f32, mov0[1] as f32, mov0[2] as f32]);
        }
    }
}

pub fn view(anom: &mut Anomaly) -> Vec<Stone> {
    let mut ret: Vec<Stone> = vec![];
    let mut rs: Vec<mpsc::Receiver<Vec<Stone>>> = vec![];

    thread::scope(|s| {
        for mut a in anom.anomaly.iter_mut() {
            let (tx, rx) = mpsc::channel();
            rs.push(rx);
            s.spawn(move || {
                let k = view(&mut a);
                tx.send(k).unwrap();
            });
        }
    });

    for c in anom.component.iter_mut() {
        ret.append(&mut component_view(c));
    }

    for r in rs {
        let mut rec = r.recv().unwrap();
        ret.append(&mut rec);
    }

    ret
}

pub fn component_view(component: &mut Component) -> Vec<Stone> {
    let mut ret: Vec<Stone> = vec![];

    for c in component.component.iter_mut() {
        ret.append(&mut component_view(c));
    }

    let size = component_property(component, MS);

    for c in &component.composition {
        for d in &c.distribution {
            for v in &d(c.space.clone()) {
                let mut s = petrify(magma(2, size as f32));
                move_positions(&mut s.positions, *v);
                ret.push(s);
            }
        }
    }
    ret
}

pub fn add_particle_by(anom: &mut Anomaly, p: Anomaly) {
    anom.anomaly.push(p);
}

pub fn particle(position: [f32; 3], properties: Vec<Property>) -> Anomaly {
    let anom = Anomaly {
        anomaly: vec![],
        component: vec![Component {
            component: vec![],
            composition: vec![Composition {
                space: vec![position],
                distribution: vec![particular],
            }],
            property: properties,
        }],
        force: force_base().force,
    };

    anom
}

static EC: f64 = 313.0;
static SP: f64 = 591.0;
static MS: f64 = 343.0;
static CR: f64 = 0.10;
static IN0: f64 = 141.0;
static IN1: f64 = 141.1;
static IN2: f64 = 141.2;
static QMS: [f64; 6] = [2.2, 4.7, 1.28, 96.0, 173.1, 4.18];

pub fn e(position: [f32; 3], inertia: [f64; 3], clock: bool) -> Anomaly {
    let sp = if clock { 0.5 } else { -0.5 };
    particle(
        position,
        vec![
            Property {
                name: SP,
                value: sp,
            },
            Property {
                name: EC,
                value: -1.0,
            },
            Property {
                name: MS,
                value: 0.511,
            },
            Property {
                name: IN0,
                value: inertia[0],
            },
            Property {
                name: IN1,
                value: inertia[1],
            },
            Property {
                name: IN2,
                value: inertia[2],
            },
        ],
    )
}

pub fn q(
    position: [f32; 3],
    inertia: [f64; 3],
    clock: bool,
    charge: bool,
    color: u8,
    flavor: u8,
) -> Anomaly {
    let sp = if clock { 0.5 } else { -0.5 };
    let ch = if charge { 2.0 / 3.0 } else { -1.0 / 3.0 };

    particle(
        position,
        vec![
            Property {
                name: SP,
                value: sp,
            },
            Property {
                name: EC,
                value: ch,
            },
            Property {
                name: MS,
                value: QMS[(flavor % 6) as usize],
            },
            Property {
                name: CR,
                value: (color % 6) as f64,
            },
            Property {
                name: IN0,
                value: inertia[0],
            },
            Property {
                name: IN1,
                value: inertia[1],
            },
            Property {
                name: IN2,
                value: inertia[2],
            },
        ],
    )
}

pub fn particular(coordinates: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    return coordinates;
}

pub fn force_base() -> Force {
    return Force {
        force: vec![
            Force {
                force: vec![],
                range: vec![1e-15],
                domain: vec![Component {
                    component: vec![],
                    composition: vec![],
                    property: vec![Property {
                        name: CR,
                        value: 1.0,
                    }],
                }],
            },
            Force {
                force: vec![],
                range: vec![f64::MAX],
                domain: vec![Component {
                    component: vec![],
                    composition: vec![],
                    property: vec![Property {
                        name: EC,
                        value: 1.0 / 137.0,
                    }],
                }],
            },
            Force {
                force: vec![
                    Force {
                        force: vec![],
                        range: vec![1e-18],
                        domain: vec![Component {
                            component: vec![],
                            composition: vec![],
                            property: vec![Property {
                                name: MS,
                                value: 1e-13,
                            }],
                        }],
                    },
                    Force {
                        force: vec![],
                        range: vec![1e-18],
                        domain: vec![Component {
                            component: vec![],
                            composition: vec![],
                            property: vec![Property {
                                name: SP,
                                value: 1e-13,
                            }],
                        }],
                    },
                ],
                range: vec![],
                domain: vec![],
            },
            Force {
                force: vec![],
                range: vec![f64::MAX],
                domain: vec![Component {
                    component: vec![],
                    composition: vec![],
                    property: vec![Property {
                        name: MS,
                        value: 1e-41,
                    }],
                }],
            },
        ],
        range: vec![],
        domain: vec![],
    };
}
