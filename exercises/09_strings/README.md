# Strings

Rust has two main string types: `String` and `&str`. A `String` is an owned,
growable UTF-8 buffer. A `&str` is a borrowed string slice.

This section now builds string skills from beginner to advanced:

- converting between string slices and owned strings,
- mutating strings with `push` and `push_str`,
- formatting without taking ownership,
- trimming, replacing, lowercasing, and normalizing text,
- understanding bytes versus characters in UTF-8,
- returning borrowed slices from string functions,
- splitting and collecting owned fields,
- transforming characters and words,
- safely taking character prefixes,
- parsing small string formats,
- borrowing substrings from parser output,
- rendering simple templates.

## Further information

- [Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [String](https://doc.rust-lang.org/std/string/struct.String.html)
- [`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [`format!`](https://doc.rust-lang.org/std/macro.format.html)
