use crate::{vector::Vector3, Real};

#[derive(Debug, Default, Clone)]
pub struct Particle {
    /// Linear position of the particle in world space
    position: Vector3,
    /// Linear velocity of the particle in world space
    velocity: Vector3,
    /// Acceleration of the particle; useful for things like gravity
    acceleration: Vector3,
    /// The amount of damping applied to linear motion. Required to remove energy added through
    /// numerical instability
    damping: Real,
    /// Holds the inverse of the mass of the particle. It's more useful to hold the inverse mass
    /// because integration is simpler.
    inverse_mass: Real,
}

impl Particle {
    pub fn integrate(&mut self, dt: Real) {
        // Infinite mass? No THANK you.
        if self.inverse_mass <= 0.0 {
            return;
        };

        // No time at all
        if dt <= 0.0 {
            return;
        }

        // Update linear position from velocitu
        self.position = self.position.add_scaled(self.velocity, dt);

        // Update velocity from acceleration
        self.velocity = self.velocity.add_scaled(self.acceleration, dt);

        // Apply damping
        self.velocity = self.velocity * self.damping.powf(dt);
    }

    pub fn set_velocity(&mut self, new_velocity: Vector3) {
        self.velocity = new_velocity;
    }

    pub fn velocity(&self) -> Vector3 {
        self.velocity
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn set_mass(&mut self, mass: Real) {
        self.inverse_mass = 1.0 / mass;
    }

    pub fn mass(&self) -> Real {
        1.0 / self.inverse_mass
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector3;

    use super::Particle;

    #[test]
    pub fn test_set_mass() {
        let mut particle_a = Particle::default();
        particle_a.set_mass(2.0);
        assert_eq!(particle_a.mass(), 2.0);
    }

    #[test]
    pub fn test_integrate() {
        let dt = 1.0 / 60.0;
        let mut particle_a = Particle::default();
        particle_a.set_mass(2.0);
        particle_a.damping = 0.999; // Add reasonable damping value
        particle_a.velocity = Vector3::new(1.0, 1.0, 1.0);
        particle_a.acceleration = Vector3::new(0.5, 0.0, -0.5);
        particle_a.position = Vector3::new(0.0, 0.0, 0.0);

        particle_a.integrate(dt);

        // Calculate expected results
        let damping_factor = 0.999f32.powf(dt);
        let expected_velocity = Vector3::new(
            (1.0 + 0.5 * dt) * damping_factor,
            (1.0 + 0.0 * dt) * damping_factor,
            (1.0 - 0.5 * dt) * damping_factor,
        );
        let expected_position = Vector3::new(0.0 + (1.0 * dt), 0.0 + (1.0 * dt), 0.0 + (1.0 * dt));

        assert_eq!(
            particle_a.velocity, expected_velocity,
            "Velocity did not integrate correctly!"
        );
        assert_eq!(
            particle_a.position, expected_position,
            "Position did not integrate correctly!"
        );
    }
}
