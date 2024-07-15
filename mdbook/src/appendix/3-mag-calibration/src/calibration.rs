//! Translated from <https://github.com/lancaster-university/codal-microbit-v2/blob/006abf5566774fbcf674c0c7df27e8a9d20013de/source/MicroBitCompassCalibrator.cpp>

use core::fmt::Debug;
use embedded_hal::i2c::I2c;
use embedded_hal::delay::DelayNs;
use libm::{fabsf, sqrtf};

use lsm303agr::{interface::I2cInterface, mode::MagContinuous, Lsm303agr};

use microbit::display::blocking::Display;

const PERIMETER_POINTS: usize = 25;
const PIXEL1_THRESHOLD: i32 = 200;
const PIXEL2_THRESHOLD: i32 = 600;
const CALIBRATION_INCREMENT: i32 = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Measurement {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Measurement {
    pub fn new((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
pub struct Calibration {
    center: Measurement,
    scale: Measurement,
    _radius: u32,
}

impl Default for Calibration {
    fn default() -> Calibration {
        Calibration {
            center: Measurement { x: 0, y: 0, z: 0 },
            scale: Measurement {
                x: 1024,
                y: 1024,
                z: 1024,
            },
            _radius: 0,
        }
    }
}

pub fn calc_calibration<I, T>(
    sensor: &mut Lsm303agr<I2cInterface<I>, MagContinuous>,
    display: &mut Display,
    timer: &mut T,
) -> Calibration
where
    T: DelayNs,
    I: I2c,
{
    let data = get_data(sensor, display, timer);
    calibrate(&data)
}

fn get_data<I, T>(
    sensor: &mut Lsm303agr<I2cInterface<I>, MagContinuous>,
    display: &mut Display,
    timer: &mut T,
) -> [Measurement; PERIMETER_POINTS]
where
    T: DelayNs,
    I: I2c,
{
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut cursor = (2, 2);
    let mut data = [Measurement { x: 0, y: 0, z: 0 }; PERIMETER_POINTS];
    let mut samples = 0;
    let mut blink_state = true;

    while samples < PERIMETER_POINTS {
        while !sensor.accel_status().unwrap().xyz_new_data() {
            timer.delay_us(10u32);
        }
        let (x, y, _) = sensor.acceleration().unwrap().xyz_mg();

        if x < -PIXEL2_THRESHOLD {
            cursor.1 = 4;
        } else if x < -PIXEL1_THRESHOLD {
            cursor.1 = 3;
        } else if x > PIXEL2_THRESHOLD {
            cursor.1 = 0;
        } else if x > PIXEL1_THRESHOLD {
            cursor.1 = 1;
        } else {
            cursor.1 = 2;
        }

        if y < -PIXEL2_THRESHOLD {
            cursor.0 = 0;
        } else if y < -PIXEL1_THRESHOLD {
            cursor.0 = 1;
        } else if y > PIXEL2_THRESHOLD {
            cursor.0 = 4;
        } else if y > PIXEL1_THRESHOLD {
            cursor.0 = 3;
        } else {
            cursor.0 = 2;
        }

        // Turn the y axis properly
        cursor.0 = 4 - cursor.0;

        if leds[cursor.0][cursor.1] != 1 {
            leds[cursor.0][cursor.1] = 1;
            while !sensor.mag_status().unwrap().xyz_new_data() {
                timer.delay_us(10u32);
            }
            let measurement = Measurement::new(
                sensor.magnetic_field().unwrap().xyz_nt()
            );
            data[samples] = measurement;
            samples += 1;
        }
        let save_cursor = leds[cursor.0][cursor.1];
        if blink_state {
            leds[cursor.0][cursor.1] = 1 - leds[cursor.0][cursor.1];
        }
        display.show(timer, leds, 200);
        blink_state = !blink_state;
        leds[cursor.0][cursor.1] = save_cursor;
    }
    data
}

fn difference_square(a: Measurement, b: Measurement) -> f32 {
    let dx = (a.x - b.x) as f32;
    let dy = (a.y - b.y) as f32;
    let dz = (a.z - b.z) as f32;

    (dx * dx) + (dy * dy) + (dz * dz)
}

fn measure_score(center: Measurement, data: &[Measurement]) -> f32 {
    let mut min_d = difference_square(center, data[0]);
    let mut max_d = min_d;

    for point in data[1..].iter() {
        let d = difference_square(center, *point);
        if d < min_d {
            min_d = d;
        }

        if d > max_d {
            max_d = d;
        }
    }

    max_d - min_d
}

fn calibrate(data: &[Measurement]) -> Calibration {
    // Approximate a center for the data
    let mut center = Measurement { x: 0, y: 0, z: 0 };
    let mut best = center;

    for point in data {
        center.x += point.x;
        center.y += point.y;
        center.z += point.z;
    }

    let ndata = data.len() as i32;
    center.x /= ndata;
    center.y /= ndata;
    center.z /= ndata;

    let mut current = center;
    let mut score = measure_score(current, data);

    // Calculate a fixpoint position
    loop {
        for x in [-CALIBRATION_INCREMENT, 0, CALIBRATION_INCREMENT] {
            for y in [-CALIBRATION_INCREMENT, 0, CALIBRATION_INCREMENT] {
                for z in [-CALIBRATION_INCREMENT, 0, CALIBRATION_INCREMENT] {
                    let mut attempt = current;
                    attempt.x += x;
                    attempt.y += y;
                    attempt.z += z;

                    let attempt_score = measure_score(attempt, data);
                    if attempt_score < score {
                        score = attempt_score;
                        best = attempt;
                    }
                }
            }
        }

        if best == current {
            break;
        }

        current = best;
    }

    spherify(current, data)
}

fn spherify(center: Measurement, data: &[Measurement]) -> Calibration {
    let mut _radius = 0;
    for point in data {
        let d = sqrtf(difference_square(center, *point)) as u32;
        if d > _radius {
            _radius = d;
        }
    }

    let mut scale: f32 = 0.0;
    let mut weight_x = 0.0;
    let mut weight_y = 0.0;
    let mut weight_z = 0.0;

    for point in data {
        let d = sqrtf(difference_square(center, *point));
        let s = (_radius as f32 / d) - 1.0;
        scale = scale.max(s);

        let dx = point.x - center.x;
        let dy = point.y - center.y;
        let dz = point.z - center.z;

        weight_x += s * fabsf(dx as f32 / d);
        weight_y += s * fabsf(dy as f32 / d);
        weight_z += s * fabsf(dz as f32 / d);
    }

    let wmag = sqrtf((weight_x * weight_x) + (weight_y * weight_y) + (weight_z * weight_z));
    let scale_x = 1.0 + scale * (weight_x / wmag);
    let scale_y = 1.0 + scale * (weight_y / wmag);
    let scale_z = 1.0 + scale * (weight_z / wmag);

    Calibration {
        center,
        _radius,
        scale: Measurement {
            x: (1024.0 * scale_x) as i32,
            y: (1024.0 * scale_y) as i32,
            z: (1024.0 * scale_z) as i32,
        },
    }
}

pub fn calibrated_measurement(measurement: Measurement, calibration: &Calibration) -> Measurement {
    Measurement {
        x: ((measurement.x - calibration.center.x) * calibration.scale.x) >> 10,
        y: ((measurement.y - calibration.center.y) * calibration.scale.y) >> 10,
        z: ((measurement.z - calibration.center.z) * calibration.scale.z) >> 10,
    }
}
