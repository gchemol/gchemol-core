// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*imports][imports:1]]
use crate::{Atom, Molecule};
use gut::prelude::*;

use std::collections::HashMap;
// imports:1 ends here

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*core][core:1]]
type Bounds = HashMap<(usize, usize), f64>;

// return distance bounds between atoms
// upper-tri for upper bounds
// lower-tri for lower bounds
fn get_distance_bounds_v1(mol: &Molecule) -> Bounds {
    // max distance between two atoms
    let max_rij = 90.0;

    let sns: Vec<_> = mol.serial_numbers().collect();
    let nnodes = sns.len();

    let mut bounds = HashMap::new();
    for p in sns.into_iter().combinations(2) {
        let (i, j) = (p[0], p[1]);
        let (atom_i, atom_j) = (mol.get_atom(i).unwrap(), mol.get_atom(j).unwrap());
        // use vdw radii as the lower bound for non-bonded pair
        let vri = atom_i.get_vdw_radius().unwrap();
        let vrj = atom_j.get_vdw_radius().unwrap();
        let vrij = vri + vrj;

        // use covalent radii as the lower bound for bonded pair
        let cri = atom_i.get_cov_radius().unwrap();
        let crj = atom_j.get_cov_radius().unwrap();
        let crij = cri + crj;

        let lij = crij * 0.8;
        let uij = vrij * 0.8;
        let uij = if uij > crij * 1.2 { uij } else { crij * 1.2 };
        debug_assert!(lij <= uij);

        // if i and j is directly bonded
        // set covalent radius as the lower bound
        // or set vdw radius as the lower bound if not bonded
        let dij = atom_i.distance(atom_j);
        if let Some(nb) = mol.nbonds_between(i, j) {
            if nb == 1 {
                if dij >= lij && dij < crij * 1.2 {
                    bounds.insert((i, j), dij);
                    bounds.insert((j, i), dij);
                } else {
                    bounds.insert((i, j), lij);
                    bounds.insert((j, i), lij);
                }
            } else if nb == 2 {
                if dij > uij && dij < max_rij {
                    bounds.insert((i, j), dij);
                    bounds.insert((j, i), dij);
                } else {
                    bounds.insert((i, j), uij);
                    bounds.insert((j, i), uij + dij);
                }
            } else {
                if dij > uij && dij < max_rij {
                    bounds.insert((i, j), dij);
                    bounds.insert((j, i), max_rij);
                } else {
                    bounds.insert((i, j), uij);
                    bounds.insert((j, i), max_rij);
                }
            }
        } else {
            bounds.insert((i, j), uij);
            bounds.insert((j, i), max_rij);
        }
    }
    bounds
}
// core:1 ends here

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*core][core:1]]
#[inline]
/// Return Cartesian distance between two points in 3D space.
fn euclidean_distance(p1: [f64; 3], p2: [f64; 3]) -> f64 {
    let mut d2 = 0.0;
    for v in 0..3 {
        let dv = p2[v] - p1[v];
        d2 += dv * dv;
    }

    d2.sqrt()
}

// the weight between two atoms
fn get_weight_between(lij: f64, uij: f64, dij: f64) -> f64 {
    debug_assert!(lij <= uij);

    let weight = if dij >= lij && dij < uij {
        // avoid dividing by zero (nan)
        1e-4
    } else if dij < lij {
        1.0
    } else {
        1.0
    };

    weight
}

impl Molecule {
    /// Clean up molecule geometry using stress majorization algorithm
    pub fn clean(&mut self) -> Result<()> {
        let bounds = get_distance_bounds_v1(&self);
        let node_indices: Vec<_> = self.serial_numbers().collect();
        let nnodes = node_indices.len();

        let maxcycle = nnodes * 100;
        let ecut = 1E-4;
        let mut icycle = 0;
        let mut old_stress = 0.0;
        loop {
            let mut stress = 0.0;
            let mut positions_new = vec![];
            for i in 0..nnodes {
                let node_i = node_indices[i];
                let mut pi = self.get_atom(node_i).expect("atom i from node_i").position();
                let mut pi_new = [0.0; 3];
                let mut wijs = vec![];
                let npairs = (nnodes - 1) as f64;
                let mut stress_i = 0.0;
                for j in 0..nnodes {
                    // skip self-interaction
                    if i == j {
                        continue;
                    };

                    let node_j = node_indices[j];
                    let pj = self.get_atom(node_j).expect("atom j from node_j").position();

                    // current distance
                    let cur_dij = euclidean_distance(pi, pj);

                    // lower bound and upper bound for pair distance
                    let lij = if node_i < node_j {
                        bounds[&(node_i, node_j)]
                    } else {
                        bounds[&(node_j, node_i)]
                    };
                    // let uij = bounds[&(node_j, node_i)];
                    let uij = if node_i < node_j {
                        bounds[&(node_j, node_i)]
                    } else {
                        bounds[&(node_i, node_j)]
                    };
                    // let (lij, uij) = (lij.min(uij), uij.max(lij));

                    // ij pair counts twice, so divide the weight
                    // let wij = lij.powi(-4);
                    let wij = get_weight_between(lij, uij, cur_dij);
                    // dbg!((wij, lij, uij, cur_dij));
                    let wij = 0.5 * wij;
                    wijs.push(wij);

                    // collect position contribution of atom j to atom i
                    let xij = [pi[0] - pj[0], pi[1] - pj[1], pi[2] - pj[2]];
                    let mut fij = [0.0; 3];
                    for v in 0..3 {
                        pi_new[v] += wij * (pj[v] + lij / cur_dij * (pi[v] - pj[v]));
                        fij[v] = (1.0 - lij / cur_dij) * xij[v];
                    }
                    stress_i += wij * (cur_dij - lij).powi(2);
                }

                // weight sum
                let swij: f64 = wijs.iter().sum();

                // FIXME: if all pair weights are zero
                // dbg!(swij);
                debug_assert!(swij.abs() >= 1e-4);
                for v in 0..3 {
                    pi_new[v] /= swij;
                }
                positions_new.push((node_i, pi_new));
                stress += stress_i;
            }

            debug!("cycle: {} energy = {:?}", icycle, stress);
            // println!("cycle: {} energy = {:?}", icycle, stress);

            // update positions
            for (node, position) in positions_new {
                self.set_position(node, position);
            }

            if stress.is_nan() {
                bail!("found invalid number: {:?}", stress);
            }

            if stress < ecut || (stress - old_stress).abs() < ecut || (stress - old_stress).abs() / stress < ecut {
                break;
            }

            icycle += 1;
            if icycle > maxcycle {
                break;
            }

            old_stress = stress;
        }

        Ok(())
    }
}
// core:1 ends here
