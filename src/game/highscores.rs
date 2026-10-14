use super::Controls;

pub(super) const MAX_ENTRIES: usize = 10;
pub(super) const INITIALS_LEN: usize = 3;

const REPEAT_DELAY: f32 = 0.4;
const REPEAT_INTERVAL: f32 = 0.09;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ScoreEntry {
    pub(super) initials: [u8; INITIALS_LEN],
    pub(super) score: u32,
}

impl ScoreEntry {
    pub(super) fn initials_str(&self) -> &str {
        std::str::from_utf8(&self.initials).unwrap_or("???")
    }
}

/// Tabla de récords ordenada de mayor a menor puntaje.
pub(super) struct HighScores {
    entries: Vec<ScoreEntry>,
}

impl HighScores {
    pub(super) fn load() -> Self {
        let entries = storage::load().map(|data| parse(&data)).unwrap_or_default();
        Self { entries }
    }

    pub(super) fn entries(&self) -> &[ScoreEntry] {
        &self.entries
    }

    /// Un puntaje entra al ranking si hay hueco libre o supera al último.
    pub(super) fn qualifies(&self, score: u32) -> bool {
        score > 0
            && (self.entries.len() < MAX_ENTRIES
                || self.entries.last().is_some_and(|last| score > last.score))
    }

    /// Inserta el puntaje, persiste la tabla y devuelve la posición obtenida.
    pub(super) fn insert(&mut self, initials: [u8; INITIALS_LEN], score: u32) -> Option<usize> {
        if !self.qualifies(score) {
            return None;
        }
        // Ante empate, el récord más antiguo conserva la posición superior.
        let rank = self.entries.partition_point(|entry| entry.score >= score);
        self.entries.insert(rank, ScoreEntry { initials, score });
        self.entries.truncate(MAX_ENTRIES);
        storage::save(&serialize(&self.entries));
        Some(rank)
    }
}

fn serialize(entries: &[ScoreEntry]) -> String {
    entries
        .iter()
        .map(|entry| format!("{} {}\n", entry.initials_str(), entry.score))
        .collect()
}

fn parse(data: &str) -> Vec<ScoreEntry> {
    let mut entries: Vec<ScoreEntry> = data
        .lines()
        .filter_map(|line| {
            let (initials, score) = line.trim().split_once(' ')?;
            let initials: [u8; INITIALS_LEN] = initials.as_bytes().try_into().ok()?;
            if !initials.iter().all(u8::is_ascii_uppercase) {
                return None;
            }
            Some(ScoreEntry {
                initials,
                score: score.trim().parse().ok()?,
            })
        })
        .collect();
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.score));
    entries.truncate(MAX_ENTRIES);
    entries
}

/// Selector de iniciales estilo arcade: se recorre el alfabeto y se confirma letra por letra.
pub(super) struct InitialsEntry {
    letters: [u8; INITIALS_LEN],
    cursor: usize,
    repeat_timer: f32,
}

impl InitialsEntry {
    pub(super) fn new() -> Self {
        Self {
            letters: [b'A'; INITIALS_LEN],
            cursor: 0,
            repeat_timer: REPEAT_DELAY,
        }
    }

    pub(super) fn letters(&self) -> &[u8; INITIALS_LEN] {
        &self.letters
    }

    pub(super) fn cursor(&self) -> usize {
        self.cursor
    }

