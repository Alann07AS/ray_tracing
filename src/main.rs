use ray_tracing::ray_tracing::{
    camera::Camera,
    ray::Ray,
    shapes::{color::Color, light::*, plane::Plane, shapes::Shape, sphere::Sphere},
    vector::Vector,
};
use std::fs::File;
use std::io::Write;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    const WIDTH: f64 = 800.;
    const HEIGHT: f64 = 600.;

    let event_loop = EventLoop::new();

    // Create a window with winit
    let window = WindowBuilder::new()
        .with_title("Simple Window")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .build(&event_loop)
        .unwrap();

    // Create a pixel buffer
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let mut camera = Camera::new(WIDTH, HEIGHT);
    camera.orientation(Vector {
        x: 0.,
        y: 0.,
        z: -1.,
    });

    // const AMBIANT_LIGHT: Color = Color::WHITE; //Color {r: 100., g: 100., b: 100.};
    const AMBIANT_LIGHT: AmbiantLight = AmbiantLight {
        color: Color::WHITE,
        intensity: 90.,
    }; //low
    let mut source_light_1: SpotLight = SpotLight::new();
    source_light_1.source = Vector::new(0., 0., -3.);
    source_light_1.direction = Vector::new(0., -1., 0.);
    source_light_1.angle(70.);

    let source_light_2: PlaneLight = PlaneLight {
        color: Color::GREEN,
        intensity: 100.,
        source: Vector {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        direction: Vector {
            x: 0.5,
            y: -0.5,
            z: 0.,
        }
        .unit(),
    };
    let source_light_3: SourceLight = SourceLight {
        color: Color::BLUE,
        intensity: 100.,
        source: Vector {
            x: 0.,
            y: 0.,
            z: -3.,
        },
    };
    // let ambiant_light = AmbiantLight {
    //     color: AMBIANT_LIGHT,
    //     intensity: 100.,
    // };
    const BACKGROUND: Color = Color {
        r: 255.,
        g: 255.,
        b: 255.,
    };
    // const BACKGROUND: Color = Color {
    // r: 0.,
    // g: 0.,
    // b: 0.,
    // };

    let mut spere1 = Sphere {
        center: Vector::new(-2., 0., -3.),
        rayon: 0.5,
        color: Color::dark_from_color(Color::WHITE),
    };

    let spere2 = Sphere {
        center: Vector::new(2., 0., -3.),
        rayon: 0.5,
        color: Color::dark_from_color(Color::WHITE),
    };

    let spere3 = Sphere {
        center: Vector::new(0., 2., -3.),
        rayon: 0.5,
        color: Color::dark_from_color(Color::WHITE),
    };

    let spere4 = Sphere {
        center: Vector::new(0., -2., -3.),
        rayon: 0.5,
        color: Color::dark_from_color(Color::WHITE),
    };

    let plane = Plane {
        origin: Vector {
            x: 0.,
            y: -2.,
            z: 0.,
        },
        direction: Vector {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        color: Color::dark_from_color(Color::WHITE),
    };


    let spedre1: std::cell::RefCell<&dyn Shape> = std::cell::RefCell::new(&Sphere {
        center: Vector::new(-2., 0., -3.),
        rayon: 0.5,
        color: Color::dark_from_color(Color::WHITE),
    });
    let shapes: [std::cell::RefCell<&dyn Shape>; 5] = [spedre1, &spere2, &spere3, &spere4, &plane];
    let shapes: [&dyn Shape; 5] = [&spere1, &spere2, &spere3, &spere4, &plane];

    let mut file = File::create("test.ppm").expect("Impossible d'ouvrir le fichier");
    let mut output = "".to_string();
    output += &format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    // writeln!(file, "P3\n{} {}\n255", WIDTH, HEIGHT).expect("Échec de l'écriture de l'en-tête");

    let mut ray = Ray::new(
        camera.position,
        camera.corner,
        camera.vec_width,
        camera.vec_height,
    );
    let mut render = move |pixels: &mut Pixels, shapes: &[&dyn Shape]| {
        for y in 0..HEIGHT as i32 {
            ray.set_y(y as f64);
            for x in 0..WIDTH as i32 {
                ray.set_x(x as f64);
                let mut farest_shape: &&dyn Shape = &shapes[0];
                let mut closer_size: f64 = f64::MAX;
                for sha in shapes {
                    match sha.intersect(&ray) {
                        Some(dist) => {
                            if dist < closer_size {
                                closer_size = dist;
                                farest_shape = sha;
                            }
                        }
                        _ => (), // None => writeln!(file, "{} {} {}\n", BACKGROUND.r,BACKGROUND.g,BACKGROUND.b).expect("Échec de l'écriture des données de couleur"),
                    }
                }
                if closer_size != f64::MAX {
                    let point = ray.at(closer_size);
                    let normal = farest_shape.normal(&point);
                    // eprintln!("normal = {:?}", normal);
                    // output += &format!("{} {} {}\n", ((normal.x+1.)/2.*255.).trunc(), ((normal.y+1.)/2.*255.).trunc(), ((normal.z+1.)/2.*255.).trunc());
                    // let color = farest_shape.color().light_apply(AMBIANT_LIGHT).sclaled_255();
                    let mut color = source_light_1.light_apply(farest_shape.color(), &point, &normal);
                    let color2 = source_light_2.light_apply(farest_shape.color(), &point, &normal);
                    let color3 = source_light_3.light_apply(farest_shape.color(), &point, &normal);
                    // let color4 = AMBIANT_LIGHT.light_apply(farest_shape.color(), &point, &normal);
                    let color = color.blend(&color3).sclaled_255();
    
                    let width = WIDTH as usize;
                    let (x, y) = (x as usize, y as usize);
                    let output: &[u8; 4] = &[color.r as u8, color.g as u8, color.b as u8, 255];
                    pixels.get_frame()
                        [((y * width + x) * 4) as usize..((y * width + x + 1) * 4) as usize]
                        .copy_from_slice(output);
                    // output += &format!("{} {} {}\n", color.r, color.g, color.b);
                } else {
                    let color = BACKGROUND;
    
                    let width = WIDTH as usize;
                    let (x, y) = (x as usize, y as usize);
                    let output: &[u8; 4] = &[color.r as u8, color.g as u8, color.b as u8, 255];
                    pixels.get_frame()
                        [((y * width + x) * 4) as usize..((y * width + x + 1) * 4) as usize]
                        .copy_from_slice(output);
                    // output += &format!("{} {} {}\n", BACKGROUND.r, BACKGROUND.g, BACKGROUND.b);
                }
            }
        }
    };
    render(&mut pixels, &shapes);

    // Example: store the pressed keys
    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    println!("coucou");
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                        // Some(VirtualKeyCode::Up) =>  {let cp = &camera.position; camera.position(Vector { x: cp.x, y: cp.y+2., z: cp.z })},
                        Some(VirtualKeyCode::Up) =>  {spere1.center.x += 1.; render(&mut pixels, &shapes)},
                        _ => (),
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                // Render the pixels to the window
                if pixels
                    .render()
                    .map_err(|e| eprintln!("Failed to render pixels: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => {}
        }
    });
    writeln!(file, "{}", output).expect("Échec de l'écriture des données");
    println!("Fichier PPM généré avec succès :");
}
