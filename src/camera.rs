use raylib::prelude::*;
use std::f32::consts::PI;

pub struct CustomCamera {
    pub eye: Vector3,    // Camera position in world space
    pub center: Vector3, // Point the camera is looking at
    pub up: Vector3,     // Up vector
}

impl CustomCamera {
    pub fn new(eye: Vector3, center: Vector3, up: Vector3) -> Self {
        CustomCamera {
            eye,
            center,
            up,
        }
    }

    pub fn basis_change(&self, vector: &Vector3) -> Vector3 {
        let mut forward = self.center - self.eye;
        forward.normalize();
        let mut right = forward.cross(self.up);
        right.normalize();
        let up = right.cross(forward);

        let mut rotated = Vector3::new(
            vector.x * right.x + vector.y * up.x - vector.z * forward.x,
            vector.x * right.y + vector.y * up.y - vector.z * forward.y,
            vector.x * right.z + vector.y * up.z - vector.z * forward.z,
        );

        rotated.normalize();
        rotated
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // Calculate the vector from the center to the eye (radius vector) and measure the distance
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.length();

        // Calculate current yaw (rotation around Y-axis)
        // atan2(z, x) gives us the angle in the XZ plane
        // Range: [-π, π], where 0 is along positive X-axis, π/2 is along positive Z-axis
        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        // Calculate current pitch (rotation around X-axis)
        // xz here refers to the proyection of the radius over the x axis
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        // We use -y because positive pitch is when we look up (negative y in our coordinate system)
        // Range: [-π/2, π/2], where 0 is horizontal, π/2 is looking straight up
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Apply delta rotations
        // Keep yaw in range [0, 2π] for consistency
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        // Clamp pitch to slightly less than [-π/2, π/2] to prevent gimbal lock
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        // Calculate new eye position
        // We use spherical coordinates to cartesian conversion:
        // x = r * cos(yaw) * cos(pitch)
        // y = -r * sin(pitch)  // Negative because positive y is up
        // z = r * sin(yaw) * cos(pitch)

        let new_eye = self.center + Vector3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos()
        );

        self.eye = new_eye;
    }
}