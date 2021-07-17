use std::ptr::slice_from_raw_parts_mut;

use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use rand::{seq::SliceRandom, Rng};

/// array of bitmasks to apply to individual bytes via xor
static FLIP_ARRAY: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

/// direct port of h0mbre's second version of the bit_flip function used in his blog post
/// https://h0mbre.github.io/Fuzzing-Like-a-Caveman-2/#
#[pyfunction]
fn bit_flip(data: &PyByteArray) -> &PyByteArray {
    // create thread-local random number generator, seeded by the system
    let mut rng = rand::thread_rng();

    // attempt to subrtact 4 from data.len() and store it as a usize. In the event that data.len()
    // is 3 or less, checked_sub will panic, raising the exception below:
    //
    //      pyo3_runtime.PanicException: length of data is too small
    //
    // the original bit_flip function was designed around flipping bytes specifically in a JPEG.
    // Subtracting four from the length here is to preserve the SOI and EOI markers in the provided
    // JPEG.
    let length = data
        .len()
        .checked_sub(4)
        .expect("length of data is too small");

    // only flip 1% of the bits in the bytearray passed into the function
    let flip_mul = length as f64;
    let num_of_flips = (flip_mul * 0.01) as usize;

    // data.data returns a raw pointer to the start of the buffer containing the contents
    // of the bytearray
    let data_ptr = data.data();

    // we create a raw mutable slice from the raw pointer, as they're a bit easier to work with
    let data_slice = slice_from_raw_parts_mut(data_ptr, data.len());

    (0..num_of_flips).for_each(|_| {
        // the extension trait, SliceRandom, from the rand crate allows us to call .choose on
        // our array, giving us a random value from the array
        let mask = FLIP_ARRAY.choose(&mut rng).unwrap();

        // get a random index that will be used as an index into our `data.data` buffer, pointed
        // to by `data_slice`
        let data_index = rng.gen_range(0..length);

        unsafe {
            // dereference the `data_slice` pointer, and index into it while xor'ing the value with our
            // randomized `mask` value
            (*data_slice)[data_index] ^= mask;
        }
    });

    data // return the mutated data to the caller
}

#[pymodule]
/// A Python module implemented in Rust
fn bit_flipper(_py: Python, m: &PyModule) -> PyResult<()> {
    // add the bit_flip function to the given module
    m.add_function(wrap_pyfunction!(bit_flip, m)?)?;
    Ok(())
}
