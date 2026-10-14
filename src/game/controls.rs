use macroquad::prelude::*;

const BUTTON_SIZE: f32 = 80.0;
const BUTTON_GAP: f32 = 20.0;
const SCREEN_MARGIN: f32 = 20.0;
const FIRE_RADIUS: f32 = 50.0;

const BUTTON_FILL: Color = Color::new(1.0, 1.0, 1.0, 0.3);
const BUTTON_FILL_ACTIVE: Color = Color::new(1.0, 1.0, 1.0, 0.5);
const BUTTON_OUTLINE: Color = Color::new(1.0, 1.0, 1.0, 0.4);
const BUTTON_SYMBOL: Color = Color::new(1.0, 1.0, 1.0, 0.6);

/// Estado de entrada unificado del frame actual (teclado + pantalla táctil + ratón).
#[derive(Default, Clone, Copy)]
pub(super) struct Controls {
    pub(super) left: bool,
    pub(super) right: bool,
    /// Flancos de pulsación (solo el primer frame), útiles para navegar menús.
    pub(super) left_pressed: bool,
    pub(super) right_pressed: bool,
    pub(super) up_pressed: bool,
    pub(super) down_pressed: bool,
    pub(super) back_pressed: bool,
    /// Flanco de pulsación del botón Disparo / Start (equivale a Space/Enter pulsado).
    pub(super) action_pressed: bool,
    pub(super) action_down: bool,
}

struct Layout {
    left: Rect,
    right: Rect,
    fire_center: Vec2,
}

impl Layout {
    fn current() -> Self {
        let y = screen_height() - BUTTON_SIZE - SCREEN_MARGIN;
        Self {
            left: Rect::new(SCREEN_MARGIN, y, BUTTON_SIZE, BUTTON_SIZE),
            right: Rect::new(
                SCREEN_MARGIN + BUTTON_SIZE + BUTTON_GAP,
                y,
                BUTTON_SIZE,
                BUTTON_SIZE,
            ),
            fire_center: vec2(
                screen_width() - SCREEN_MARGIN - FIRE_RADIUS,
                screen_height() - SCREEN_MARGIN - FIRE_RADIUS,
            ),
        }
    }

    fn fire_hit(&self, p: Vec2) -> bool {
        // Caja delimitadora del círculo de disparo.
        Rect::new(
            self.fire_center.x - FIRE_RADIUS,
            self.fire_center.y - FIRE_RADIUS,
            FIRE_RADIUS * 2.0,
            FIRE_RADIUS * 2.0,
        )
        .contains(p)
    }
}

impl Controls {
    /// Lee todas las fuentes de entrada. Debe llamarse una sola vez por frame.
    pub(super) fn poll() -> Self {
        let layout = Layout::current();

        let mut controls = Self {
            left: is_key_down(KeyCode::Left) || is_key_down(KeyCode::A),
            right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
            left_pressed: is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A),
            right_pressed: is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D),
            up_pressed: is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W),
            down_pressed: is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S),
            back_pressed: is_key_pressed(KeyCode::Backspace),
            action_pressed: is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter),
            action_down: false,
        };

        for touch in touches() {
            let held = !matches!(touch.phase, TouchPhase::Ended | TouchPhase::Cancelled);
            controls.apply_pointer(&layout, touch.position, held, touch.phase == TouchPhase::Started);
        }

        // Clic de ratón en escritorio (en móvil los toques también lo simulan; el OR evita duplicados).
        let mouse = Vec2::from(mouse_position());
        controls.apply_pointer(
            &layout,
            mouse,
            is_mouse_button_down(MouseButton::Left),
            is_mouse_button_pressed(MouseButton::Left),
        );

        controls
    }

    fn apply_pointer(&mut self, layout: &Layout, pos: Vec2, held: bool, started: bool) {
        if held {
            self.left |= layout.left.contains(pos);
            self.right |= layout.right.contains(pos);
        }
        if started {
            self.left_pressed |= layout.left.contains(pos);
            self.right_pressed |= layout.right.contains(pos);
        }
        if layout.fire_hit(pos) {
            self.action_down |= held;
            self.action_pressed |= started;
        }
    }

    /// Dibuja los botones virtuales en la capa de UI (llamar al final del frame).
    pub(super) fn draw(&self) {
        set_default_camera();
        let layout = Layout::current();

        draw_arrow_button(layout.left, self.left, true);
        draw_arrow_button(layout.right, self.right, false);

        let c = layout.fire_center;
        let fill = if self.action_down { BUTTON_FILL_ACTIVE } else { BUTTON_FILL };
        draw_circle(c.x, c.y, FIRE_RADIUS, fill);
        draw_circle_lines(c.x, c.y, FIRE_RADIUS, 2.0, BUTTON_OUTLINE);
        let label = "FIRE";
        let size = measure_text(label, None, 24, 1.0);
        draw_text(
            label,
            c.x - size.width / 2.0,
            c.y + size.offset_y / 2.0,
            24.0,
            BUTTON_SYMBOL,
        );
    }
}

fn draw_arrow_button(rect: Rect, active: bool, points_left: bool) {
    let fill = if active { BUTTON_FILL_ACTIVE } else { BUTTON_FILL };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BUTTON_OUTLINE);

    let center = rect.center();
    let half = rect.w * 0.25;
    let (tip, back) = if points_left {
        (center.x - half, center.x + half)
    } else {
        (center.x + half, center.x - half)
    };
    draw_triangle(
        vec2(tip, center.y),
        vec2(back, center.y - half),
        vec2(back, center.y + half),
        BUTTON_SYMBOL,
    );
}
