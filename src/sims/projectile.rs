use ray_ray::math::Tuple;

use log::info;


pub struct Projectile{
    pub position_p: Tuple,
    pub velocity_v: Tuple,
}

pub struct Env{
    pub wind_v: Tuple,
    pub gravity_v: Tuple
}

pub struct ProjectileSim{
    pub projectile: Projectile,
    pub env: Env,
    pub tick : i32
}


impl ProjectileSim{ 
    pub fn tick(&mut self){
        self.tick += 1;
        let new_pos =  self.projectile.position_p + self.projectile.velocity_v;
        self.projectile.position_p = new_pos;
        self.projectile.velocity_v = self.projectile.velocity_v + self.env.gravity_v + self.env.wind_v
    }

    pub fn run() -> ProjectileSim{
        let mut sim = ProjectileSim{
            projectile: Projectile{
                position_p: Tuple::point(0.0, 1.0, 0.0),
                velocity_v: Tuple::vector(1.0, 1.0, 0.0).normalize(),
            },
            env: Env{
                wind_v: Tuple::vector(-0.01, 0.0, 0.0),
                gravity_v: Tuple::vector(0.0, -0.1, 0.0)
            },
            tick: 0
        };
       
        while sim.projectile.position_p.y > 0.0{
            sim.tick();
            info!("Projectile Position @ tick ({}): {:?}", sim.tick, sim.projectile.position_p)
        }

        sim
    }
}



pub fn main(){
    env_logger::init();

    let sim_res = ProjectileSim::run();
    info!("Projectile Position @ tick ({}): {:?}", sim_res.tick, sim_res.projectile.position_p)

    o
}


#[cfg(test)]
mod test{
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_sim_projectile(){

        let sim_res = ProjectileSim::run();
        assert_eq!(sim_res.projectile.position_p.y < 0.0, true);
        assert_eq!(sim_res.tick > 0, true);
    }
}