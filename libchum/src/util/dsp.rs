use crate::util;

pub const SAMPLES_PER_FRAME: usize = 14;
pub const BYTES_PER_FRAME: usize = 8;
pub const NIBBLES_PER_FRAME: usize = 16;
pub const HEADERS_PER_FRAME: usize = NIBBLES_PER_FRAME - SAMPLES_PER_FRAME;

fn f64_to_i16_clamp(value: f64) -> i16 {
    if value > i16::MAX as f64 {
        i16::MAX
    } else if value < i16::MIN as f64 {
        i16::MIN
    } else {
        value.round() as i16
    }
}

fn clamp_i32(value: i32) -> i32 {
    if value > i16::MAX as i32 {
        i16::MAX as i32
    } else if value < i16::MIN as i32 {
        i16::MIN as i32
    } else {
        value
    }
}

fn i32_to_nibble(value: i32) -> u8 {
    if value > 7 {
        7
    } else if value < -8 {
        8
    } else if value >= 0 {
        value as u8
    } else {
        (16 + value) as u8
    }
}

// Adapted from https://github.com/Thealexbarney/DspTool
pub fn decode(coef: &[i16; 16], data: &[u8], num_samples: usize) -> Vec<i16> {
    let mut hist1 = 0i16;
    let mut hist2 = 0i16;
    let frame_count = data.len() / BYTES_PER_FRAME;
    let mut out = Vec::with_capacity(num_samples);
    for i_frame in 0..frame_count {
        let index = i_frame * BYTES_PER_FRAME;
        let frame = &data[index..index + BYTES_PER_FRAME];
        let (h_high, h_low) = util::get_nibbles(frame[0]);
        let predictor: usize = h_high as usize;
        let scale: i32 = 1i32 << h_low as i32;
        let coef1 = coef[(predictor * 2) % coef.len()];
        let coef2 = coef[(predictor * 2 + 1) % coef.len()];
        let samples_to_read = SAMPLES_PER_FRAME.min(num_samples - out.len());
        for i_sample in 0..samples_to_read {
            let sample = if i_sample % 2 == 0 {
                util::get_high_nibble(frame[1 + i_sample / 2])
            } else {
                util::get_low_nibble(frame[1 + i_sample / 2])
            } as i32;
            let sample = if sample >= 8 { sample - 16 } else { sample };
            let sample = (((scale * sample) << 11)
                + 1024
                + (coef1 as i32 * hist1 as i32 + coef2 as i32 * hist2 as i32))
                >> 11;
            let real_sample = if sample > i16::MAX as i32 {
                i16::MAX
            } else if sample < i16::MIN as i32 {
                i16::MIN
            } else {
                sample as i16
            };
            hist2 = hist1;
            hist1 = real_sample;
            out.push(real_sample);
        }
    }
    out
}

// no idea what's going on with anything below, just kinda copying from the above source

// history buffer MUST be 28 bytes
fn inner_product_merge(pcm_buf: &[i16]) -> [f64; 3] {
    let mut tvec = [0.0; 3];
    for i in 0..=2 {
        for x in 0..14 {
            tvec[i] -= pcm_buf[14 + x - i] as f64 * pcm_buf[14 + x] as f64;
        }
    }
    tvec
}

// history buffer MUST be 28 bytes
fn outer_product_merge(pcm_buf: &[i16]) -> [[f64; 3]; 3] {
    let mut out = [[0.0; 3]; 3];
    for x in 1..=2 {
        for y in 1..=2 {
            for z in 0..14 {
                out[x][y] += pcm_buf[14 + z - x] as f64 * pcm_buf[14 + z - y] as f64;
            }
        }
    }
    out
}

struct AnalyzeRangeResult {
    vec_idxs: [usize; 3],
    // recips: [f64; 3],
    mtx: [[f64; 3]; 3],
}

