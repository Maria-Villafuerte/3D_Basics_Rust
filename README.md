# Raycasting 3D Basics in Rust

## Descripción

El objetivo de este proyecto es demostrar los conocimientos adquiridos durante la primera parte del curso. En particular, se enfoca en practicar el concepto de proyección, es decir, convertir una representación 2D en una 3D.

Este proyecto implementa un Ray Caster simple en Rust que renderiza un nivel entero y jugable. Asegúrate de que el jugador pueda moverse sin atravesar paredes ni experimentar fallos. El proyecto incluye diferentes características opcionales que permiten obtener puntos adicionales en base a los requisitos del curso.

## Características

- **Raycasting 3D:** Implementa una proyección simple de un nivel 3D desde una perspectiva 2D.
- **Colores y Texturas:** Cada pared en el mapa tiene un color o textura diferente.
- **FPS:** La aplicación mantiene al menos 15 fps y muestra la tasa de fotogramas por segundo.
- **Movimiento y Rotación de la Cámara:** Implementa movimiento hacia adelante y hacia atrás, así como rotación horizontal. Rotación con el mouse también está disponible.
- **Minimapa:** Muestra la posición del jugador en el mundo en una esquina de la pantalla.
- **Música de Fondo:** Incluye música de fondo durante el juego.
- **Efectos de Sonido:** Agrega efectos de sonido para una experiencia más inmersiva.
- **Sprites y Animaciones:** Utiliza billboarding para mostrar sprites y agrega al menos una animación a un sprite en la pantalla.
- **Pantalla de Bienvenida:** Una pantalla de bienvenida inicial.
- **Selección de Niveles:** Permite seleccionar entre múltiples niveles en la pantalla de bienvenida.
- **Pantalla de Éxito:** Muestra una pantalla de éxito cuando se cumple una condición en el nivel.

## Requisitos

- Rust 1.60 o superior
- [Cargo](https://doc.rust-lang.org/cargo/) (incluido con Rust)
- [Dependencias del proyecto](Cargo.toml)

## Demostración 

![Raycasting Demo](/GiftDemo.gif)



## Instalación

1. Clona el repositorio:
   ```bash
   git clone https://github.com/Maria-Villafuerte/Raycasting_3D_Basics_Rust.git
2. Navega al directorio del proyecto
   ```bash
   cd Raycasting_3D_Basics_Rust
3. Instala las dependencias:
   ```bash
   cargo build
4. Ejecuta el proyecto:
   ```bash
   cargo run


## Criterios de Evaluación
- Hardware No Tradicional (0 a 50 puntos): Implementación en hardware distinto a una computadora tradicional.
- Soporte para Control (20 puntos): Implementación de soporte para controladores.
- Estética del Nivel (0 a 30 puntos): Calidad estética del nivel.
- FPS (15 puntos): Mantener al menos 15 fps.
- Movimiento y Rotación de la Cámara (10 puntos): Implementar movimiento y rotación de la cámara.
- Rotación con el Mouse (10 puntos): Implementar rotación horizontal con el mouse.
- Minimapa (10 puntos): Mostrar la posición del jugador en un minimapa.
- Música de Fondo (5 puntos): Agregar música de fondo.
- Efectos de Sonido (10 puntos): Agregar efectos de sonido.
- Sprites y Billboarding (20 puntos): Agregar sprites y billboarding.
- Animaciones (30 puntos): Agregar al menos una animación a un sprite.
- Pantalla de Bienvenida (5 puntos): Incluir una pantalla de bienvenida.
- Selección de Niveles (10 puntos): Permitir selección de niveles en la pantalla de bienvenida.
- Pantalla de Éxito (10 puntos): Mostrar una pantalla de éxito al cumplir condiciones.
