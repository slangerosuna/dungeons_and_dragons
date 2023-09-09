pub use crate::physics::*;

pub struct RigidBody {
    pub position: Vector3,
    pub velocity: Vector3,

    pub rotation: Matrix4,
    pub angular_velocity: Vector3,

    pub mass: f32,
    pub inverse_mass: f32,

    pub inertia_tensor: Matrix4,
    
    pub force_accumulator: Vector3,
    pub torque_accumulator: Vector3,
    
    pub impulse_accumulator: Vector3,
    pub angular_impulse_accumulator: Vector3,

    //-drag coeff * 0.5 * density of air 
    //* cross sectional area
    pub drag_super_constant: f32,

    pub coll: Option<Collider>,
}

impl RigidBody {
    //TODO implement code for reversing
    
    //TODO code to interact with colliders

    pub fn new
        (position: Vector3, rotation: Matrix4, mass: f32, 
         inertia_tensor: Matrix4, drag_super_constant: f32, 
         coll: Option<Collider>) -> RigidBody {
            RigidBody {
                position: position,
                velocity: 
                    Vector3::new(0.0, 0.0, 0.0),

                rotation: rotation,
                angular_velocity: 
                    Vector3::new(0.0, 0.0, 0.0),

                mass: mass,
                inverse_mass: 1.0 / mass,

                inertia_tensor: inertia_tensor,

                force_accumulator: 
                    Vector3::new(0.0, 0.0, 0.0),
                torque_accumulator: 
                    Vector3::new(0.0, 0.0, 0.0),

                impulse_accumulator: 
                    Vector3::new(0.0, 0.0, 0.0),
                angular_impulse_accumulator: 
                    Vector3::new(0.0, 0.0, 0.0),

                drag_super_constant: drag_super_constant,

                coll: coll,
        }
    }

    pub fn calc_gravity_force(&self) -> Vector3 {
        Vector3::new(0.0, -9.81 * self.mass, 0.0)
    }

    pub fn calc_drag_force(&self) -> Vector3 {
        self.velocity
            .multiply(self.velocity.magnitude() 
                      * self.drag_super_constant)
    }

    pub fn add_force(&mut self, force: Vector3) {
        self.force_accumulator = 
            self.force_accumulator.add(&force);
    }
    
    pub fn add_torque(&mut self, torque: Vector3) {
        self.torque_accumulator = 
            self.torque_accumulator.add(&torque);
    }

    pub fn add_force_displaced_from_center
        (&mut self, force: Vector3, displacement: Vector3) {
            self.force_accumulator = 
                self.force_accumulator.add(&force);
            self.torque_accumulator = 
                self.torque_accumulator
                .add(&displacement.cross(&force));
    }

    pub fn add_impulse
        (&mut self, impulse: Vector3) {
            self.impulse_accumulator = 
                self.impulse_accumulator.add(&impulse);
    }

    pub fn add_angular_impulse
        (&mut self, angular_impulse: Vector3) {
            self.angular_impulse_accumulator = 
                self.angular_impulse_accumulator
                .add(&angular_impulse);
    }

    pub fn add_impulse_displaced_from_center
        (&mut self, impulse: Vector3, displacement: Vector3) {
            self.impulse_accumulator = 
                self.impulse_accumulator.add(&impulse);
            self.angular_impulse_accumulator = 
                self.angular_impulse_accumulator
                .add(&displacement.cross(&impulse));
    }

    pub fn adv_timestep
        (&mut self, dt: f32) {
            //impulses applied instantly to prevent 
            //multiple frames of overlap on collision
            self.velocity = 
                self.velocity
                .add(&self.impulse_accumulator
                     .multiply(self.inverse_mass));
            self.angular_velocity = 
                self.angular_velocity
                .add(&self.inertia_tensor
                     .multiply_vector(
                         &self.angular_impulse_accumulator));
            self.impulse_accumulator = 
                Vector3::new(0.0, 0.0, 0.0);
            self.angular_impulse_accumulator = 
                Vector3::new(0.0, 0.0, 0.0);

            let prev_velocity = self.velocity;

            //uses left hand approximation for ease of use 
            //due to relatively constant acceleration
            self.velocity = 
                self.velocity
                .add(&self.force_accumulator
                    .multiply(dt * self.inverse_mass));
            
            //uses trapazoidal approximation for accuracy 
            //due to quite variable velocity
            self.position = 
                self.position
                    .add(&self.velocity.multiply(dt / 2.0)
                    .add(&prev_velocity.multiply(dt / 2.0)));

            let prev_angular_velocity = self.angular_velocity;

            //uses left hand approximation for ease of use 
            //due to relatively constant angular acceleration
            self.angular_velocity = 
                self.angular_velocity
                    .add(&self.inertia_tensor
                         .multiply_vector(
                             &self.torque_accumulator))
                    .multiply(dt);
            
            //uses trapazoidal approximation for accuracy 
            //due to variable angular_velocity
            let average_velocity = 
                self.angular_velocity
                    .add(&prev_angular_velocity)
                    .multiply(0.5);
            self.rotation = 
                self.rotation
                    .multiply(&Matrix4::axis_angle(
                        //multiply by inverse to convert 
                        //from world space to local space
                        self.rotation.inverse()
                            .multiply_vector(&average_velocity
                                                .normalized()), 
                        average_velocity.magnitude() * dt));

            self.force_accumulator = 
                Vector3::new(0.0, 0.0, 0.0);
            self.torque_accumulator = 
                Vector3::new(0.0, 0.0, 0.0);
    }
}
