use piston_window::*;
use rand;

const BLUE: [f32; 4] = [0.0, 0.0, 0.8, 1.0];
const GRAY: [f32; 4] = [0.7, 0.7, 0.7, 0.8];
const YELLOW: [f32; 4] = [0.98, 0.98, 0.0, 1.0];
const RED: [f32; 4] = [0.9, 0.0, 0.0, 1.0];
const LOW_RED: [f32; 4] = [0.9, 0.0, 0.0, 0.3];
const MEDIUM_RED: [f32; 4] = [0.9, 0.0, 0.0, 0.6];
const GREEN: [f32; 4] = [0.0, 0.7, 0.0, 1.0];
const LOW_GREEN: [f32; 4] = [0.0, 0.7, 0.0, 0.3];
const MEDIUM_GREEN: [f32; 4] = [0.0, 0.7, 0.0, 0.6];

const END_LINE: f64 = 40.0;
const DEPTH: f64 = 20.0;
const FIGURE_RADIUS: f64 = 10.0;

const FRAME_TIME: f64 = 0.02;

pub struct App {
    width: f64,
    height: f64,
    field: [i32; 9],
    ref_positions: [(f64, f64); 9],
    game_over: bool,
    win_sector: u32,
    game_over_status: u8,
    motion: bool,
    first_motion: bool,
    last_position: usize,
    time: f64,
    glyphs: Glyphs,
}

impl App {
    pub fn new(width: f64, height: f64, glyphs: Glyphs) -> App {
        let zero_x = width / 6.0;
        let zero_y = height / 6.0;
        
        let mut ref_positions = [(0.0, 0.0); 9];
        for i in 0..9 {
            ref_positions[i] = (
                zero_x + (width / 3.0) * (i % 3) as f64, 
                zero_y + (height / 3.0) * (i / 3) as f64
            );
        }
        let rand = rand::random::<bool>();

        App {
            width,
            height,
            field: [0; 9],
            ref_positions,
            game_over: false,
            win_sector: 0,
            game_over_status: 0,
            motion: rand,
            first_motion: rand,
            last_position: 10,
            time: 0.0,
            glyphs,
        }
    }

    fn draw_circle(&self, con: &Context, g: &mut G2d, position: (f64, f64), paint:types::Color) {
        let circle_width = self.width / 5.0;
        let circle_height = self.height / 5.0;
        let position_x = position.0 - circle_width / 2.0;
        let position_y = position.1 - circle_height / 2.0;
        circle_arc(paint, FIGURE_RADIUS, 0.0, std::f64::consts::PI * 2.0, [position_x, position_y, circle_width, circle_height], con.transform, g);
    }

    fn draw_cross(&self, con: &Context, g: &mut G2d, position: (f64, f64), paint:types::Color) {
        let line_width =  self.width / 9.0;
        let line_height = self.height / 9.0;
        let position_x = position.0 - line_width;
        let position_y = position.1 - line_height;

        let position_xp = position.0 + line_width;
        let position_yp = position.1 + line_height;
        
        line(paint, FIGURE_RADIUS, [position_x, position_y, position_xp, position_yp], con.transform, g);
        line(paint, FIGURE_RADIUS, [position_x, position_yp, position_xp, position_y], con.transform, g);
    }

    fn draw_grid(&self, con: &Context, g: &mut G2d) {
        let first_line = self.width / 3.0;
        let second_line = self.height / 3.0;
        rectangle(YELLOW, [first_line - (DEPTH / 2.0), 0.0, DEPTH, self.height], con.transform, g);
        rectangle(YELLOW, [first_line * 2.0 - (DEPTH / 2.0), 0.0, DEPTH, self.height], con.transform, g);
        rectangle(YELLOW, [0.0, second_line - (DEPTH / 2.0), self.width, DEPTH], con.transform, g);
        rectangle(YELLOW, [0.0, second_line * 2.0 - (DEPTH / 2.0), self.width, DEPTH], con.transform, g);
    }

    fn draw_end_line(&self, con: &Context, g: &mut G2d) {
        if self.game_over_status == 3 {return}
        let mut win_line: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        let one_of_six_width = self.width / 6.0;
        let one_of_six_height = self.height / 6.0;

        match self.win_sector {
            1 => {win_line = [one_of_six_width, 0.0, one_of_six_width, self.height]},
            2 => {win_line = [one_of_six_width * 3.0, 0.0, one_of_six_width * 3.0, self.height]},
            3 => {win_line = [one_of_six_width * 5.0, 0.0, one_of_six_width * 5.0, self.height]},
            4 => {win_line = [0.0, one_of_six_height, self.width, one_of_six_height]},
            5 => {win_line = [0.0, one_of_six_height * 3.0, self.width, one_of_six_height * 3.0]},
            6 => {win_line = [0.0, one_of_six_height * 5.0, self.width, one_of_six_height * 5.0]},
            7 => {win_line = [0.0, 0.0, self.width, self.height]},
            8 => {win_line = [self.width, 0.0, 0.0, self.height]},
            _ => (),
        }
        let mut color = MEDIUM_GREEN;
        if self.motion{color = MEDIUM_RED}
        line(color, END_LINE, win_line, con.transform, g); 
    }