fn analyze_ranges(mut mtx: [[f64; 3]; 3]) -> Option<AnalyzeRangeResult> {
    let mut recips = [0.0; 3];
    let mut vec_idxs = [0; 3];
    for x in 1..=2 {
        let val = f64::max(mtx[x][1].abs(), mtx[x][2]);
        if val < f64::EPSILON {
            return None;
        }
        recips[x] = 1.0 / val;
    }
    let mut max_index = 0;
    for i in 1..=2 {
        for x in 1..i {
            let mut tmp = mtx[x][i];
            for y in 1..x {
                tmp -= mtx[x][y] * mtx[y][i];
            }
            mtx[x][i] = tmp;
        }
        let mut val = 0.0;
        for x in i..=2 {
            let mut tmp = mtx[x][i];
            for y in 1..i {
                tmp -= mtx[x][y] * mtx[y][i];
            }
            mtx[x][i] = tmp;
            tmp = tmp.abs() * recips[x];
            if tmp >= val {
                val = tmp;
                max_index = x;
            }
        }
        if max_index != i {
            for y in 1..=2 {
                let tmp = mtx[max_index][y];
                mtx[max_index][y] = mtx[i][y];
                mtx[i][y] = tmp;
            }
            recips[max_index] = recips[i];
        }
        vec_idxs[i] = max_index;
        if mtx[i][i] == 0.0 {
            return None;
        }
        if i != 2 {
            let tmp = 1.0 / mtx[i][i];
            for x in (i + 1)..=2 {
                mtx[x][i] *= tmp;
            }
        }
    }
    let mut min = 1e10f64;
    let mut max = 0.0f64;
    for i in 1..=2 {
        let tmp = mtx[i][i].abs();
        min = min.min(tmp);
        max = max.max(tmp);
    }
    if min / max < 1.0e-10 {
        None
    } else {
        Some(AnalyzeRangeResult {
            mtx,
            vec_idxs,
            //recips
        })
    }
}

fn bidirectional_filter(mtx: [[f64; 3]; 3], vec_idxs: [usize; 3], vec_in: [f64; 3]) -> [f64; 3] {
    let mut out = vec_in;
    let mut x = 0;
    for i in 1..=2 {
        let index = vec_idxs[i];
        let mut tmp = out[index];
        out[index] = out[i];
        if x != 0 {
            for y in x..=(i - 1) {
                tmp -= out[i] * mtx[i][y];
            }
        } else if tmp != 0.0 {
            x = i;
        }
        out[i] = tmp;
    }
    for i in (1..=2).rev() {
        let mut tmp = out[i];
        for y in (i + 1)..=2 {
            tmp -= out[y] * mtx[i][y];
        }
        out[i] = tmp / mtx[i][i];
    }
    out[0] = 1.0;
    out
}

fn quadratic_merge(mut v: [f64; 3]) -> Option<[f64; 3]> {
    let v2 = v[2];
    let tmp = 1.0 - (v2 * v2);
    if tmp == 0.0 {
        return None;
    }
    let v0 = (v[0] - (v2 * v2)) / tmp;
    let v1 = (v[1] - (v[1] * v2)) / tmp;
    v[0] = v0;
    v[1] = v1;
    if v1.abs() > 1.0 {
        None
    } else {
        Some(v)
    }
}

fn finish_record(mut v: [f64; 3]) -> [f64; 3] {
    for z in 1..=2 {
        if v[z] >= 1.0 {
            v[z] = 0.9999999999;
        } else if v[z] <= -1.0 {
            v[z] = -0.9999999999;
        }
    }
    [1.0, (v[2] * v[1]) + v[1], v[2]]
}

fn matrix_filter(src: [f64; 3]) -> [f64; 3] {
    let mut mtx = [[0.0; 3]; 3];
    mtx[2][0] = 1.0;
    for i in 1..=2 {
        mtx[2][i] = -src[i];
    }
    for i in (1..=2).rev() {
        let val = 1.0 - (mtx[i][i] * mtx[i][i]);
        for y in 1..=i {
            mtx[i - 1][y] = ((mtx[i][i] * mtx[i][y]) + mtx[i][y]) / val;
        }
    }
    let mut dst = [0.0; 3];
    for i in 1..=2 {
        for y in 1..=i {
            dst[i] += mtx[i][y] * dst[i - y];
        }
    }
    dst
}

fn merge_finish_record(src: [f64; 3]) -> [f64; 3] {
    let mut val = src[0];
    let mut dst = [1.0, 0.0, 0.0];
    let mut tmp = [0.0; 3];
    for i in 1..=2 {
        let mut v2 = 0.0;
        for y in 1..i {
            v2 += dst[y] * src[i - y];
        }
        if val > 0.0 {
            dst[i] = -(v2 + src[i]) / val;
        } else {
            dst[i] = 0.0;
        }
        tmp[i] = dst[i];
        for y in 1..i {
            dst[y] += dst[i] * dst[i - y];
        }
        val *= 1.0 - (dst[i] * dst[i]);
    }
    finish_record(tmp)
}

