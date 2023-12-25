use std::io::{self, Write};
use std::{f32::consts::PI, thread, time};

const A_INIT: f32 = 0.0; // 回転のための変数
const B_INIT: f32 = 0.0;
const C_INIT: f32 = 0.0;

const CUBE_WIDTH: f32 = 10.0; // キューブの幅
const WIDTH: usize = 160; // コンソールの幅
const HEIGHT: usize = 44; // コンソールの高さ
const DISTANCE_FROM_CAM: f32 = 60.0; // カメラからの距離
const K1: f32 = 40.0; // 視点のズーム

const INCREMENT_SPEED: f32 = 0.6; // キューブの回転速度

// 8つの頂点を持つキューブの定義
const VERTICES: [[f32; 3]; 8] = [
    [-CUBE_WIDTH, -CUBE_WIDTH, -CUBE_WIDTH],
    [CUBE_WIDTH, -CUBE_WIDTH, -CUBE_WIDTH],
    [CUBE_WIDTH, CUBE_WIDTH, -CUBE_WIDTH],
    [-CUBE_WIDTH, CUBE_WIDTH, -CUBE_WIDTH],
    [-CUBE_WIDTH, -CUBE_WIDTH, CUBE_WIDTH],
    [CUBE_WIDTH, -CUBE_WIDTH, CUBE_WIDTH],
    [CUBE_WIDTH, CUBE_WIDTH, CUBE_WIDTH],
    [-CUBE_WIDTH, CUBE_WIDTH, CUBE_WIDTH],
];

fn calculate_x(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.sin() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos()
        + j * a.cos() * c.sin()
        + k * a.sin() * c.sin()
        + i * b.cos() * c.cos()
}

fn calculate_y(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.cos() * c.cos() + k * a.sin() * c.cos() - j * a.sin() * b.sin() * c.sin()
        + k * a.cos() * b.sin() * c.sin()
        - i * b.cos() * c.sin()
}

fn calculate_z(i: f32, j: f32, k: f32, a: f32, b: f32) -> f32 {
    k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
}

fn calculate_for_surface(
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    a: f32,
    b: f32,
    c: f32,
    z_buffer: &mut [f32],
    buffer: &mut [char],
) {
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b) + DISTANCE_FROM_CAM;

    let ooz = 1.0 / z;

    let xp = (WIDTH as f32 / 2.0 + K1 * ooz * x * 2.0) as isize;
    let yp = (HEIGHT as f32 / 2.0 + K1 * ooz * y) as isize;

    let idx = xp + yp * WIDTH as isize;
    if idx >= 0 && (idx as usize) < WIDTH * HEIGHT {
        if ooz > z_buffer[idx as usize] {
            z_buffer[idx as usize] = ooz;
            buffer[idx as usize] = '0';
        }
    }
}

fn draw_line(
    x0: f32,
    y0: f32,
    z0: f32,
    x1: f32,
    y1: f32,
    z1: f32,
    a: f32,
    b: f32,
    c: f32,
    z_buffer: &mut [f32],
    buffer: &mut [char],
) {
    let steps = ((x1 - x0).abs()).max((y1 - y0).abs()).max((z1 - z0).abs()) as usize;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        calculate_for_surface(
            x0 + (x1 - x0) * t,
            y0 + (y1 - y0) * t,
            z0 + (z1 - z0) * t,
            a,
            b,
            c,
            z_buffer,
            buffer,
        );
    }
}

fn main() {
    let mut a = A_INIT;
    let mut b = B_INIT;
    let mut c = C_INIT;

    let mut z_buffer: Vec<f32> = vec![0.0; WIDTH * HEIGHT];
    let mut buffer: Vec<char> = vec![' '; WIDTH * HEIGHT];

    loop {
        for item in buffer.iter_mut() {
            *item = ' ';
        }
        for z in z_buffer.iter_mut() {
            *z = 0.0;
        }

        // キューブの各辺を描画するためのインデックス
        let edges: [[usize; 2]; 12] = [
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 0],
            [4, 5],
            [5, 6],
            [6, 7],
            [7, 4],
            [0, 4],
            [1, 5],
            [2, 6],
            [3, 7],
        ];

        // キューブの各辺を描画
        for edge in edges.iter() {
            let v0 = VERTICES[edge[0]];
            let v1 = VERTICES[edge[1]];
            draw_line(
                v0[0],
                v0[1],
                v0[2],
                v1[0],
                v1[1],
                v1[2],
                a,
                b,
                c,
                &mut z_buffer,
                &mut buffer,
            );
        }

        // バッファを出力
        print!("\x1b[H");
        for k in 0..(WIDTH * HEIGHT) {
            print!("{}", buffer[k]);
            if k % WIDTH == WIDTH - 1 {
                print!("\n");
            }
        }
        io::stdout().flush().unwrap();

        // キューブを回転
        a += INCREMENT_SPEED * PI / 180.0;
        b += INCREMENT_SPEED * PI / 180.0;
        c += INCREMENT_SPEED * PI / 180.0;

        // 一時停止
        thread::sleep(time::Duration::from_millis(50));
    }
}
