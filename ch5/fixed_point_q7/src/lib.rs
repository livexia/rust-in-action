#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(value: f64) -> Self {
        if value >= 1.0 {
            Q7(127)
        } else if value <= -1.0 {
            Q7(-128)
        } else {
            Q7((value * 128.0) as i8)
        }
    }
}

impl From<f32> for Q7 {
    fn from(value: f32) -> Self {
        Q7::from(value as f64)
    }
}

impl From<Q7> for f64 {
    fn from(value: Q7) -> Self {
        (value.0 as f64) * 2f64.powf(-7.0)
    }
}

impl From<Q7> for f32 {
    fn from(value: Q7) -> Self {
        f64::from(value) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.9;
        let q1: Q7 = n1.into();

        let n2 = -0.5;
        let q2 = Q7::from(n2);

        let n3 = 123.456;
        let q3: Q7 = n3.into();

        assert_eq!(q1, Q7(115));
        assert_eq!(q2, Q7(-64));
        assert_eq!(q3, Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1: Q7 = 0.9f32.into();
        let n1 = f32::from(q1);
        assert!((n1 - 0.9).abs() < 0.01);

        let q2 = Q7::from(n1);
        let n2: f32 = q2.into();
        assert_eq!(n1, n2);
    }
}
