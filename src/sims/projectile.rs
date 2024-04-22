use std::path::PathBuf;

use ray_ray::{math::Tuple, rendering::{Canvas, PPMFile, RerunViewer}};

use log::info;

pub struct Projectile {
    pub position_p: Tuple,
    pub velocity_v: Tuple,
}

pub struct Env {
    pub wind_v: Tuple,
    pub gravity_v: Tuple,
}

pub struct ProjectileSim {
    pub projectile: Projectile,
    pub env: Env,
    pub tick: i32,
    pub canvas: Canvas,
}

impl ProjectileSim {
    pub fn tick(&mut self) {
        self.tick += 1;
        let new_pos = self.projectile.position_p + self.projectile.velocity_v;
        self.projectile.position_p = new_pos;
        self.projectile.velocity_v =
            self.projectile.velocity_v + self.env.gravity_v + self.env.wind_v
    }

    pub fn run() -> ProjectileSim {
        let mut sim = ProjectileSim {
            projectile: Projectile {
                position_p: Tuple::point(0.0, 1.0, 0.0),
                velocity_v: Tuple::vector(1.0, 1.8, 0.0).normalize() * 9.5,
            },
            env: Env {
                wind_v: Tuple::vector(-0.01, 0.0, 0.0),
                gravity_v: Tuple::vector(0.0, -0.1, 0.0),
            },
            tick: 0,
            canvas: Canvas::new(900, 550)

        };

        while sim.projectile.position_p.y > 0.0 {
            sim.tick();
            
       
            info!(
                "Projectile Position @ tick ({}): {:?}",
                sim.tick, sim.projectile.position_p
            );

            if (sim.projectile.position_p.y < 0.) || (sim.projectile.position_p.y < 0.){
                continue;
            }
            let canvas_y = sim.canvas.height;
            let pos = (sim.projectile.position_p.x as usize, canvas_y - sim.projectile.position_p.y as usize ); 
            sim.canvas.write_pixel([1.0, 1.0, 1.0].into(),pos );

        }

        sim
    }
}

pub fn main() {
    env_logger::init();

    let rec = rerun::RecordingStreamBuilder::new("projectile_test").spawn().unwrap();
 
rec.set_time_seconds("time", 0.0);
    let sim_res = ProjectileSim::run();
    info!(
        "Projectile Position @ tick ({}): {:?}",
        sim_res.tick, sim_res.projectile.position_p
    );

    let ppm = PPMFile::from_canvas(&sim_res.canvas);
    ppm.save_file(&PathBuf::from("./projectile.ppm"));

   
    rec.set_time_seconds("time", 1.0);
    let rerun_image = RerunViewer::from_canvas(&sim_res.canvas);
    rec.log("image/raw", &rerun_image).unwrap();

}   

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_sim_projectile() {
        let sim_res = ProjectileSim::run();
        assert_eq!(sim_res.projectile.position_p.y < 0.0, true);
        assert_eq!(sim_res.tick > 0, true);
    }
}
