use std::ffi::{c_void, CString};
use std::os::raw::c_char;

// equivalent to -lclhash
#[link(name = "clhash")]
extern "C" {
    /// Convenience method. Will generate a random key from two 64-bit seeds.
    /// Caller is responsible to call "free" on the result.
    ///
    /// original definition:
    ///   void * get_random_key_for_clhash(uint64_t seed1, uint64_t seed2);
    fn get_random_key_for_clhash(seed1: u64, seed2: u64) -> *mut c_void;

    /// random: the random data source (can be generated with get_random_key_for_clhash)
    /// stringbyte: the input data source, could be anything you want to hash
    /// length: number of bytes in the string
    ///
    /// original definition:
    ///   uint64_t clhash(const void* random, const char * stringbyte, const size_t lengthbyte);
    fn clhash(random: *mut c_void, string_byte: *const c_char, length_byte: usize) -> u64;
}

fn main() {
    // Foreign functions are assumed to be unsafe so calls to them need to be wrapped with unsafe {}
    let random = unsafe {
        let raw_void = get_random_key_for_clhash(0x23a23cf5033c3c81, 0xb3816f6a2c68e530);
        assert!(!raw_void.is_null()); // fail fast if we get a null pointer back from the call
        raw_void
    };

    // create two c-style strings that we can pass to the hashing function
    let my_dog = CString::new("my dog").unwrap();
    let my_cat = CString::new("my cat").unwrap();

    // to call clhash, we pass in the pointer to random, a pointer to a valid zero-terminated
    // c-style string, and the length of the string we're passing in
    let dog_hash = unsafe { clhash(random, my_dog.as_ptr(), 7) };

    // for demonstration purposes we'll do the same thing with 'my cat' and grab a second dog hash
    let cat_hash = unsafe { clhash(random, my_cat.as_ptr(), 7) };
    let dog_hash_2 = unsafe { clhash(random, my_dog.as_ptr(), 7) };

    println!("dog_hash is {}", dog_hash);
    println!("second_dog_hash is {}", dog_hash_2);
    println!("cat_hash is {}", cat_hash);

    println!();

    println!("dog_hash == cat_hash: {}", dog_hash == cat_hash);
    println!("dog_hash == second_dog_hash: {}", dog_hash == dog_hash_2);

    // recall from the clhash docstring: "Caller is responsible to call "free" on the result"
    //
    // the clhash source code uses `posix_memalign`. POSIX requires that memory obtained from
    // `posix_memalign()` can be freed using `free`. Rust may or may not be using the same allocator.
    // So, in order to safely deallocate our pointer to `random`, we need to call `free` function
    // provided by libc
    unsafe { libc::free(random) };

    // valgrind output shows:
    //    ==1850166== HEAP SUMMARY:
    //    ==1850166==     in use at exit: 0 bytes in 0 blocks
    //    ==1850166==   total heap usage: 15 allocs, 15 frees, 4,263 bytes allocated
}