fn contrast_vectors(a: [f64; 3], b: [f64; 3]) -> f64 {
    let v0 = (b[2] * b[1] - b[1]) / (1.0 - b[2] * b[2]);
    let v1 = (a[0] * a[0]) + (a[1] * a[1]) + (a[2] * a[2]);
    let v2 = (a[0] * a[1]) + (a[1] * a[2]);
    let v3 = a[0] * a[2];
    v1 + (2.0 * v0 * v2) + (2.0 * (-b[1] * v0 - b[2]) * v3)
}

fn filter_records(mut vec_best: [[f64; 3]; 8], exp: usize, records: &[[f64; 3]]) -> [[f64; 3]; 8] {
    // let mut buffer2 = [0.0; 3];
    let mut buffer1 = [0; 8];
    let mut buffer_list = [[0.0; 3]; 8];
    for _x in 0..2 {
        for y in 0..exp {
            buffer1[y] = 0;
            for i in 0..=2 {
                buffer_list[y][i] = 0.0;
            }
        }
        for z in 0..records.len() {
            let mut index = 0;
            let mut value = 1.0e30;
            for i in 0..exp {
                let tmp = contrast_vectors(vec_best[i], records[z]);
                if tmp < value {
                    value = tmp;
                    index = i;
                }
            }
            buffer1[index] += 1;
            let buffer2 = matrix_filter(records[z]);
            for i in 0..=2 {
                buffer_list[index][i] += buffer2[i];
            }
        }
        for i in 0..exp {
            if buffer1[i] > 0 {
                for y in 0..=2 {
                    buffer_list[i][y] /= buffer1[i] as f64;
                }
            }
        }
        for i in 0..exp {
            vec_best[i] = merge_finish_record(buffer_list[i]);
        }
    }
    vec_best
}

fn calculate_coefficients(source: &[i16]) -> [i16; 16] {
    let frame_count = util::div_up(source.len(), SAMPLES_PER_FRAME);
    let mut pcm_hist_buf = [0i16; 28];
    let mut records = Vec::with_capacity(frame_count); //vec![[0.0; 3]; frame_count];
    let mut vec_best = [[0.0; 3]; 8];
    for i_frame in 0..frame_count {
        let i_sample = i_frame * SAMPLES_PER_FRAME;
        let remaining = source.len() - i_sample;
        let n = remaining.min(14);
        for i in 0..n {
            pcm_hist_buf[14 + i] = source[i_sample + i];
        }
        for i in n..14 {
            pcm_hist_buf[14 + i] = 0;
        }
        let inner_merge = inner_product_merge(&pcm_hist_buf[..]);
        if inner_merge[0].abs() > 10.0 {
            let outer_merge = outer_product_merge(&pcm_hist_buf[..]);
            if let Some(analysis) = analyze_ranges(outer_merge) {
                let filter_result =
                    bidirectional_filter(analysis.mtx, analysis.vec_idxs, inner_merge);
                if let Some(value) = quadratic_merge(filter_result) {
                    records.push(finish_record(value));
                }
            }
        }
        for i in 0..14 {
            pcm_hist_buf[i] = pcm_hist_buf[i + 14];
        }
    }
    let mut vec1 = [1.0, 0.0, 0.0];
    for z in 0..records.len() {
        vec_best[0] = matrix_filter(records[z]);
        for y in 1..=2 {
            vec1[y] += vec_best[0][y];
        }
    }
    for y in 1..=2 {
        vec1[y] /= records.len() as f64;
    }
    vec_best[0] = merge_finish_record(vec1);
    for w in 0..3 {
        let exp = 1 << w;
        let vec2 = [0.0, -1.0, 0.0];
        for i in 0..exp {
            for y in 0..=2 {
                vec_best[exp + i][y] = (0.01 * vec2[y]) + vec_best[i][y];
            }
        }
        let exp = 1 << (1 + w);
        vec_best = filter_records(vec_best, exp, &records.as_slice());
    }
    let mut coef = [0i16; 16];
    for z in 0..8 {
        coef[z * 2] = f64_to_i16_clamp(-vec_best[z][1] * 2048.0);
        coef[z * 2 + 1] = f64_to_i16_clamp(-vec_best[z][2] * 2048.0);
    }
    coef
}

pub struct EncodeResult {
    pub coef: [i16; 16],
    pub data: Vec<u8>,
}

