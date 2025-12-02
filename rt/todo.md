# Ray Tracer Project - TODO List

## ✅ Completed Features

- [x] **Core Ray Tracer Implementation**
  - Ray casting and recursive ray bouncing
  - PPM image output (P3 format)
  - Monte Carlo path tracing

- [x] **All 4 Required Objects**
  - [x] Sphere (`sphere.rs`)
  - [x] Cylinder (`cylinder.rs`)
  - [x] Cube (`cube.rs`)
  - [x] Flat Plane (`plane.rs`)

- [x] **Object Positioning**
  - Objects can be positioned anywhere in 3D space
  - Sphere: center + radius
  - Cylinder: center + radius + height
  - Cube: min/max corners or center + size
  - Plane: point + normal vector

- [x] **Camera System**
  - Camera position (`lookfrom`)
  - Camera target (`lookat`)
  - Camera up vector (`vup`)
  - Field of view (`vfov`)
  - Aspect ratio control

- [x] **Materials**
  - Lambertian (matte/diffuse)
  - Metal (reflective)
  - Dialectric (glass/refractive)

---

## ❌ Remaining Tasks

### 1. Light Management System ⚠️ **REQUIRED**

#### 1.1 Light Sources
- [ ] Create `Light` struct/trait
  - Point light (position + intensity/color)
  - Directional light (direction + intensity)
  - Optional: Area lights for soft shadows

#### 1.2 Shadow Implementation
- [ ] Shadow ray casting
  - Cast ray from hit point to light source
  - Check if ray hits any object before reaching light
  - If blocked, point is in shadow

#### 1.3 Direct Lighting Calculation
- [ ] Diffuse lighting (Lambertian shading)
  - Calculate: `light_intensity * max(0, dot(normal, light_direction))`
  - Add to material color

#### 1.4 Brightness Control
- [ ] Add brightness parameter to light sources
- [ ] Add global brightness multiplier (optional)
- [ ] Provide easy API to adjust scene brightness

**Implementation Notes:**
- Modify `ray_color()` function in `camera.rs`
- Add light sources to world/scene
- Integrate with existing material system
- Consider ambient lighting for areas not directly lit

---


### 2. Resolution Configuration (Optional but Recommended)

- [ ] Make resolution easily configurable
  - Currently hardcoded in `main.rs` (`cam.image_width = 800`)
  - Consider: command-line arguments, config file, or constants at top of `main.rs`
  - Should support 800x600 for final renders

---

## 📋 Implementation Priority

### High Priority (Required for Project)
1. **Light Management System** - Required feature
2. **Documentation** - Required for auditors

### Medium Priority (Recommended)
3. **Resolution Configuration** - Makes project more usable

---
