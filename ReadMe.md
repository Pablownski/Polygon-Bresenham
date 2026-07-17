# Polygon-Bresenham

Framebuffer en Rust con trazado de líneas usando el algoritmo de Bresenham (todos los octantes).

## Estructura

- `src/framebuffer.rs` — buffer de píxeles (`Framebuffer`)
- `src/line.rs` — `draw_line(fb, x0, y0, x1, y1)` con Bresenham
- `src/bmp.rs` — escritura del framebuffer a `.bmp`
- `src/main.rs` — punto de entrada

## Salida

![output](output.png)

Generado con:

```
cargo run
```
