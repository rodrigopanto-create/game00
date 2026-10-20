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

## Cargador web

- `static/mq_js_bundle.js` se ejecuta en modo estricto. Los callbacks de plugins
  deben declararse en su propio scope: el plugin `quad_net` usa
  `var register_plugin` para evitar un `ReferenceError` al cargar el juego.
  Conservar esta correccion al reemplazar o regenerar el bundle.
- El service worker abre la cache con `caches.open`. Incrementar su version
  en `static/sw.js` al actualizar los recursos precargados para no reutilizar
  un cargador o un WASM anterior.
- El workflow de Pages ejecuta pruebas y Clippy, compila el binario con
  `cargo build --locked --release --target wasm32-unknown-unknown --bin game00`
  y copia el WASM a la raiz del sitio junto a los archivos estaticos.
  El cargador de Macroquad usa ese binario directamente, sin generar bindings
  adicionales con `wasm-bindgen-cli`.