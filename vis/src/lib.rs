#![allow(unused_imports)]
use itertools::Itertools;
use wasm_bindgen::prelude::*;
use std::cmp::*;
use std::collections::*;
use std::vec;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use rand::{Rng, SeedableRng};
use svg::node::{
    element::{Circle, Group, Line, Rectangle, Title},
    Text,
};

const T: usize = 1000;
const H: usize = 60;
const W: usize = 25;

#[derive(Clone, Copy)]
pub struct Enemy {
    is_empty: bool,
    x: usize,
    y: usize,
    fhp: usize,
    hp: usize,
    exp: usize, 
}

impl Enemy {
    pub fn new(is_empty: bool, x: usize, y: usize, fhp: usize, hp: usize, exp: usize) -> Enemy {
        Enemy { is_empty: (is_empty), x: (x), y: (y), fhp: (fhp), hp: (hp), exp: (exp) }
    }

    pub fn empty() -> Enemy {
        Enemy { is_empty: true, x: 0, y: 0, fhp: 0, hp: 0, exp: 0 }
    }
}

pub struct Input {
    pub ns: Vec<usize>,
    pub ves: Vec<Vec<Enemy>>,
}

pub fn parse_input(f: &str) -> Input {
    let v: Vec<usize> = f.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    
    let mut ns = vec![];
    let mut ves = vec![];
    let mut idx = 25;
    for _i in 0..T {
        let n = v[idx];
        ns.push(n);
        idx += 1;
        let mut es = vec![];
        for _j in 0..n {
            let hp = v[idx];
            let exp = v[idx+1];
            let y = v[idx+2];
            es.push(Enemy::new(false, 59, y, hp, hp, exp));
            idx += 3;
        }
        ves.push(es);
    }
    Input { ns, ves }
}

pub struct Output {
    pub action: Vec<char>,
}

pub fn parse_output(f: &str) -> Output {
    let action = f.chars()
        .filter(|&c| c != '\n')
        .collect();
    Output { action }
}

pub struct State {
    y: usize,
    score: usize,
    level: usize,
    total_exp: usize,
    grid: Vec<Vec<Enemy>>
}

impl State {
    pub fn new() -> State {
        State { y: (12), score: (0), level: (1), total_exp: (0), grid: (vec![vec![Enemy::empty(); W]; H]) }
    }

    pub fn advance(&mut self) -> Result<bool, &str> {
        for i in 0..H {
            for j in 0..W {
                if !self.grid[i][j].is_empty {
                    if i == 0 {
                        self.grid[i][j] = Enemy::empty();
                    } else if i == 1 && self.y == self.grid[i][j].y {
                        return Err("teki ga butukatta");
                    } else {
                        self.grid[i-1][j] = Enemy::new(false, i-1, j, self.grid[i][j].fhp, self.grid[i][j].hp, self.grid[i][j].exp);
                        self.grid[i][j] = Enemy::empty();
                    }
                }
            }
        }
        Ok(true)
    }

    pub fn add_enemies(&mut self, input: &Input, t: usize) {
        for i in 0..input.ns[t] {
            let e = input.ves[t][i].clone();
            self.grid[e.x][e.y] = e;
        }
    }

    pub fn walk(&mut self, output: &Output, t: usize) -> Result<bool, &str> {
        if output.action.len() < t+1 {
            return Err("output ga mijikai");
        }
        let dir = output.action[t];
        let dy: i32 = if dir == 'L' {
            -1
        } else if dir == 'S' {
            0
        } else {
            1
        };
        let ny = if self.y == 0 && dy == -1 {
            W-1
        } else {
            ((self.y as i32 + dy) % W as i32) as usize
        };
        if !self.grid[0][ny].is_empty {
            return Err("jibunn de butukatta");
        }
        self.y = ny;
        Ok(true)
    }

    pub fn attack(&mut self) {
        for i in 0..H {
            if !self.grid[i][self.y].is_empty {
                if self.grid[i][self.y].hp <= self.level {
                    self.total_exp += self.grid[i][self.y].exp;
                    self.score += self.grid[i][self.y].fhp;
                    self.level = 1 + self.total_exp / 100;
                    self.grid[i][self.y] = Enemy::empty();
                } else {
                    self.grid[i][self.y].hp -= self.level;
                }
                return;
            }
        }
    }
}

