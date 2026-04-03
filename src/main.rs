use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn rotate(mul: i32, shift: i32, mut x: i32, mut y: i32) -> (i32, i32) {
    let val = x;
    x -= (mul * y) >> shift;
    y += (mul * val) >> shift;
    let val2 = (3145728 - x * x - y * y) >> 11;
    x = (x * val2) >> 10;
    y = (y * val2) >> 10;
    (x, y)
}

fn main() {
    let mut s_a: i32 = 1024;
    let mut c_a: i32 = 0;
    let mut s_b: i32 = 1024;
    let mut c_b: i32 = 0;

    let mut b = [0u8; 1760];
    let mut z = [0i8; 1760];

    loop {
        b.fill(b' ');
        z.fill(127);

        let mut s_j: i32 = 0;
        let mut c_j: i32 = 1024;

        for _ in 0..90 {
            let mut s_i: i32 = 0;
            let mut c_i: i32 = 1024;

            for _ in 0..324 {
                let r1 = 1;
                let r2 = 2048;
                let k2 = 5120 * 1024;

                let x0 = r1 * c_j + r2;
                let x1 = (c_i * x0) >> 10;
                let x2 = (c_a * s_j) >> 10;
                let x3 = (s_i * x0) >> 10;
                let x4 = r1 * x2 - ((s_a * x3) >> 10);
                let x5 = (s_a * s_j) >> 10;
                let x6 = k2 + r1 * 1024 * x5 + c_a * x3;
                let x7 = (c_j * s_i) >> 10;

                let x = 40 + 30 * (c_b * x1 - s_b * x4) / x6;
                let y = 12 + 15 * (c_b * x4 + s_b * x1) / x6;

                let n = (((-c_a * x7 - c_b * ((-s_a * x7 >> 10) + x2) - c_i * ((c_j * s_b) >> 10)) >> 10) - x5) >> 7;

                let o = (x + 80 * y) as usize;
                let zz = ((x6 - k2) >> 15) as i8;

                if 22 > y && y > 0 && x > 0 && 80 > x && zz < z[o] {
                    z[o] = zz;
                    // Clamp applied to prevent Rust panic on out-of-bounds read
                    let n_idx = (if n > 0 { n as usize } else { 0 }).min(11);
                    b[o] = b".,-~:;=!*#$@"[n_idx];
                }

                (c_i, s_i) = rotate(5, 8, c_i, s_i);
            }
            (c_j, s_j) = rotate(9, 7, c_j, s_j);
        }

        let mut stdout = io::stdout();
        for k in 0..=1760 {
            let char_out = if k % 80 != 0 { b[k] } else { b'\n' };
            let _ = stdout.write_all(&[char_out]);
        }
        let _ = stdout.flush();

        (c_a, s_a) = rotate(5, 7, c_a, s_a);
        (c_b, s_b) = rotate(5, 8, c_b, s_b);

        thread::sleep(Duration::from_millis(15));
        let _ = stdout.write_all(b"\x1b[23A");
    }
}