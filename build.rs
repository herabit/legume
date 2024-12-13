use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    write_non_empty();
}

// This was more or less just stolen from typenum, with some added pizzaz.
fn array_lengths() -> impl Iterator<Item = u64> {
    // We want to handle all lengths in the range 0..=1024.
    //
    // Additionally we want to handle powers of two,
    // powers of ten, and one plus and one minus each power
    // of two and power of ten.

    // This is some power of two plus one.
    //
    // We add one because we want to handle one past it.
    const HIGHEST: u64 = 4096 + 1;

    fn pow_of(pow: u64, upper: u32) -> impl Iterator<Item = u64> {
        let first = (HIGHEST as f64).log(pow as f64).round() as u32 + 1;
        (first..upper)
            .filter_map(move |i| pow.checked_pow(i))
            .flat_map(|x| [x - 1, x].into_iter().chain(x.checked_add(1)))
    }

    (0..HIGHEST + 1).chain(pow_of(2, 64)).chain(pow_of(2, 10))
}

fn nonzero_array_lengths() -> impl Iterator<Item = u64> {
    array_lengths().skip(1)
}

fn feature_gate_length(value: u64) -> &'static str {
    const U16_MAX: u64 = u16::MAX as u64;
    const U32_MAX: u64 = u32::MAX as u64;
    const U64_MAX: u64 = u64::MAX;

    match value {
        ..=U16_MAX => "",
        ..=U32_MAX => {
            "#[cfg(any(\
                target_pointer_width = \"32\",\
                target_pointer_width = \"64\"\
            ))]\n"
        }
        ..=U64_MAX => "#[cfg(target_pointer_width = \"64\")]\n",
    }
}

fn non_empty_impls() -> String {
    use std::fmt::Write;

    let mut output = String::new();

    output.push_str("use crate::array::{Length, NonEmpty};\n\n");

    nonzero_array_lengths().for_each(|length| {
        let cfg = feature_gate_length(length);

        output.push_str(cfg);
        let _ = writeln!(&mut output, "impl NonEmpty for Length<{length}> {{}}",);
    });

    output
}

fn write_non_empty() {
    let path = {
        let mut path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        path.push("arr_len.rs");

        path
    };

    let non_empty = non_empty_impls();

    fs::write(&path, &non_empty).expect("failed writing arr_len data");
}
