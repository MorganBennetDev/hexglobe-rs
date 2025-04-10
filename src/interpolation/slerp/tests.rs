use assert2::check;
use ntest::timeout;
use glam::Vec3;
use crate::interpolation::slerp::{slerp_n, sphere_exp, sphere_ln};

#[test]
fn exp() {
    std::panic::set_hook(Box::new(|_| {}));
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
    std::panic::set_hook(Box::new(|_| {}));
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
    std::panic::set_hook(Box::new(|_| {}));
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
    std::panic::set_hook(Box::new(|_| {}));
    let u = Vec3::new(1.0 / 3.0f32.sqrt(), 1.0 / 3.0f32.sqrt(), 1.0 / 3.0f32.sqrt());
    let v = Vec3::new(1.0 / 3.0f32.sqrt(), -1.0 / 3.0f32.sqrt(), 1.0 / 3.0f32.sqrt());
    let w = Vec3::new(1.0 / 3.0f32.sqrt(), 1.0 / 3.0f32.sqrt(), -1.0 / 3.0f32.sqrt());
    
    slerp_n(&[0.2, 0.3, 0.5], &[u, v, w]);
}