    /// Devuelve las iniciales cuando el jugador confirma la última letra.
    pub(super) fn update(&mut self, controls: &Controls, delta: f32) -> Option<[u8; INITIALS_LEN]> {
        let mut step = 0;
        if controls.right_pressed || controls.up_pressed {
            step += 1;
        }
        if controls.left_pressed || controls.down_pressed {
            step -= 1;
        }

        // Mantener pulsada una flecha (tecla o botón táctil) recorre el alfabeto automáticamente.
        let held_dir = controls.right as i32 - controls.left as i32;
        if held_dir == 0 || controls.left_pressed || controls.right_pressed {
            self.repeat_timer = REPEAT_DELAY;
        } else {
            self.repeat_timer -= delta;
            if self.repeat_timer <= 0.0 {
                step += held_dir;
                self.repeat_timer += REPEAT_INTERVAL;
            }
        }

        if step != 0 {
            let letter = &mut self.letters[self.cursor];
            *letter = b'A' + ((*letter - b'A') as i32 + step).rem_euclid(26) as u8;
        }

        if controls.back_pressed && self.cursor > 0 {
            self.cursor -= 1;
        }

        if controls.action_pressed {
            if self.cursor + 1 == INITIALS_LEN {
                return Some(self.letters);
            }
            self.cursor += 1;
        }
        None
    }
}

#[cfg(target_arch = "wasm32")]
mod storage {
    // Implementadas por el plugin JS registrado en static/index.html (usa localStorage).
    #[link(wasm_import_module = "env")]
    extern "C" {
        fn game00_storage_len() -> u32;
        fn game00_storage_read(ptr: *mut u8, max_len: u32) -> u32;
        fn game00_storage_write(ptr: *const u8, len: u32);
    }

    /// Permite al bundle de miniquad verificar la versión del plugin JS.
    #[no_mangle]
    pub extern "C" fn game00_storage_crate_version() -> u32 {
        1
    }

    pub(super) fn load() -> Option<String> {
        let len = unsafe { game00_storage_len() } as usize;
        if len == 0 {
            return None;
        }
        let mut buf = vec![0u8; len];
        let read = unsafe { game00_storage_read(buf.as_mut_ptr(), len as u32) } as usize;
        buf.truncate(read);
        String::from_utf8(buf).ok()
    }

    pub(super) fn save(data: &str) {
        unsafe { game00_storage_write(data.as_ptr(), data.len() as u32) }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod storage {
    const FILE_NAME: &str = "highscores.txt";

    // Si el archivo no se puede leer/escribir (p. ej. sandbox), el ranking vive solo en memoria.
    pub(super) fn load() -> Option<String> {
        std::fs::read_to_string(FILE_NAME).ok()
    }

    pub(super) fn save(data: &str) {
        let _ = std::fs::write(FILE_NAME, data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(scores: &[u32]) -> HighScores {
        HighScores {
            entries: scores
                .iter()
                .map(|&score| ScoreEntry {
                    initials: *b"AAA",
                    score,
                })
                .collect(),
        }
    }

    #[test]
    fn parse_skips_invalid_lines_and_sorts() {
        let entries = parse("BBB 10\nbad line\nabc 5\nCCC 30\nDD 7\n");
        let scores: Vec<u32> = entries.iter().map(|e| e.score).collect();
        assert_eq!(scores, vec![30, 10]);
        assert_eq!(entries[0].initials_str(), "CCC");
    }

    #[test]
    fn serialize_roundtrip() {
        let entries = table(&[300, 200, 100]).entries;
        assert_eq!(parse(&serialize(&entries)), entries);
    }

    #[test]
    fn qualifies_when_not_full_or_beats_last() {
        assert!(table(&[]).qualifies(1));
        assert!(!table(&[]).qualifies(0));
        let full = table(&[100; MAX_ENTRIES]);
        assert!(!full.qualifies(100));
        assert!(full.qualifies(101));
    }

    #[test]
    fn initials_wrap_alphabet_and_confirm() {
        let mut entry = InitialsEntry::new();
        let left = Controls {
            left_pressed: true,
            ..Default::default()
        };
        let fire = Controls {
            action_pressed: true,
            ..Default::default()
        };
        entry.update(&left, 0.016);
        assert_eq!(entry.letters()[0], b'Z');
        assert_eq!(entry.update(&fire, 0.016), None);
        assert_eq!(entry.update(&fire, 0.016), None);
        assert_eq!(entry.update(&fire, 0.016), Some(*b"ZAA"));
    }
}
