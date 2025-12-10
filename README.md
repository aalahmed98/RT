# Ray Tracer

A ray tracing renderer implemented in Rust, featuring shadows, brightness control, and multiple material types.

## Features

- **Multiple Object Types**: Spheres, Cubes, Cylinders, and Planes
- **Material System**: Lambertian (matte), Metal (reflective), and Dialectric (glass/transparent)
- **Shadows & Lighting**: Dynamic shadow casting with multiple light sources
- **Brightness Control**: Each object can emit light based on brightness parameter
- **Camera Control**: Adjustable camera position, field of view, and depth of field

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run > output.ppm
```

The output will be a PPM image file that can be viewed with most image viewers.

## Quick Start

1. Open `src/main.rs`
2. Find the scene you want to render (SCENE 1, 2, 3, or 4)
3. Uncomment that scene section
4. Make sure all other scenes are commented out
5. Run: `cargo run > scene1.ppm`

## Scene Selection

The project includes 4 pre-configured scenes:

- **SCENE 1**: Simple sphere scene with shadows
- **SCENE 2**: Plane and cube with lower brightness
- **SCENE 3**: All object types (sphere, cube, cylinder, plane)
- **SCENE 4**: Same as Scene 3 from different camera angle

To use a scene, uncomment it in `src/main.rs` and comment out all others.

## Creating Objects

### Sphere

```rust
let material = Lambertian::new(Color::new(0.8, 0.2, 0.2), 0.5);
world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 0.8, material));
//                                 ^center          ^radius
```

### Cube

```rust
let material = Lambertian::new(Color::new(0.2, 0.6, 0.8), 0.4);
world.add(Cube::from_center_size(
    Point3::new(0.0, 1.0, 0.0),  // center
    1.2,                          // size (edge length)
    material
));
```

### Cylinder

```rust
let material = Lambertian::new(Color::new(0.6, 0.8, 0.2), 0.45);
world.add(Cylinder::new(
    Point3::new(0.0, 1.0, 0.0),  // center
    0.6,                          // radius
    2.0,                          // height
    material
));
```

### Plane

```rust
let material = Lambertian::new(Color::new(0.3, 0.3, 0.3), 0.0);
world.add(Plane::new(
    Point3::new(0.0, 0.0, 0.0),  // point on plane
    Vec3::new(0.0, 1.0, 0.0),    // normal vector (pointing up)
    material
));
```

## Materials

### Lambertian (Matte/Diffuse)

```rust
Lambertian::new(Color::new(r, g, b), brightness)
// Example:
Lambertian::new(Color::new(0.8, 0.2, 0.2), 0.6)
//                                              ^ brightness (0.0 to 1.0+)
```

### Metal (Reflective)

```rust
Metal::new(Color::new(r, g, b), fuzz, brightness)
// Example:
Metal::new(Color::new(0.9, 0.9, 0.9), 0.0, 0.0)
//                                    ^fuzz ^brightness
// fuzz: 0.0 = perfect mirror, 1.0 = very rough
```

### Dialectric (Glass/Transparent)

```rust
Dialectric::new(refraction_index, brightness)
// Example:
Dialectric::new(1.5, 0.0)
//              ^refraction_index (1.5 = glass)
```

## Brightness Parameter

Controls how much light an object emits:

- `0.0` - No emission (normal object)
- `0.1-0.3` - Subtle glow
- `0.3-0.5` - Moderate emission, soft shadows
- `0.5-1.0` - Bright object, visible shadows
- `> 1.0` - Very bright, acts like light source

## Light Sources

```rust
let lights = vec![
    Light::new(
        Point3::new(5.0, 6.0, 3.0),      // position
        Color::new(1.0, 1.0, 1.0),       // color (white)
        1.0                              // intensity
    ),
];
```

## Camera Settings

```rust
let mut cam = Camera::new();

cam.aspect_ratio = 16.0/9.0;           // Image aspect ratio
cam.image_width = 600;                 // Image width in pixels
cam.samples_per_pixel = 500;           // Samples per pixel (higher = better quality, slower)
cam.max_depth = 50;                    // Maximum ray bounce depth
cam.vfov = 20.0;                       // Vertical field of view (degrees)
cam.lookfrom = Point3::new(8.0, 3.0, 5.0);  // Camera position
cam.lookat = Point3::new(0.0, 1.0, 0.0);    // Point camera is looking at
cam.vup = Vec3::new(0.0, 1.0, 0.0);         // Up vector
cam.defocus_angle = 0.0;               // Depth of field blur (0.0 = no blur)
cam.focus_dist = 10.0;                  // Focus distance

cam.render(&world, &lights);
```

### Camera Tips

- **Field of View**: Lower values (15-25°) = zoomed in, Higher values (30-50°) = wider view
- **Samples per Pixel**: 
  - 100-200: Fast, noisy
  - 500: Good balance
  - 1000+: High quality, slow
- **Image Width**: 
  - 400: Fast rendering
  - 600: Good quality
  - 800+: High resolution, slow

## Color Values

Colors use RGB values from 0.0 to 1.0:

```rust
Color::new(1.0, 0.0, 0.0)  // Red
Color::new(0.0, 1.0, 0.0)  // Green
Color::new(0.0, 0.0, 1.0)  // Blue
Color::new(1.0, 1.0, 1.0)  // White
Color::new(0.0, 0.0, 0.0)  // Black
```

## Example: Creating a Custom Scene

```rust
fn main() {
    let mut world = HittableList::new();
    
    // Ground plane
    let ground = Lambertian::new(Color::new(0.2, 0.2, 0.2), 0.0);
    world.add(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground
    ));
    
    // Add objects
    let sphere_mat = Lambertian::new(Color::new(0.8, 0.2, 0.2), 0.6);
    world.add(Sphere::new(Point3::new(0.0, 1.5, 0.0), 1.0, sphere_mat));
    
    // Add lights
    let lights = vec![
        Light::new(Point3::new(5.0, 6.0, 3.0), Color::new(1.0, 1.0, 1.0), 1.0),
    ];
    
    // Setup camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(8.0, 3.0, 5.0);
    cam.lookat = Point3::new(0.0, 1.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;
    
    // Render
    cam.render(&world, &lights);
}
```

## Performance Tips

- Reduce `samples_per_pixel` for faster rendering (200-300 for quick tests)
- Lower `image_width` for faster rendering (400 for quick tests)
- Reduce `max_depth` if you don't need deep reflections/refractions (20-30)
- Use fewer light sources for faster rendering

## Output Format

The renderer outputs PPM (Portable Pixmap) format, which can be viewed with:
- Most image viewers (GIMP, ImageMagick, etc.)
- Online PPM viewers
- Convert to PNG: `convert output.ppm output.png` (ImageMagick)

## Project Structure

```
rt/
├── src/
│   ├── main.rs          # Scene definitions and main entry point
│   ├── camera.rs         # Camera and rendering logic
│   ├── material.rs       # Material types (Lambertian, Metal, Dialectric)
│   ├── light.rs          # Light source definition
│   ├── sphere.rs         # Sphere object
│   ├── cube.rs           # Cube object
│   ├── cylinder.rs       # Cylinder object
│   ├── plane.rs          # Plane object
│   └── ...
└── README.md            # This file
```

## License

This project is for educational purposes.

