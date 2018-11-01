
//mod tests;
extern crate rustfft;
extern crate primal;

use rustfft::algorithm::Radix4;
use rustfft::algorithm::MixedRadix;
use rustfft::{FFT, FFTplanner};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

pub fn run_fft(time_data: Vec<Complex<f64>>) -> Vec<f64> {
    let mut inp = time_data.clone();
    // Computes a forward FFT of size time_data.len()
    let mut out: Vec<Complex<f64>> = vec![Zero::zero(); time_data.len()];

    let fft = Radix4::new(inp.len(), false);
    fft.process(&mut inp, &mut out);
    let mut output = Vec::with_capacity(out.len());
    for c in out {
        let res = c.re*c.re + c.im*c.im;
        output.push(res);
    }
    // println!("{:?}", inp);
    output
}

pub fn run_mixed_radix_fft(time_data: Vec<Complex<f64>>) -> Vec<f64> {
    let mut input = time_data.clone();
    let mut planner = FFTplanner::new(false);
    let (a,b) = find_med_multiplier(time_data.len() as u64);
    let inner_fft_n1 = planner.plan_fft(a as usize);
    let inner_fft_n2 = planner.plan_fft(b as usize);

    let mut out: Vec<Complex<f64>> = vec![Zero::zero(); time_data.len()];
    // the mixed radix FFT length will be
    // inner_fft_n1.len() * inner_fft_n2.len()
    let fft = MixedRadix::new(inner_fft_n1, inner_fft_n2);
    fft.process(&mut input, &mut out);
    let mut output = Vec::with_capacity(out.len());
    for c in out {
        let res = c.re*c.re + c.im*c.im;
        output.push(res);
    }
    output
}

pub fn find_largest_prime(num : u64) -> u64 {
    let mut p = num;
    while !primal::is_prime(p) {
        p -= 1;
    }
    p
}

pub fn find_prime_factors(num : u64) -> Vec<u64> {
    let mut i: u64 = 2;
    let mut res = Vec::new();
    let mut num = num;
    while i <= num {
        if num % i == 0 {
            num = num / i;
            res.push(i);
        } else {
            i += 1;
        }
    }
    res
}

// Returns integers (y, k) such that num = y*k
// with y being reasonably close to sqrt(x)
pub fn find_med_multiplier(num : u64) -> (u64, u64) {
    let factors = find_prime_factors(num);
    let sqrt = (num as f64).sqrt();
    let mut z = 1;
    let mut pz = 1;
    for x in factors {
        z *= x;
        if sqrt < z as f64 {
            pz = z/x;
            break;
        }
    }
    let smaller = sqrt - pz as f64;
    let larger = z as f64 - sqrt;
    let mut res = (z,num/z);
    if smaller < larger { res = (pz,num/pz) }
    res
}

const NUM_BANDS : usize = 12;

#[macro_use]
macro_rules! freq2oct {
    ($freq:expr) => {
        freq_to_octave($freq, 440f64/16f64)
    };
}

pub fn freq_to_octave(freq: f64, base: f64) -> f64 {
    (freq / base).ln() / 2f64.ln()
}

pub fn freq_to_index(freq: f64, frame_size: usize, sample_rate: i64) -> usize {
    (frame_size as f64 * freq / sample_rate as f64).round() as usize
}

pub fn index_to_freq(idx: usize, frame_size: usize, sample_rate: i64) -> f64 {
    (idx as i64 * sample_rate) as f64 / frame_size as f64
}

pub fn index_to_octave(idx: usize, frame_size: usize, sample_rate: i64) -> f64 {
    freq2oct!(index_to_freq(idx, frame_size, sample_rate))
}

pub fn chroma(x: i32, y :i32, z: i32, a: i32) -> i32 {
    x+y
}

#[derive(Debug, Clone)]
pub struct Note {
    note: i64,
    note_fract: f64
}

pub fn prepare_notes(min_freq: f64, max_freq: f64, frame_size: usize, sample_rate: i64)
-> std::vec::Vec<Note> {
    let start = freq_to_index(min_freq, frame_size, sample_rate).max(1);
    let min_plus_size = freq_to_index(max_freq, frame_size, sample_rate).min(frame_size / 2);

    let size = min_plus_size - start;
    let mut notes = vec![Note{note: 0, note_fract: 0.0}; size];

    for idx in 0..size {
        let octave = index_to_octave(idx + start, frame_size, sample_rate);
        let note = NUM_BANDS as f64 * octave.fract();
        let nt = Note{note: note.floor() as i64, note_fract: note.fract()};
        notes[idx] = nt;
    }
    notes
}

pub fn prepare_features(notes: std::vec::Vec<Note>, frame: std::vec::Vec<f64>) -> std::vec::Vec<f64> {
    let mut features = vec![0f64; NUM_BANDS];
    for item in notes.iter() {
        match item {
            Note { note, note_fract } => {

            }
        }
    }
    features
}