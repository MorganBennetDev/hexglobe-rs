use glam::Vec3;
use itertools::Itertools;

#[cfg(test)]
mod tests;

// Map from S^2 to T_q(S^2). Inverse of exp_q(p)
// Outputs a vector with magnitude equal to the angle between p and q in the direction from q to p tangent to S^2 at q.
fn sphere_ln(q: &Vec3, p: &Vec3) -> Vec3 {
    let r = p.angle_between(*q);
    
    let k = if r == 0.0 {
        1.0
    } else {
        r / r.sin()
    };
    
    k * (p - q * r.cos())
}

// Map from T_q(S^2) to S^2. Preserves distance and direction
fn sphere_exp(q: &Vec3, dp: &Vec3) -> Vec3 {
    let r = dp.length();
    
    let k = if r == 0.0 {
        1.0
    } else {
        r.sin() / r
    };
    
    q * r.cos() + k * dp
}

/// Performs weighted spherical linear interpolation on a set of `N` vectors all lying on the same sphere using the
/// local linear convergence algorithm (A1) described by Buss and Fillmore in [Spherical Averages and Applications
/// to Spherical Splines and Interpolation](https://mathweb.ucsd.edu/~sbuss/ResearchWeb/spheremean/paper.pdf).
/// I attempted to implement the quadratic convergence algorithm but was not able to do so in a way that led to
/// empirically better benchmarks. If you can manage this, contributions are always welcome.
pub fn slerp_n<const N: usize>(w: &[f32; N], p: &[Vec3; N]) -> Vec3 {
    let total_weight = w.iter().cloned().tree_reduce(|a, b| a + b);
    debug_assert!(total_weight.is_some(), "Sum of weights must exist.");
    debug_assert!((total_weight.unwrap() - 1.0) <= f32::EPSILON, "Sum of weights must be equal to 1.0.");
    
    let mut q = w.iter()
        .zip(p.iter())
        .map(|(w_i, p_i)| w_i * p_i)
        .tree_reduce(|v_i, v_j| v_i + v_j)
        .unwrap()
        .normalize();
    
    loop {
        let u = w.iter()
            .zip(p.iter())
            .map(|(w_i, p_i)| w_i * sphere_ln(&q, &p_i))
            .tree_reduce(|p_i, p_j| p_i + p_j)
            .unwrap();
        
        q = sphere_exp(&q, &u);
        
        if u.length() < 1.0e-6 {
            return q;
        }
    }
}

/// Shorthand for `slerp_n(&[w1, w2, w3], &[p1, p2, p3])`.
pub fn slerp_3(w1: f32, p1: Vec3, w2: f32, p2: Vec3, w3: f32, p3: Vec3) -> Vec3 {
    slerp_n(&[w1, w2, w3], &[p1, p2, p3])
}