    fn draw_figures(&mut self, con: &Context, g: &mut G2d) {
        for i in 0..9 {
            if self.field[i] == 3 && self.last_position != i || self.field[i] == 4 && self.last_position != i {
                self.field[i] = 0;
            }
            match self.field[i] {
                1 => self.draw_circle(con, g, self.ref_positions[i], RED),
                2 => self.draw_cross(con, g, self.ref_positions[i], GREEN),
                3 => self.draw_circle(con, g, self.ref_positions[i], LOW_RED),
                4 => self.draw_cross(con, g, self.ref_positions[i], LOW_GREEN),
                _ => (),
            }
        }
    }

    fn game_over(&mut self, con: &Context, g: &mut G2d, d: &mut GfxDevice){
        let mut current_color: [f32; 4] = BLUE;
        let mut win: &str = "";
        const GREEN_WIN: &str = "Green wins!";
        const RED_WIN: &str = "Red wins!";
        const DRAW: &str = "Draw!";

        match self.game_over_status {
            1 => {current_color = GREEN; win = GREEN_WIN},
            2 => {current_color = RED; win = RED_WIN},
            3 => {current_color = BLUE; win = DRAW},
            _ => (),
        }
        let _ = text(current_color, 96, "Game Over!", &mut self.glyphs, con.transform.trans(20.0, 100.0), g);
        let _ = text(current_color, 96, win, &mut self.glyphs, con.transform.trans(20.0, 200.0), g);
        let _ = text(current_color, 32, "Click to play again", &mut self.glyphs, con.transform.trans(20.0, self.height - 20.0), g);

        self.glyphs.factory.encoder.flush(d);
    }

    fn restart_game(&mut self)
    {
        self.field = [0; 9];
        self.last_position = 0;
        self.game_over = false;
        self.game_over_status = 0;
        self.last_position = 10;
        self.time = 0.0;
        self.first_motion = !self.first_motion;
        self.motion = self.first_motion;
    }

    fn ret_square(&self, position: [f64; 2]) -> usize {
        let mut _square  = 0;
        let ref1 = self.width / 3.0;
        let ref2 = self.height / 3.0;
        let mut ref_up1 = ref1;
        let mut ref_up2 = ref2;

        for _ in 0..3 {
            if position[0] >= ref_up1
            {
                _square += 1;
                ref_up1 += ref1;
                continue;
            }
            for _ in 0..3 {
                if position[1] >= ref_up2
                {
                    _square += 3;
                    ref_up2 += ref2;
                    continue;
                } 
                break;
            }  
        }
        _square
    }

    fn check_game_over(&mut self) {
        if self.field[0] == self.field[1] && self.field[1] == self.field[2] && self.field[0] != 0 {
            self.game_over = true;
            self.win_sector = 4;
        } else if self.field[3] == self.field[4] && self.field[4] == self.field[5] && self.field[3] != 0 {
            self.game_over = true;
            self.win_sector = 5;
        } else if self.field[6] == self.field[7] && self.field[7] == self.field[8] && self.field[6] != 0 {
            self.game_over = true;
            self.win_sector = 6;
        } else if self.field[0] == self.field[3] && self.field[3] == self.field[6] && self.field[0] != 0 {
            self.game_over = true;
            self.win_sector = 1;
        } else if self.field[1] == self.field[4] && self.field[4] == self.field[7] && self.field[1] != 0 {
            self.game_over = true;
            self.win_sector = 2;
        } else if self.field[2] == self.field[5] && self.field[5] == self.field[8] && self.field[2] != 0 {
            self.game_over = true;
            self.win_sector = 3;
        } else if self.field[0] == self.field[4] && self.field[4] == self.field[8] && self.field[0] != 0 {
            self.game_over = true;
            self.win_sector = 7;
        } else if self.field[2] == self.field[4] && self.field[4] == self.field[6] && self.field[2] != 0 {
            self.game_over = true;
            self.win_sector = 8;
        }

        if self.game_over == true
        {
            if self.motion {self.game_over_status = 1} else {self.game_over_status = 2}
            return;
        }

        let mut _num = 0;
        for i in 0..9 {
            if self.field[i] == 0 {
                _num += 1;
            }
        }
        if _num == 0 {
            self.game_over = true;
            self.game_over_status = 3;
        }
    }

    pub fn draw(&mut self, con: &Context, g: &mut G2d, d: &mut GfxDevice) {
        self.draw_grid(con, g);
        self.draw_figures(con, g);
        if self.game_over {
            self.draw_end_line(con, g);
            rectangle(GRAY, [0.0, 0.0, self.width, self.height], con.transform, g);
            self.game_over(con, g, d);
        }
    }

    pub fn on_click(&mut self, position: [f64; 2]) {
        if self.game_over {self.restart_game(); return}

        let _square = self.ret_square(position);

        if self.field[_square] == 1 || self.field[_square] == 2 {
            return;
        }

        if self.motion {
            self.field[_square] = 2;
        }
        else {
            self.field[_square] = 1;
        }
        self.check_game_over();
        self.motion = !self.motion;
    }

    pub fn update(&mut self, delta_time: f64, position: [f64; 2]) {
        if self.game_over {return}

        self.time += delta_time;

        if self.time > FRAME_TIME {
            let _square = self.ret_square(position);

            if self.field[_square] == 1 || self.field[_square] == 2 {
                return;
            }
            self.last_position = _square;

            if self.motion {
                self.field[_square] = 4;
            }
            else {
                self.field[_square] = 3;
            }
            self.time = 0.0;
        }
    }
}
