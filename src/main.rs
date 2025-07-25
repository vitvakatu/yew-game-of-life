use gloo_timers::callback::Interval;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct App {
    width: u32,
    height: u32,
    _interval: Option<Interval>,
    cells: Vec<Cell>,
}

impl App {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn new(interval: Option<Interval>) -> App {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        App {
            width,
            height,
            _interval: interval,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    fn create_interval(ctx: &Context<App>) -> Interval {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(12, move || callback.emit(()));
        interval
    }
}

fn view_cell(idx: usize, cell: &Cell, link: &Scope<App>) -> Html {
    let status = match cell {
        Cell::Alive => "cellule-live",
        Cell::Dead => "cellule-dead",
    };
    let onclick = link.callback(move |_| Msg::Click(idx));

    html! {
        <div key={idx} class={classes!("game-cellule", status)} onclick={onclick}></div>
    }
}

pub enum Msg {
    Tick,
    StartStop,
    Click(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let interval = Self::create_interval(ctx);

        Self::new(Some(interval))
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => self.tick(),
            Msg::StartStop => {
                if self._interval.take().is_some() {
                } else {
                    self._interval = Some(Self::create_interval(ctx));
                }
            }
            Msg::Click(idx) => {
                self.cells[idx] = match self.cells[idx] {
                    Cell::Alive => Cell::Dead,
                    Cell::Dead => Cell::Alive,
                };
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self
            .cells
            .chunks(self.width as usize)
            .enumerate()
            .map(|(y, cells)| {
                let cells = cells.iter().enumerate().map(|(x, cell)| {
                    let idx = x + y * self.width as usize;
                    view_cell(idx, cell, ctx.link())
                });
                html! {
                    <div key={y} class="game-row">
                        { for cells }
                    </div>
                }
            });
        let onclick = ctx.link().callback(|_| Msg::StartStop);
        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <h1 class="app-title">{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                    </section>
                    <button onclick={onclick}>{ "Start stop" }</button>
                </section>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_neighbor_count() {
        let mut app = App::new(None);
        app.cells = vec![Cell::Dead; 64 * 64];
        for i in 0..64 {
            for j in 0..64 {
                assert_eq!(app.live_neighbor_count(i, j), 0);
            }
        }
        // Верхний левый угол
        app.cells[0] = Cell::Alive;
        assert_eq!(app.live_neighbor_count(0, 0), 0);
        app.cells[1] = Cell::Alive;
        assert_eq!(app.live_neighbor_count(0, 0), 1);
        // Поле замкнуто со всех сторон (представляет собой поверхность тора)
        assert_eq!(app.live_neighbor_count(63, 0), 2);
        assert_eq!(app.live_neighbor_count(0, 63), 1);
        assert_eq!(app.live_neighbor_count(63, 63), 1);
        // Правый верхний угол
        app.cells[63] = Cell::Alive;
        assert_eq!(app.live_neighbor_count(1, 63), 2);
    }
}
