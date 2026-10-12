# Desarrollo

## Requisitos

- Rust estable y Cargo.
- Conexion a internet para descargar dependencias la primera vez.
- Para construir Android: GitHub Actions ejecuta el workflow incluido; una compilacion local requiere las herramientas Android que necesite `cargo-apk`.

## Comandos

Desde la raiz del repositorio:

```sh
cargo run
cargo check
cargo test
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

`cargo test` ejecuta las pruebas unitarias, de integracion y de documentacion disponibles. Si aun no se han agregado pruebas, Cargo informara que no hay casos que ejecutar.

## Organizacion del codigo

```text
src/
  main.rs
  lib.rs
  entities.rs
  game.rs
  game/
    screens.rs
    session.rs
    session/
      player.rs
      world.rs
      hud.rs
```

Mantener `main.rs` como punto de entrada, `game.rs` enfocado en estados/transiciones y las responsabilidades de la sesion en sus submodulos. Cuando una entidad o una mecanica cambie, actualizar las pruebas y la documentacion correspondiente.

## Agregar o modificar tipos de objetivo

Las propiedades de cada tipo se definen en `TargetType` (`src/entities.rs`): tamano, multiplicador de velocidad, puntos y color. La aparicion, el movimiento especial y el uso de esas propiedades se encuentran en `src/game/session/world.rs`. Al agregar un tipo:

1. Agregar la variante y sus propiedades en todos los `match` de `TargetType`.
2. Definir como se instancia y actualiza en el mundo.
3. Verificar que la colision use el rectangulo del tamano correcto.
4. Actualizar las tablas de mecanicas en `README.md` y `docs/ARCHITECTURE.md`.

## Android en GitHub Actions

El archivo `.github/workflows/android.yml` se ejecuta en cada `push` a la rama `main`. El job:

1. Descarga el repositorio.
2. Instala el target `aarch64-linux-android`.
3. Instala `cargo-apk`.
4. Construye el APK de release.
5. Sube los APK encontrados en `target/android-artifacts/` como artefacto `game00-android`. El paso de subida falla si la compilacion no produjo archivos.

Los artefactos se consultan en la pagina del workflow run correspondiente en GitHub Actions. Los cambios publicados en ramas distintas a `main` no disparan este workflow.

## Checklist antes de publicar

- Ejecutar `cargo fmt --all`.
- Ejecutar `cargo check` y `cargo test`.
- Ejecutar `cargo clippy --all-targets -- -D warnings` cuando sea posible.
- Actualizar README y documentacion de arquitectura si cambia el comportamiento, estructura o compilacion.
- Revisar `git diff` y confirmar que no se incluyan archivos generados ni secretos.

## Assets

El juego actual dibuja la nave, balas y objetivos con primitivas de Macroquad y no necesita assets externos. Si se agregan recursos, documentar su ubicacion, carga y requisitos de compilacion en el README.
