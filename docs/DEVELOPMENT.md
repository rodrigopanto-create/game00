# Desarrollo

## Comandos de trabajo

```bash
cargo run
cargo check
cargo test
cargo fmt
cargo clippy -- -D warnings
```

## Convenciones recomendadas

- Mantener `main.rs` minimo.
- Exponer API publica del crate desde `lib.rs`.
- Evitar `unwrap()` en cargas de recursos cuando se agreguen assets.
- Separar logica de juego por responsabilidad.

## Estructura sugerida para escalar

```text
src/
  main.rs
  lib.rs
  entities.rs
  game/
    mod.rs
    input.rs
    update.rs
    collision.rs
    render.rs
```

## Manejo de assets

Si luego se agregan sprites, usar:

- Carpeta `assets/` en la raiz del proyecto.
- Carga con rutas relativas estables.
- Fallback visual cuando un recurso no cargue.

## Checklist antes de merge

- `cargo check` sin errores.
- `cargo fmt` aplicado.
- `cargo clippy -- -D warnings` sin warnings.
- README actualizado si cambia gameplay o setup.