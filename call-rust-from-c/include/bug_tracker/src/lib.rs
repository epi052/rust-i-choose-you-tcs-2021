use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;

/// simple `Bug` container, highest severity bugs are the first to be removed from the tracker  
pub struct BugTracker {
    /// max-heap of `Bug`s
    bugs: BinaryHeap<Bug>,
}

impl BugTracker {
    /// instantiate a new max-heap and return the wrapping struct
    fn new() -> Self {
        Self {
            bugs: BinaryHeap::new(),
        }
    }
}

// the `repr(C)` attribute affects the order/size/alignment of the struct and its fields, making sure
// it conforms to the way that C would lay out the memory. This means C code, and any language
// that can understand C code, can use this struct if it knows about its definition.
#[repr(C)]
#[derive(Eq, PartialEq)]
/// a container representing a single bug
pub struct Bug {
    /// unique bug identifier
    id: *mut c_char,

    /// bug severity, informative -> critical
    severity: Severity,
}

impl Ord for Bug {
    /// required to define ordering of a Bug (severity-based sorting)
    fn cmp(&self, other: &Self) -> Ordering {
        self.severity.cmp(&other.severity)
    }
}

impl PartialOrd for Bug {
    /// required to define ordering of a Bug (severity-based sorting)
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[repr(C)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
/// enum representing the severity of any given Bug
pub enum Severity {
    /// lowest severity
    Informative,

    /// second lowest severity
    Low,

    /// medium severity
    Medium,

    /// second highest severity
    High,

    /// highest severity
    Critical,
}

// the no_mangle attribute tells the compiler to turn off Rust’s name mangling for this function, so
// that we can link to it. When name mangling is turned on, a function named handle_connection ends
// up being mangled to something like _ZN6server17handle_connection17heb3ea72ba341fa07E. A C
// program would have no way of calling the function if it gets mangled.
//
// similarly, the `extern "C"` part of the function declaration forces this function to adhere to
// the C calling convention
#[no_mangle]
/// create a new `BugTracker`, populate it with a few `Bug`s and return its pointer to the caller
pub extern "C" fn new_bugtracker() -> *mut BugTracker {
    let mut bt = BugTracker::new();

    bt.bugs.push(Bug {
        id: CString::new("bug-id-1").unwrap().into_raw(),
        severity: Severity::Informative,
    });

    bt.bugs.push(Bug {
        id: CString::new("bug-id-2").unwrap().into_raw(),
        severity: Severity::Critical,
    });

    bt.bugs.push(Bug {
        id: CString::new("bug-id-3").unwrap().into_raw(),
        severity: Severity::Low,
    });

    // create a new instance of `BugTracker`, we box the result of `BugTracker::new()`
    // (its constructor) the struct is placed onto the heap by `Box::new()`. We then use the
    // heap address and convert it into a raw pointer using `Box::into_raw()`
    Box::into_raw(Box::new(bt))
}

#[no_mangle]
/// free the memory allocated for the given `BugTracker` and any `Bug`s it currently knows about
pub extern "C" fn free_bugtracker(bt: *mut BugTracker) {
    if !bt.is_null() {
        // pointer isn't null, safe to free

        let mut tracker = unsafe {
            // convert the pointer back to Box<BugTracker>, this new Box pointer will eventually
            // go out of scope and in doing so, it will automatically be dropped (freed)
            Box::from_raw(bt)
        };

        while let Some(bug) = tracker.bugs.pop() {
            // if the bugtracker has any bugs left in it, we need to perform the same action to
            // the raw pointer `bug.id`
            unsafe {
                CString::from_raw(bug.id);
            }
        }
    }
}

#[no_mangle]
/// free the memory allocated for the given `Bug`
pub extern "C" fn free_bug(bug: *mut Bug) {
    if !bug.is_null() {
        // pointer isn't null, safe to free
        unsafe {
            // underlying `*mut c_char` needs to be freed. Similar to the `Box::from_raw` below,
            // we'll convert the raw pointer back into a rust type and let it go out of scope,
            // effectively freeing the char array
            CString::from_raw((*bug).id);

            // convert the pointer back to Box<Bug>, this new Box pointer will go out of
            // scope and in doing so, it will automatically be dropped (freed)
            Box::from_raw(bug);
        }
    }
}

#[no_mangle]
/// given a pointer to a BugTracker, return the next highest severity bug
pub extern "C" fn get_next_bug(bt: *mut BugTracker) -> *mut Bug {
    if !bt.is_null() {
        // pointer isn't null, safe to free
        let tracker = unsafe {
            // create a reference from a raw pointer, &mut * indicates that the pointer should be
            // de-referenced and then re-referenced. This gives us a &mut BugTracker to work with
            &mut *bt
        };

        // `if let` is syntactic sugar for a `match` that runs code when the value matches a single
        // pattern and then ignores all other values. We lose the exhaustive checking that `match`
        // enforces, but have less boilerplate
        //
        // the equivalent match would look like
        // match tracker.bugs.pop() {
        //     None => {}
        //     Some(bug) => {
        //         return Box::into_raw(Box::new(bug));
        //     }
        // }
        if let Some(bug) = tracker.bugs.pop() {
            return Box::into_raw(Box::new(bug));
        }
    }

    // return NULL if bt is NULL
    null_mut()
}
