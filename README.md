# Game00

Shooter arcade 2D hecho en Rust con [Macroquad](https://macroquad.rs/). El jugador controla una nave triangular, dispara a objetivos que descienden y trata de sobrevivir mientras aumenta la dificultad.

## Requisitos

- Rust estable y Cargo.
- Para generar el APK en CI: herramientas Android compatibles con `cargo-apk`.

Comprueba la instalacion de Rust:

```sh
rustc --version
cargo --version
```

## Ejecutar

Desde la raiz del repositorio:

```sh
cargo run
```

Verificar, probar y formatear:

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## Controles

| Accion | Teclas | Pantalla tactil / raton |
| --- | --- | --- |
| Iniciar o continuar desde la pantalla inicial | Espacio o Enter | Boton FIRE (abajo a la derecha) |
| Moverse a la izquierda | Flecha izquierda o A | Boton flecha izquierda (abajo a la izquierda) |
| Moverse a la derecha | Flecha derecha o D | Boton flecha derecha (abajo a la izquierda) |
| Disparar | Espacio o Enter | Boton FIRE |
| Continuar desde Game Over / volver al menu desde High Scores | Espacio o Enter | Boton FIRE |
| Cambiar letra de las iniciales | Flechas (arriba/abajo/izquierda/derecha), W/S/A/D | Botones de flecha (mantener para avanzar rapido) |
| Confirmar letra | Espacio o Enter | Boton FIRE |
| Volver a la letra anterior | Retroceso (Backspace) | - |

Los botones virtuales semitransparentes se dibujan siempre sobre el juego y admiten multitoque (moverse y disparar a la vez).

## High Scores

- Se guarda un top 10 ordenado de mayor a menor puntaje.
- Si el puntaje final es mayor que 0 y hay hueco en la tabla o supera al ultimo, aparece `NEW RECORD!` para introducir 3 iniciales estilo arcade.
- Despues se muestra la tabla (la nueva entrada parpadea) durante 10 s o hasta pulsar FIRE.
- Persistencia: en escritorio se guarda en `highscores.txt` en el directorio de trabajo; en web, en `localStorage` (clave `game00_highscores`) mediante el plugin JS de `static/index.html`. Si no se puede escribir (p. ej. Android), la tabla dura solo la sesion.

## Mecanicas

- La nave verde triangular se mueve horizontalmente y permanece dentro de la pantalla.
- Las balas viajan hacia arriba. La colision entre balas y objetivos usa rectangulos AABB.
- Aparece un objetivo cuando el temporizador supera el intervalo actual de aparicion.
- La velocidad base de los objetivos aumenta con el tiempo hasta un maximo de 550 px/s; el intervalo de aparicion disminuye hasta 0.3 s.
- Perder un objetivo que alcanza la parte inferior quita 15 puntos de vida. La partida termina al llegar a cero.
- Destruir un objetivo estandar o ZigZag da 100 puntos; destruir uno Fast da 200.

### Tipos de objetivo

| Tipo | Apariencia | Tamano | Movimiento | Puntos |
| --- | --- | --- | --- | --- |
| `Standard` | Rojo | 35 x 35 | Desciende a velocidad base | 100 |
| `Fast` | Naranja | 20 x 20 | Desciende al doble de la velocidad base | 200 |
| `ZigZag` | Morado/azul | 35 x 35 | Desciende a velocidad base y oscila horizontalmente | 100 |

Los tipos se seleccionan aleatoriamente al aparecer. El HUD muestra el puntaje y la vida restante.

## Estructura del proyecto

```text
src/
  main.rs                 # Entrada Macroquad
  lib.rs                  # Modulos publicos y reexportacion de run()
  entities.rs             # Bullet, Target y TargetType
  game.rs                 # Loop principal y transiciones de estado
  game/
    controls.rs           # Entrada unificada (teclado/tactil/raton) y botones virtuales
    highscores.rs         # Ranking top 10, persistencia y entrada de iniciales
    screens.rs            # Pantallas inicial, Game Over, nuevo record y High Scores
    session.rs            # Estado y coordinacion de la partida
    session/
      player.rs           # Entrada, movimiento, disparo y dibujo del jugador
      world.rs            # Aparicion, movimiento, colisiones y dibujo de entidades
      hud.rs              # Puntaje y barra de vida
docs/
  ARCHITECTURE.md         # Responsabilidades y flujo de ejecucion
  DEVELOPMENT.md          # Comandos y guia para contribuir
.github/
  workflows/
    android.yml           # Construccion del APK en GitHub Actions
```

## Construccion Android

El workflow [android.yml](.github/workflows/android.yml) se ejecuta en cada `push` a `main`. Construye un APK de release con `cargo-apk` desde `target/android-artifacts/` y lo publica como artefacto de GitHub Actions con el nombre `game00-android`. Consulta [Desarrollo](docs/DEVELOPMENT.md) para los detalles del workflow.

## Documentacion

- [Arquitectura](docs/ARCHITECTURE.md)
- [Desarrollo](docs/DEVELOPMENT.md)

## Licencia

Todavia no definida.
