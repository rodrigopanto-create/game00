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
- Disparar: Espacio

## Como esta organizado

```text
src/
  main.rs      # Entry point minimo
  lib.rs       # Exporta modulos y API publica
  entities.rs  # Tipos de entidades (Bullet, Target)
  game.rs      # Loop del juego, update y render
```

Documentacion ampliada:

- docs/ARCHITECTURE.md
- docs/DEVELOPMENT.md

## Mecanicas actuales

- Spawn de objetivos cada 1 segundo.
- Proyectiles con velocidad fija hacia arriba.
- Objetivos que descienden en vertical.
- Colision AABB usando `Rect::overlaps`.
- Score: +100 por objetivo destruido.

## Proximos pasos sugeridos

- Separar `game.rs` en modulos de `input`, `update` y `render`.
- Cambiar textura generada por sprites reales en carpeta `assets/`.
- Agregar estados de juego (menu, jugando, game over).
- Agregar sonido y efectos visuales.

## Comandos utiles

```bash
cargo check
cargo fmt
cargo clippy -- -D warnings
```

## Licencia

Todavia no definida.