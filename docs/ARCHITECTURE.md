# Arquitectura

## Vision general

El proyecto usa una arquitectura simple orientada a loop de juego:

1. Entrada
2. Actualizacion de estado
3. Colisiones
4. Limpieza de entidades
5. Render

Todo ocurre dentro de `run()` y se ejecuta una vez por frame.

## Modulos

### `main.rs`

- Inicializa Macroquad.
- Delega toda la ejecucion a `run()`.

### `lib.rs`

- Declara modulos publicos.
- Reexporta `run` como API del crate.

### `entities.rs`

- Define datos de entidades del juego.
- `Bullet`: rectangulo, velocidad y bandera de actividad.
- `Target`: rectangulo y bandera de actividad.

### `game.rs`

- Contiene el loop principal.
- Procesa input del jugador.
- Spawnea enemigos por temporizador.
- Actualiza posiciones.
- Resuelve colisiones.
- Renderiza mundo y HUD.

## Flujo por frame

1. Calcular `delta` con `get_frame_time()`.
2. Aplicar input horizontal al jugador.
3. Limitar jugador al ancho de pantalla.
4. Crear bala cuando se presiona espacio.
5. Spawnear target cuando el timer supera 1.0s.
6. Mover balas y targets.
7. Detectar colisiones bala vs target.
8. Limpiar entidades inactivas.
9. Dibujar fondo, jugador, balas, targets y score.
10. Esperar siguiente frame con `next_frame().await`.

## Decisiones actuales

- Entidades representadas con `Rect` para simplificar colision y render.
- Colision AABB para costo bajo por frame.
- Textura del jugador generada en runtime para evitar dependencia de assets.

## Deuda tecnica conocida

- `game.rs` aun concentra demasiadas responsabilidades.
- Falta sistema de estados de juego.
- Falta separar configuraciones en constantes estructuradas.