#[wasm_bindgen]
pub struct Res {
    pub score: i32,
    pub level: i32,
    pub total_exp: i32,
    #[wasm_bindgen(getter_with_clone)]
    pub error: String,
    #[wasm_bindgen(getter_with_clone)]
    pub svg: String
}

#[wasm_bindgen]
pub fn vis(input: &str, output: &str, t: i32) -> Res {
    let input = parse_input(input);
    let output = parse_output(output);
    let t = min(t as usize+1, output.action.len());

    let mut state = State::new();
    let mut err = "".to_string();
    for t in 0..t {
        match state.advance() {
            Ok(_) => {},
            Err(errmsg) => {
                err = errmsg.to_string();
                break;
            },
        };
        state.add_enemies(&input, t);
        match state.walk(&output, t) {
            Ok(_) => {},
            Err(errmsg) => {
                err = errmsg.to_string();
                break;
            },
        }
        state.attack();
    }
    let max_px = 650usize;
    let scale = max_px as f64 / H as f64;
    let mut doc = svg::Document::new()
        .set("viewBox", (-5, -5, max_px + 10, max_px + 10))
        .set("width", max_px + 10)
        .set("height", max_px + 10);
    doc = doc.add(
        Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("height", H as f64 * scale)
            .set("width", W as f64 * scale)
			.set("fill", "#eee")
			.set("stroke", "black")
			.set("stroke-width", 1)
    );
    let mut cnt_enemy = 0;
    for i in 0..H {
        for j in 0..W {
            if state.grid[i][j].is_empty {
                doc = doc.add(
                    Rectangle::new()
                        .set("y", i as f64 * scale)
                        .set("x", j as f64 * scale)
                        .set("height", scale)
                        .set("width", scale)
                        .set("fill", "#632")
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                );
            } else {
                doc = doc.add(
                    Rectangle::new()
                        .set("y", i as f64 * scale)
                        .set("x", j as f64 * scale)
                        .set("height", scale)
                        .set("width", scale)
                        .set("fill", "#f00")
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("id", format!("enemy{}", cnt_enemy))
                        .set("title", format!("x={} y={} hp={} maxhp={} exp={}", i, j, state.grid[i][j].hp, state.grid[i][j].fhp, state.grid[i][j].exp))
                );
                cnt_enemy += 1;
            }
        }
    }
    let mut x2 = 60;
    for i in 0..H {
        if !state.grid[i][state.y].is_empty {
            x2 = i;
            break;
        }
    }
    doc = doc.add(
        Line::new()
            .set("y1", 0.5 as f64 * scale)
            .set("x1", (state.y as f64 + 0.5) * scale)
            .set("y2", x2 as f64 * scale)
            .set("x2", (state.y as f64 + 0.5) * scale)
            .set("stroke", "yellow")
            .set("stroke-width", 5)
    );
    doc = doc.add(
        Rectangle::new()
            .set("y", 0 as f64 * scale)
            .set("x", state.y as f64 * scale)
            .set("height", scale)
            .set("width", scale)
            .set("fill", "#8e8")
            .set("stroke", "black")
            .set("stroke-width", 1)
    );

    Res { score: state.score as i32, level: state.level as i32, total_exp: state.total_exp as i32, error: err, svg: doc.to_string() }
}


#[wasm_bindgen]
pub fn get_max_turn(input: &str, output: &str) -> i32 {
    let input = parse_input(input);
    let output = parse_output(output);
    
    let mut state = State::new();
    for t in 0..T {
        match state.advance() {
            Ok(_) => {},
            Err(_errmsg) => {
                return t as i32;
            },
        };
        state.add_enemies(&input, t);
        match state.walk(&output, t) {
            Ok(_) => {},
            Err(_errmsg) => {
                return t as i32;
            },
        }
        state.attack();
    }
    T as i32
} 

#[wasm_bindgen]
pub fn edit_output(_input: &str, output: &str, t: i32, button: &str) -> String {
    let output = parse_output(&output);
    let mut new_output = "".to_string();

    let t = if button == "-" {
        t-1
    } else {
        t
    };
    for i in 0..t {
        new_output += &output.action[i as usize].to_string();
    }
    if button != "-" {
        new_output += button;
    }

    new_output
}