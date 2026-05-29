//! ABI layout assertions for the `#[repr(C)]` structs shared with the Swift bridge.
//!
//! These structs are written by the Swift `@_cdecl` bridge (via raw pointers)
//! and read back on the Rust side. If their size or alignment ever drifts from
//! what the Swift side expects, the marshalled bytes silently corrupt. These
//! tests pin the layout so accidental field reordering / type changes are caught
//! at `cargo test` time rather than as runtime garbage.

use std::mem::{align_of, size_of};

use speech::ffi::{sp_verify_ffi_layout, RecognitionMetadataRaw, TranscriptionSegmentRaw};

#[test]
fn transcription_segment_raw_layout() {
    // *mut c_char (8) + f32 (4) + pad (4) + f64 (8) + f64 (8)
    assert_eq!(
        size_of::<TranscriptionSegmentRaw>(),
        32,
        "TranscriptionSegmentRaw size drifted"
    );
    assert_eq!(
        align_of::<TranscriptionSegmentRaw>(),
        8,
        "TranscriptionSegmentRaw alignment drifted"
    );
}

#[test]
fn recognition_metadata_raw_layout() {
    // bool (1) + pad (7) + 4 x f64 (32)
    assert_eq!(
        size_of::<RecognitionMetadataRaw>(),
        40,
        "RecognitionMetadataRaw size drifted"
    );
    assert_eq!(
        align_of::<RecognitionMetadataRaw>(),
        8,
        "RecognitionMetadataRaw alignment drifted"
    );
}

/// Cross-language ABI check: asks the Swift bridge to verify that *its*
/// `MemoryLayout` (size/stride/alignment) for the FFI structs matches the values
/// pinned on the Rust side. A `false` return means the Rust and Swift layouts
/// genuinely disagree, which is a real ABI bug.
#[test]
fn ffi_layout_matches_swift() {
    // SAFETY: `sp_verify_ffi_layout` takes no arguments and only reads
    // compile-time `MemoryLayout` constants in the Swift bridge.
    let matches = unsafe { sp_verify_ffi_layout() };
    assert!(
        matches,
        "Swift FFI struct layout disagrees with Rust layout (ABI mismatch)"
    );
}
