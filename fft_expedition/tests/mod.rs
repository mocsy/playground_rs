#[cfg(test)]
extern crate semitone;
extern crate rustfft;
extern crate rand;
extern crate primal;

mod tests {
    use rustfft::num_complex::Complex;
    use rand::{StdRng, SeedableRng};
    use rand::distributions::{Normal, Distribution};


    #[test]
    fn normal_a() {
        use semitone::chroma;
        let chroma = chroma(10, 510, 256, 1000);
        let mut frame = vec![0.0; 128];
        frame[113] = 1.0;

        //assert_eq!(add(2,3), 5);
    }

    #[test]
    fn test_ln() {
        let five_dot_five : f64 = 5.5;
        assert_eq!(five_dot_five.ln(), 1.7047480922384253);
    }

    #[test]
    fn test_freq_to_index() {
        use semitone::freq_to_index;
        let freq: f64 = 510f64;
        let frame_size: usize = 256;
        let sample_rate: i64 = 1000;
        let idx = freq_to_index(freq, frame_size, sample_rate);
        assert!(idx < frame_size);

        for i in 2..10000000 {
            let frame_size = i;
            let idx = freq_to_index(freq, frame_size, sample_rate);
            assert!(idx < frame_size);
        }

        // freq can't be higher than sample_rate
        let freq: f64 = 1001f64;
        let idx = freq_to_index(freq, frame_size, sample_rate);
        assert_eq!(idx, frame_size);
    }

    #[test]
    fn test_fft_rand() {
        use semitone::run_fft;
        use rustfft::num_complex::Complex;
        use std;

        let c = Complex::new((std::i16::MAX/2) as f64, 0f64);
        let input: Vec<Complex<f64>> = vec![c; 32];
        let input_rand = random_signal(32);
        // println!("{:?}", input_rand);
        let output = run_fft(input_rand);
        // println!("{:?}", output);
    }

    fn random_signal(length: usize) -> Vec<Complex<f64>> {
        use rustfft::num_complex::Complex;
        use rand;

        let mut sig = Vec::with_capacity(length);
        let normal_dist = Normal::new(0.0, 10.0);
        for _ in 0..length {
            sig.push(Complex{re: (normal_dist.sample(&mut rand::thread_rng())),
                            im: (normal_dist.sample(&mut rand::thread_rng()))});
        }
        return sig;
    }

    #[test]
    fn test_fft_sin() {
        use semitone::run_fft;
        use std;

        let input = sine_signal(16, std::i16::MAX as usize, 218.0, 1000.0);
        // println!("{:?}", input);
        let output = run_fft(input);
        // println!("{:?}", output);

        let mut rx4 = Vec::new();
        for i in 0..output.len() {
            let magnitude = output[i].sqrt() / output.len() as f64;
            rx4.push(magnitude);
            // println!("{:?}", magnitude);
        }

        use semitone::run_mixed_radix_fft;

        let input = sine_signal(16, std::i16::MAX as usize, 218.0, 1000.0);
        // println!("{:?}", input);
        let output = run_mixed_radix_fft(input);
        // println!("{:?}", output);
        
        let mut mx4 = Vec::new();
        for i in 0..output.len() {
            let magnitude = output[i].sqrt() / output.len() as f64;
            mx4.push(magnitude);
            // println!("{:?}", magnitude);
        }
        assert!(vecdiff(&rx4, &mx4) < 0.00000001f64);
    }

    fn sine_signal(length: usize, amp: usize, freq: f64, sample_rate: f64) -> Vec<Complex<f64>> {
        use std;
        let sample = 2f64 * std::f64::consts::PI / sample_rate;
        let len = length << 1;
        let mut real_signal = Vec::with_capacity(len);
        for x in 0..len {
            real_signal.push(amp as f64 * (x as f64 * freq * sample).sin());
        }
        //println!("{:?}", real_signal);
        let mut signal = Vec::with_capacity(length);
        for x in 0..length {
            signal.push(Complex{re: real_signal[x << 1],
                                im: real_signal[(x << 1) + 1]});
        }
        signal
    }

    #[test]
    fn test_find_largest_prime() {
        use semitone::find_largest_prime;
        let p = find_largest_prime(1200f64.sqrt() as u64);
        assert_eq!(p, 31);
    }

    #[test]
    fn test_find_prime_factors() {
        use semitone::find_prime_factors;
        let res = find_prime_factors(1200);
        assert_eq!(res, [2, 2, 2, 2, 3, 5, 5]);
    }

    #[test]
    fn test_find_med_multiplier() {
        use semitone::find_med_multiplier;
        let res = find_med_multiplier(1200);
        assert_eq!(res, (48,25));
    }

    fn vecdiff(vec1: &[f64], vec2: &[f64]) -> f64 {
        assert_eq!(vec1.len(), vec2.len());
        let mut diff = 0f64;
        for (&a, &b) in vec1.iter().zip(vec2.iter()) {
            diff += a - b;
        }
        diff/vec1.len() as f64
    }
}
