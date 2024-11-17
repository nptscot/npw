/// Given stats about a route, calculate its "uptake", between 0 and 1.
// Ported from
// https://github.com/itsleeds/pct/blob/e630464efeaef539b18647b10745b863c9cd9948/R/uptake.R#L216
pub fn pct_godutch_2020(distance_meters: f64) -> f64 {
    // TODO Find a data source and calculate this
    let gradient_percent = 0.0;

    let alpha = -4.018 + 2.550;
    let d1 = -0.6369 - 0.08036;
    let d2 = 1.988;
    let d3 = 0.008775;
    let h1 = -0.2555;
    let h2 = -0.78;
    let i1 = 0.02006;
    let i2 = -0.1234;

    let gradient_percent = gradient_percent + h2;
    let distance_km = (distance_meters / 1000.0).min(30.0);

    let p = alpha
        + (d1 * distance_km)
        + (d2 * distance_km.sqrt())
        + (d3 * distance_km.powi(2))
        + (h1 * gradient_percent)
        + (i1 * distance_km * gradient_percent)
        + (i2 * distance_km.sqrt() * gradient_percent);
    inverse_logit(p)
}

fn inverse_logit(p: f64) -> f64 {
    let result = p.exp() / (1.0 + p.exp());
    if result < 0.0 || result > 1.0 {
        panic!("inverse_logit({p}) = {result}, which isn't between 0 and 1");
    }
    result
}
