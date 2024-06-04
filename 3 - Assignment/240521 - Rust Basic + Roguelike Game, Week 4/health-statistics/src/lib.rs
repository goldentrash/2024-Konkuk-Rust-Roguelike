pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        let Measurements {
            height,
            blood_pressure,
        } = measurements;

        let height_change = height - self.height;
        let blood_pressure_change = if let Some(last_blood_pressure) = self.last_blood_pressure {
            Some((
                measurements.blood_pressure.0 as i32 - last_blood_pressure.0 as i32,
                measurements.blood_pressure.1 as i32 - last_blood_pressure.1 as i32,
            ))
        } else {
            None
        };

        self.height = height;
        self.visit_count += 1;
        self.last_blood_pressure = Some(blood_pressure);

        HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count as u32,
            height_change,
            blood_pressure_change,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visit() {
        let mut bob = User::new(String::from("Bob"), 32, 155.2);
        assert_eq!(bob.visit_count, 0);

        let report = bob.visit_doctor(Measurements {
            height: 156.1,
            blood_pressure: (120, 80),
        });
        assert_eq!(report.patient_name, "Bob");
        assert_eq!(report.visit_count, 1);
        assert_eq!(report.blood_pressure_change, None);

        let report = bob.visit_doctor(Measurements {
            height: 156.1,
            blood_pressure: (115, 76),
        });
        assert_eq!(report.visit_count, 2);
        assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    }
}