pub fn encode(data: &[i16]) -> EncodeResult {
    let coef = calculate_coefficients(data);
    let frame_count = util::div_up(data.len(), SAMPLES_PER_FRAME);
    let mut out = Vec::new();
    let mut pcm_buffer = [0i16; 2 + SAMPLES_PER_FRAME];
    // let mut adpcm_buffer = [0u8; BYTES_PER_FRAME];
    for i_frame in 0..frame_count {
        let i_pcm = i_frame * SAMPLES_PER_FRAME;
        //let pcm = &data[i_pcm..(i_pcm+SAMPLES_PER_FRAME)];
        let remaining = data.len() - i_frame * SAMPLES_PER_FRAME;
        let num_samples = remaining.min(SAMPLES_PER_FRAME);
        for i in 2..(2 + SAMPLES_PER_FRAME) {
            pcm_buffer[i] = 0;
        }
        for i in 0..num_samples {
            pcm_buffer[i + 2] = data[i_pcm + i];
        }
        out.extend_from_slice(&dsp_encode_frame(&mut pcm_buffer, num_samples, &coef));
        pcm_buffer[0] = pcm_buffer[14];
        pcm_buffer[1] = pcm_buffer[15];
    }
    EncodeResult { coef, data: out }
}

fn dsp_encode_frame(pcm: &mut [i16], num_samples: usize, coef: &[i16; 16]) -> [u8; 8] {
    let mut in_samples = [[0i32; 16]; 8];
    let mut out_samples = [[0i32; 14]; 8];
    let mut best_index = 0;
    let mut scale = [0i32; 8];
    let mut dist_acc = [0.0f64; 8];
    for i in 0..8 {
        in_samples[i][0] = pcm[0] as i32;
        in_samples[i][1] = pcm[1] as i32;
        let mut distance = 0i32;
        for s in 0..num_samples {
            let v1 = ((pcm[s] as i32 * coef[i * 2 + 1] as i32)
                + (pcm[s + 1] as i32 * coef[i * 2] as i32))
                / 2048;
            in_samples[i][s + 2] = v1;
            let v2 = clamp_i32(pcm[s + 2] as i32 - v1);
            if v2.abs() > distance.abs() {
                distance = v2;
            }
        }
        scale[i] = 0;
        while scale[i] <= 12 && (distance > 7 || distance < -8) {
            distance /= 2;
            scale[i] += 1;
        }
        scale[i] = if scale[i] <= 1 { -1 } else { scale[i] - 2 };
        loop {
            scale[i] += 1;
            dist_acc[i] = 0.0;
            let mut index = 0;
            for s in 0..num_samples {
                let v1 = (in_samples[i][s] * coef[i * 2 + 1] as i32)
                    + (in_samples[i][s + 1] * coef[i * 2] as i32);
                let v2 = ((pcm[s + 2] as i32) << 11) - v1;
                let mut v3 =
                    ((v2 as f64 / (1 << scale[i]) as f64) / 2048.0 + 0.499999).round() as i32;
                if v3 < -8 {
                    if index < -8 - v3 {
                        index = -8 - v3;
                    }
                    v3 = -8;
                } else if v3 > 7 {
                    if index < v3 - 7 {
                        index = v3 - 7;
                    }
                    v3 = 7;
                }
                out_samples[i][s] = v3;
                let v1 = (v1 + ((v3 * (1 << scale[i])) << 1) + 1024) >> 11;
                let v2 = clamp_i32(v1);
                in_samples[i][s + 2] = v2;
                let v3 = pcm[s + 2] as i32 - v2;
                dist_acc[i] += v3 as f64 * v3 as f64;
            }
            let mut x = index + 8;
            while x > 256 {
                scale[i] += 1;
                if scale[i] >= 12 {
                    scale[i] = 11;
                }
                x >>= 1;
            }
            if (scale[i] >= 12) || (index <= 1) {
                break;
            }
        }
    }
    let mut min = f64::MAX;
    for i in 0..8 {
        if dist_acc[i] < min {
            min = dist_acc[i];
            best_index = i;
        }
    }
    for s in 0..num_samples {
        pcm[s + 2] = clamp_i32(in_samples[best_index][s + 2]) as i16;
    }
    let mut frame = [0u8; 8];
    frame[0] = ((best_index as u8) << 4) | (scale[best_index] as u8 & 0xF);
    for s in num_samples..14 {
        out_samples[best_index][s] = 0;
    }
    for y in 0..7 {
        frame[y + 1] = (i32_to_nibble(out_samples[best_index][y * 2]) << 4)
            | (i32_to_nibble(out_samples[best_index][y * 2 + 1]));
    }
    frame
}
