use na::Vec3;

use geometry::Object;

pub enum Material {
    Plain {
        color: f64;
    },

    Lambert {
        ambient: Vec3<f64>;
        diffuse: Vec3<f64>;
        specular: Vec3<f64>;
        emission: Vec3<f64>;
        shininess: f64;
    }
}

pub struct Light {
    pub pos: Vec3<f64>;
    pub color: Vec3<f64>;
}

// Shadows are todo!
impl Material {
    fn compute_color(&self, &Ray, tnear: f64, nhit: Vec3<f64>,
                     objects: &Vec<Box<Object>>, lights: &Vec<Box<Light>>)
                     -> Vec3<f64> {
        match *self {
            Material::Plain { color } => { Vec3<f64> { x: color,
                                                       y: color,
                                                       z: color }
            }

            Material::Lambert { ambient, diffuse, specular, emission,
                                shininess } => {
                let mut color = Vec3<f64>::zero();
                let phit = ray.origin + ray.dir + tnear;
                for l in &lights {
                    let ldir = l.pos.normalize();
                    let halfv = (ray.dir + ldir).normalize();
                    let ndotl = nhit.dot(&ldir);
                    let lambert = diffuse * l.color * max(ndotl, 0.0);

                    let ndoth = nhit.dot(&halfv);
                    let phong = specular * l.color
                        * expt(max(ndoth, 0.0), shininess);

                    color = color + lambert + phong;
                }
                color + ambient + emission
            }
        }
    }
}
