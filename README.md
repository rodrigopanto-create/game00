# Game00

Juego arcade 2D simple hecho con Rust y Macroquad.

## Estado del proyecto

- Base jugable implementada.
- Arquitectura modular separada en entidades y loop principal.
- Sin assets externos obligatorios: el jugador usa una textura generada en tiempo de ejecución.

## Requisitos

- Rust estable (recomendado usar la ultima version estable).
- Cargo.

Verificar instalacion:

```bash
rustc --version
cargo --version
```

## Ejecutar

Desde la raiz del proyecto:

```bash
cargo run
```

## Controles

- Mover a la izquierda: Flecha izquierda o A
- Mover a la derecha: Flecha derecha o D
- Disparar: Espacio o Enter (mantener para disparo continuo).
- Pantalla tactil o raton: botones de izquierda, derecha y FIRE; se puede mantener
  FIRE mientras se mueve la nave con otro dedo.

## Como esta organizado

```text
src/
  main.rs      # Entry point minimo
  lib.rs       # Exporta modulos y API publica
  entities.rs  # Enemigos, proyectiles, bonos, efectos y jefe
  game.rs      # Loop y estados de pantallas
  game/
    controls.rs # Entrada unificada y botones virtuales
    session.rs  # Partida y fases de cada nivel
    session/    # Actualizacion y dibujo de jugador, mundo y HUD
```

Documentacion ampliada:

- docs/ARCHITECTURE.md
- docs/DEVELOPMENT.md

## Mecanicas actuales

- Spawn inicial de objetivos cada 1 segundo; la velocidad y frecuencia aumentan
  con el tiempo y con cada nivel.
- Proyectiles con velocidad fija hacia arriba.
- Objetivos que descienden en vertical.
- Colision AABB usando `Rect::overlaps`.
- Score: +100 por enemigo normal, +200 por rapido y +300 por zigzag.
- Cada cuatro enemigos destruidos cae un bono de 24x24 desde el enemigo:
  `R` (RapidFire, naranja), `M` (MultiShot, azul), `S` (SpeedBoost, verde),
  en ese orden ciclico. Los enemigos que escapan no cuentan como bajas.
- Al tocar la nave, cada bono dura 8 segundos: R reduce el intervalo entre
  disparos de 0.25 a 0.1 segundos, M dispara tres balas y S aumenta la
  velocidad de 400 a 600 px/s. Los efectos distintos coexisten; recoger el
  mismo tipo renueva su duracion, sin acumular potencia. El HUD muestra
  el tiempo restante. Los bonos no recogidos desaparecen al salir de pantalla.
- El jefe aparece tras 12 bajas en el nivel 1; cada nivel exige 4 bajas mas.
  Al aparecer se retiran los enemigos y disparos del jugador que quedan en
  pantalla y se suspende el spawn normal. Los bonos caidos siguen disponibles.
- El jefe tiene barra de vida, movimiento lateral y rafagas de tres proyectiles
  en abanico. Empieza con 30 puntos de vida; cada bala del jugador quita uno.
  Sus disparos causan 10 de dano al jugador.
- Derrotarlo otorga `2000 * nivel` puntos y muestra una transicion de victoria
  de 2.5 segundos sin combate. Se limpian proyectiles y bonos pendientes;
  se conserva la vida, el score y los efectos activos (pausados en la transicion).
  El siguiente nivel aumenta la velocidad y frecuencia de enemigos, la vida
  del jefe (+10) y la velocidad y frecuencia de sus ataques.
- Nave, bonos, jefe y HUD comparten el render de escritorio y movil; la nave
  se mantiene por encima de los controles tactiles, que se dibujan al final.

## Proximos pasos sugeridos

- Cambiar textura generada por sprites reales en carpeta `assets/`.
- Agregar sonido y efectos visuales.

## Comandos utiles

```bash
cargo check
cargo fmt
cargo clippy -- -D warnings
```

## Licencia

Todavia no definida.