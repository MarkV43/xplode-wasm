mod utils;

use wasm_bindgen::prelude::*;
use xplode_core::game::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct WasmValue {
    pub number: u8,
    pub bomb: bool,
}

impl From<TileValue> for WasmValue {
    fn from(value: TileValue) -> Self {
        let number = match value {
            TileValue::Bomb => 0,
            TileValue::Safe(n) => n,
        };

        let bomb = matches!(value, TileValue::Bomb);

        Self { number, bomb }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum WasmState {
    Hidden = 0,
    Flag = 1,
    Open = 2,
}

impl From<TileState> for WasmState {
    fn from(value: TileState) -> Self {
        match value {
            TileState::Flag => WasmState::Flag,
            TileState::Hidden => WasmState::Hidden,
            TileState::Open => WasmState::Open,
        }
    }
}

#[wasm_bindgen]
pub struct WasmTile {
    pub state: WasmState,
    pub value: Option<WasmValue>,
}

impl From<(&TileState, Option<&TileValue>)> for WasmTile {
    fn from(value: (&TileState, Option<&TileValue>)) -> Self {
        let state: WasmState = value.0.clone().into();
        let value: Option<WasmValue> = value.1.map(|k| k.clone().into());

        Self { state, value }
    }
}

#[wasm_bindgen]
pub struct WasmGame {
    game: Game,
}

#[wasm_bindgen]
impl WasmGame {
    pub fn new(width: usize, height: usize, bombs: usize) -> Self {
        Self {
            game: Game::new(width, height, bombs),
        }
    }

    pub fn new_safe(width: usize, height: usize, bombs: usize, cx: usize, cy: usize) -> Self {
        Self {
            game: Game::new_safe(width, height, bombs, cx, cy),
        }
    }

    pub fn new_safe_zero(width: usize, height: usize, bombs: usize, cx: usize, cy: usize) -> Self {
        Self {
            game: Game::new_safe_zero(width, height, bombs, cx, cy),
        }
    }

    pub fn new_safe_zero_seeded(
        width: usize,
        height: usize,
        bombs: usize,
        cx: usize,
        cy: usize,
        seed: u64,
    ) -> Self {
        Self {
            game: Game::new_safe_zero_seeded(width, height, bombs, cx, cy, seed),
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) -> Option<WasmValue> {
        self.game.reveal(x, y).map(|v| v.into())
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        self.game.flag(x, y)
    }

    pub fn set_flag(&mut self, x: usize, y: usize, flag: bool) {
        self.game.set_flag(x, y, flag)
    }

    pub fn get(&self, x: usize, y: usize) -> WasmTile {
        self.game.get(x, y).into()
    }

    pub fn get_width(&self) -> usize {
        self.game.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.game.get_height()
    }
}
