use std::f32::consts::PI;
use assert2::check;
use ntest::timeout;
use glam::Vec3;
use crate::interpolation::slerp::{slerp_n, sphere_exp, sphere_ln};

#[test]
fn exp() {
    let q = Vec3::new(2.0, 1.0, -3.0).normalize();
    let v = q.any_orthogonal_vector();
    
    let u = sphere_exp(&q, &v);
    let d = u - q;
    
    check!((u.length() - 1.0).abs() <= f32::EPSILON, "Sphere exponential output should lie on the unit sphere.");
    check!((u.angle_between(q) - v.length()).abs() <= f32::EPSILON, "Sphere exponential should preserve distance.");
    check!(((d - q.dot(d) * q).normalize() - v.normalize()).abs().max_element() <= f32::EPSILON, "Sphere exponential should preserve direction.");
}

#[test]
fn ln() {
    let v = Vec3::new(1.0, 2.0, 1.0).normalize();
    let q = Vec3::new(2.0, 1.0, -3.0).normalize();
    let d = v - q;
    
    let u = sphere_ln(&q, &v);
    
    check!(u.dot(q).abs() <= f32::EPSILON, "Sphere natural logarithm output should lie on T_q(S^2).");
    check!((u.length() - v.angle_between(q)).abs() <= f32::EPSILON, "Sphere natural logarithm should preserve distance.");
    check!(((d - q.dot(d) * q).normalize() - u.normalize()).abs().max_element() <= f32::EPSILON, "Sphere natural logarithm should preserve direction.");
}

#[test]
fn ln_exp() {
    let q = Vec3::new(2.0, 1.0, -3.0).normalize();
    let v = q.any_orthogonal_vector();
    
    let u = sphere_ln(&q, &sphere_exp(&q, &v));
    
    check!((u - v).length() <= f32::EPSILON, "Sphere natural logarithm should reverse sphere exponential.");
    
    let v = Vec3::new(1.0, 2.0, -1.0).normalize();
    
    let u = sphere_exp(&q, &sphere_ln(&q, &v));
    
    check!((u - v).abs().max_element() <= f32::EPSILON, "Sphere exponential should reverse sphere natural logarithm.");
}

#[test]
#[timeout(1)]
fn halting() {
    let u = Vec3::new(1.0 / 3.0f32.sqrt(), 1.0 / 3.0f32.sqrt(), 1.0 / 3.0f32.sqrt());
    let v = Vec3::new(1.0 / 3.0f32.sqrt(), -1.0 / 3.0f32.sqrt(), 1.0 / 3.0f32.sqrt());
    let w = Vec3::new(1.0 / 3.0f32.sqrt(), 1.0 / 3.0f32.sqrt(), -1.0 / 3.0f32.sqrt());
    
    slerp_n(&[0.2, 0.3, 0.5], &[u, v, w]);
}

fn area(a: f32, b: f32, c: f32) -> f32 {
    // Spherical law of cosines
    let angle_a = ((a.cos() - b.cos() * c.cos()) / (b.sin() * c.sin())).acos();
    let angle_b = ((b.cos() - a.cos() * c.cos()) / (a.sin() * c.sin())).acos();
    let angle_c = ((c.cos() - a.cos() * b.cos()) / (a.sin() * b.sin())).acos();
    
    angle_a + angle_b + angle_c - PI
}

#[test]
fn slerp() {
    let u = Vec3::new(1.0, 2.0, 3.0).normalize();
    let v = Vec3::new(3.0, 2.0, 1.0).normalize();
    let w = Vec3::new(2.0, 1.0, 3.0).normalize();
    
    let w0 = 0.2;
    let w1 = 0.3;
    let w2 = 0.5;
    let p = slerp_n(&[w0, w1, w2], &[u, v, w]);
    
    let a = v.angle_between(w);
    let b = w.angle_between(u);
    let c = u.angle_between(v);
    
    let p_a = p.angle_between(u);
    let p_b = p.angle_between(v);
    let p_c = p.angle_between(w);
    
    let t = area(a, b, c);
    
    check!((area(a, p_b, p_c) - t * w0).abs() < 1.0e-2, "Ratio of area to total area is incorrect.");
    check!((area(b, p_a, p_c) - t * w1).abs() < 1.0e-2, "Ratio of area to total area is incorrect.");
    check!((area(c, p_a, p_b) - t * w2).abs() < 1.0e-2, "Ratio of area to total area is incorrect.");
    
}
