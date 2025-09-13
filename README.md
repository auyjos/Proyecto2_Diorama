# 🌐 Sphere Raytracer with Phong Lighting

Un raytracer en Rust que implementa el modelo de iluminación Phong para renderizar esferas con iluminación realista.

## 📋 Características

- **Raytracing de Esferas**: Intersección ray-sphere usando la ecuación cuadrática
- **Modelo de Iluminación Phong**: Implementación completa con componentes:
  - **Ambiente**: Iluminación base uniforme
  - **Difusa**: Reflexión lambertiana (mate)
  - **Especular**: Reflexión brillante con exponente configurable
- **Z-Buffer**: Manejo correcto de profundidad para múltiples objetos
- **Múltiples Luces**: Soporte para varias fuentes de luz de diferentes colores
- **Materiales Personalizables**: Propiedades de diffuse, specular y albedo

## 🏗️ Arquitectura del Proyecto

### Estructura de Archivos

```
src/
├── main.rs           # Punto de entrada y loop principal
├── framebuffer.rs    # Manejo del buffer de píxeles
├── sphere.rs         # Implementación de esferas y rendering
├── ray_intersect.rs  # Trait para intersección de rayos
├── material.rs       # Definición de materiales
├── light.rs          # Estructura de luces
└── color.rs          # Sistema de colores con operaciones
```

### Componentes Principales

#### 🎯 **Intersect Structure**
```rust
pub struct Intersect {
    pub distance: f32,       // Distancia desde el origen del rayo
    pub is_intersecting: bool, // Si hay intersección
    pub material: Material,   // Material del objeto
    pub point: Vector3,      // Punto de intersección
    pub normal: Vector3,     // Vector normal en el punto
}
```

#### 🌟 **Material System**
```rust
pub struct Material {
    pub diffuse: Color,     // Color difuso del material
    pub specular: f32,      // Exponente especular (brillo)
    pub albedo: [f32; 2],   // [difuso, especular] - pesos de reflexión
}
```

#### 💡 **Light Structure**
```rust
pub struct Light {
    pub position: Vector3,  // Posición de la luz
    pub color: Color,       // Color de la luz
    pub intensity: f32,     // Intensidad luminosa
}
```

## 🧮 Matemáticas Implementadas

### Ray-Sphere Intersection
Usando la **ecuación cuadrática** para encontrar intersecciones:

```rust
// Coeficientes de la ecuación cuadrática
let a = ray_direction.dot(ray_direction);           // ||d||²
let b = 2.0 * oc.dot(ray_direction);               // 2(oc·d)
let c = oc.dot(oc) - radius * radius;              // ||oc||² - r²

// Discriminante: b² - 4ac
let discriminant = b * b - 4.0 * a * c;

// Soluciones: t = (-b ± √discriminant) / 2a
let t1 = (-b - sqrt_discriminant) / (2.0 * a);     // Entrada
let t2 = (-b + sqrt_discriminant) / (2.0 * a);     // Salida
```

### Modelo de Iluminación Phong
Combinación de tres componentes de luz:

```rust
// 1. Componente Ambiente
let ambient = material.diffuse * ambient_intensity;

// 2. Componente Difusa (Lambert)
let diffuse_intensity = normal.dot(light_direction).max(0.0);
let diffuse = material.diffuse * (diffuse_intensity * albedo[0]);

// 3. Componente Especular
let reflect_dir = reflect(-light_direction, normal);
let spec_intensity = view_dir.dot(reflect_dir).max(0.0).powf(specular);
let specular = white * (spec_intensity * albedo[1]);

// Color final
let final_color = ambient + diffuse + specular;
```

### Reflexión de Vectores
```rust
fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    incident - normal * 2.0 * incident.dot(normal)
}
```

## 🎨 Configuración de la Escena

La escena actual incluye:

### Esferas
- **Esfera Ivory**: 
  - Posición: `(1.0, 0.0, -4.0)`
  - Material: Superficie brillante con alto especular
  - Color: Blanco marfil `(255, 255, 240)`

- **Esfera Rubber**:
  - Posición: `(2.0, 0.0, -5.0)`
  - Material: Superficie mate con bajo especular
  - Color: Marrón `(139, 69, 19)`

### Luces
- **Luz Principal**: Blanca en `(-3.0, 3.0, -2.0)` con intensidad 1.0
- **Luz Secundaria**: Rojiza en `(3.0, -3.0, -2.0)` con intensidad 0.5

## 🚀 Compilación y Ejecución

### Requisitos
- Rust 1.70+
- Raylib para Rust

### Comandos
```bash
# Compilar
cargo build

# Ejecutar
cargo run

# Compilar optimizado
cargo build --release
```

## 🔧 Personalización

### Agregar Nuevas Esferas
```rust
let new_sphere = Sphere {
    center: Vector3::new(x, y, z),
    radius: radio,
    material: Material::new(
        Color::new(r, g, b),    // Color difuso
        exponent,               // Exponente especular (1-200)
        [diffuse_weight, specular_weight] // Albedo [0.0-1.0]
    ),
};
```

### Modificar Materiales
- **Alto Especular (brillante)**: `specular: 50.0+`, `albedo: [0.6, 0.4]`
- **Bajo Especular (mate)**: `specular: 10.0-`, `albedo: [0.9, 0.1]`
- **Metálico**: `specular: 100.0+`, `albedo: [0.3, 0.7]`

### Agregar Luces
```rust
let nueva_luz = Light::new(
    Vector3::new(x, y, z),      // Posición
    Color::new(r, g, b),        // Color
    intensidad                  // Intensidad (0.0-2.0)
);
```

## 📊 Parámetros Técnicos

### Resolución
- **Predeterminada**: 800x600 píxeles
- **Configurable**: A través de `window_width` y `window_height`

### Campo de Visión
- **FOV**: Calculado automáticamente
- **Aspect Ratio**: Se ajusta dinámicamente al tamaño de ventana

### Optimizaciones
- **Z-Buffer**: Evita cálculos innecesarios de lighting
- **Early Exit**: Rayos que no intersectan objetos retornan color de cielo
- **In-place Normalization**: Operaciones vectoriales optimizadas

## 🎯 Futuras Mejoras

- [ ] **Sombras**: Cast de rayos hacia luces para detectar oclusión
- [ ] **Reflexiones**: Rayos secundarios para superficies reflectantes
- [ ] **Refracciones**: Simulación de materiales transparentes
- [ ] **Texturas**: Mapeo de texturas en superficies
- [ ] **Anti-aliasing**: Suavizado de bordes
- [ ] **Más Primitivas**: Planos, cubos, cilindros
- [ ] **Aceleración Espacial**: BVH o Octrees para escenas complejas

## 📝 Créditos

Implementado siguiendo los principios fundamentales del raytracing y el modelo de iluminación Phong, usando Rust y Raylib para un rendimiento óptimo.

Autor: Jośe Andrés Auyón Cóbar

---

**Desarrollado con ❤️ en Rust** 🦀
