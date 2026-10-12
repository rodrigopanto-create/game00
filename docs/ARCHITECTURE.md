# Arquitectura

## Vision general

El crate se organiza en entidades y modulos de juego. Macroquad mantiene el loop de renderizado: en cada frame se lee el estado de entrada, se actualiza la partida cuando corresponde, se dibuja la pantalla activa y se espera el siguiente frame.

## Modulos

### Punto de entrada y API

- `src/main.rs` inicia Macroquad con el titulo `Macroquad Shooter` y llama a `game00::run()`. En Windows, oculta la consola con `windows_subsystem`.
- `src/lib.rs` expone `entities` y `game`, y reexporta `run()` para que el ejecutable tenga un punto de entrada simple.
- `src/entities.rs` contiene los datos de las entidades:
  - `Bullet`: rectangulo, velocidad vertical y estado activo.
  - `Target`: rectangulo, estado activo, tipo y datos de movimiento ZigZag.
  - `TargetType`: comportamiento, tamano, color y puntuacion de cada tipo.

### Loop y pantallas

- `src/game.rs` controla el loop y las transiciones entre `StartScreen`, `Playing` y `GameOver`.
- `src/game/screens.rs` dibuja la pantalla inicial y la pantalla de fin de partida.
- `src/game/session.rs` mantiene el estado de una partida: jugador, entidades, vida, puntaje y dificultad. Coordina la actualizacion y el renderizado.
- `src/game/session/player.rs` procesa movimiento/disparo y dibuja la nave.
- `src/game/session/world.rs` crea objetivos, actualiza balas y objetivos, resuelve colisiones, elimina entidades inactivas y dibuja el mundo.
- `src/game/session/hud.rs` dibuja el puntaje y la barra de vida.

## Ciclo de vida y flujo por frame

1. `run()` obtiene el tiempo transcurrido desde el frame anterior.
2. En `StartScreen`, acepta Espacio o Enter, reinicia la sesion e inicia la partida.
3. En `Playing`, `GameSession::update()` actualiza la dificultad, el jugador y el mundo. Si la vida llega a cero, el estado cambia a `GameOver`.
4. La sesion dibuja fondo, nave, balas, objetivos y HUD.
5. En `GameOver`, se muestra el puntaje final. Espacio o Enter reinicia los datos y vuelve a la pantalla inicial.
6. `next_frame().await` cede el control a Macroquad.

El orden de actualizacion del mundo es: aparicion de objetivos, movimiento de balas, movimiento de objetivos y perdida de vida, colisiones, y limpieza de entidades inactivas.

## Tipos de objetivo

`TargetType` concentra las propiedades comunes de cada variante:

| Tipo | Tamano | Multiplicador de velocidad | Color | Puntos |
| --- | ---: | ---: | --- | ---: |
| `Standard` | 35 px | 1x | Rojo | 100 |
| `Fast` | 20 px | 2x | Naranja | 200 |
| `ZigZag` | 35 px | 1x | Morado/azul | 100 |

La velocidad base sube desde 150 px/s en funcion del tiempo de partida, hasta 550 px/s. El intervalo de aparicion baja desde 1.0 s hasta un minimo de 0.3 s. Los objetivos ZigZag suman una oscilacion seno horizontal a su movimiento descendente.

Todos los objetivos usan su `Rect` tanto para dibujarse como para colisionar. Las balas y objetivos se retiran cuando dejan de estar activos.

## Automatizacion Android

`.github/workflows/android.yml` ejecuta en GitHub Actions al hacer `push` a `main`. Instala el target Rust `aarch64-linux-android`, instala `cargo-apk`, construye el APK release y carga los APK de `target/android-artifacts/` como artefacto `game00-android`. El workflow falla si no se encuentra ningun APK.

## Decisiones

- Macroquad gestiona el ciclo de vida de la ventana, entrada y render.
- Las entidades usan rectangulos para mantener sencilla la deteccion AABB y la representacion visual.
- Los modulos de `session` son privados al modulo de juego y separan responsabilidades sin exponer el estado interno como API publica.
- El jugador y los objetivos se dibujan con primitivas; no requieren archivos de imagen externos.
