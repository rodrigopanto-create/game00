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
- `PowerUpType`, `PowerUp` y `PowerUpEffects`: coleccionables y efectos
  temporales independientes, renovables y combinables.
- `Boss` y `EnemyBullet`: vida, movimiento y ataques del jefe.

### `game.rs`

- Contiene el loop principal.
- Gestiona las pantallas y delega el combate a `game/session.rs`.
- `game/controls.rs` unifica teclado, raton y tactil, incluido disparo mantenido.
- `game/session.rs` mantiene las fases Wave, Boss y Victory por nivel.
- `game/session/player.rs` aplica movimiento, cadencia y disparo multiple.
- `game/session/world.rs` gestiona spawn, coleccionables, jefe y colisiones.
- `game/session/hud.rs` dibuja score, nivel, efectos, vida y transicion.

## Flujo por frame

1. Calcular `delta` con `get_frame_time()`.
2. Aplicar input horizontal al jugador.
3. Limitar jugador al ancho de pantalla.
4. Actualizar efectos y crear balas segun el cooldown y los bonos activos.
5. Spawnear targets solo en la fase Wave, segun la dificultad actual.
6. Mover balas, targets, bonos y jefe; resolver recogidas y ataques hostiles.
7. Detectar impactos del jugador y contar bajas; iniciar Boss al alcanzar
   el umbral o Victory al agotar la vida del jefe.
8. Limpiar entidades inactivas.
9. Dibujar fondo, jugador, mundo y HUD; dibujar los botones tactiles al final.
10. Esperar siguiente frame con `next_frame().await`.

## Decisiones actuales

- Entidades representadas con `Rect` para simplificar colision y render.
- Colision AABB para costo bajo por frame.
- Graficos geometricos para evitar dependencia de assets.
- La simulacion recibe el viewport para probar colisiones y progresion sin
  inicializar una ventana de Macroquad. El render usa las mismas entidades
  en escritorio y movil.
- La fase Victory congela el combate y los efectos durante 2.5 segundos.
  Luego reinicia las bajas y los timers de oleada, conservando vida y score.

## Deuda tecnica conocida

- Falta separar configuraciones en constantes estructuradas.