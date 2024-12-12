
use anyhow::Ok;
use log::info;

use ray_ray_a2::tuples::Tuple;
#[derive(Default, Debug, Clone)]
pub struct Projectile {
    pub p_position: Tuple, // Point
    pub v_velocity: Tuple, // Vector
}

impl Projectile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(mut self, p_position: impl Into<Tuple>) -> Self {
        self.p_position = p_position.into();
        self
    }

    pub fn with_velocity(mut self, v_velocity: impl Into<Tuple>) -> Self {
        self.v_velocity = v_velocity.into();
        self
    }

    pub fn tick(&mut self, env: &Enviornment) -> &mut Self {
        self.p_position = self.p_position + self.v_velocity;
        self.v_velocity = self.v_velocity + env.v_gravity + env.v_wind;
        self
    }
}
#[derive(Debug, Clone)]
pub struct Enviornment {
    pub v_gravity: Tuple, // Vector
    pub v_wind: Tuple,    // Vector
}

impl Enviornment {
    pub fn new_no_wind() -> Self {
        Self {
            v_gravity: [0., 0., -9.81].into(),
            v_wind: [0., 0., 0.].into(),
        }
    }

    pub fn new_low_wind() -> Self {
        Self {
            v_gravity: [0., 0., -9.81].into(),
            v_wind: [3., -1., 0.].into(),
        }
    }

    pub fn new_high_wind() -> Self {
        Self {
            v_gravity: [0., 0., -9.81].into(),
            v_wind: [15., 0.5, 0.].into(),
        }
    }
}

pub struct ProjectileSim {
    pub projectile: Projectile,
    pub env: Enviornment,
    n_tick: u128,
}

impl ProjectileSim {
    pub fn new(env: Enviornment, projectile: Projectile) -> Self {
        Self {
            projectile: projectile,
            env: env,
            n_tick: 0,
        }
    }

    pub fn get_tick(&self) -> u128 {
        return self.n_tick;
    }
    // Returns if it should keep running
    pub fn tick(&mut self) -> bool {
        self.projectile.tick(&self.env);

        self.n_tick = self.n_tick + 1;
        !self.has_reached_end()
    }

    fn has_reached_end(&self) -> bool {
        self.projectile.p_position.y <= 0.
    }
}

pub fn run() -> Result<ProjectileSim, anyhow::Error>{
    pretty_env_logger::try_init();
    let p = Projectile::new()
        .with_position((0., 1., 0.))
        .with_velocity([1., 1., 0.]);

    let e = Enviornment::new_low_wind();

    let mut sim = ProjectileSim::new(e,p);

    let mut should_run = true;
    let mut last_tick = 0;
    let max_tick = 10_000;

    info!("Staring Projectile: {:#?}", sim.projectile);
    while should_run && last_tick < max_tick{
        should_run = sim.tick();
        last_tick = sim.get_tick();
    }

    info!("Ending Projectile: {:#?}", sim.projectile);
    info!("Last Tick: {:#?}", last_tick);
    Ok(sim)
}


pub fn main() -> Result<(), anyhow::Error>{

    run().unwrap();
    Ok(())
}

#[cfg(test)]

mod sim_test {
    use log::info;

    use super::*;

    #[test]
    pub fn test_projectile_basic() {
       
        let res = run();

        assert!(res.is_ok());

        let sim = res.unwrap();

        let last_tick = sim.get_tick();

        assert_ne!(last_tick, 10_000);
        assert!(sim.projectile.p_position.y <= 0.);
    }
}
