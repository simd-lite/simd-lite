// ARM Neon intrinsic specification.
// 
// This file contains the sepcification for a number of 
// intrinsics that allows us to generate them along with
// their test cases.
//
// To the syntax of the file - it's not very intelligently parsed!
//
// # Comments
// start with AT LEAST two, or four or more slashes  so // is a
// comment /////// is too.
//
// # Sections
// Sections start with EXACTLY three slasshes followed
// by AT LEAST one space. Sections are used for two things:
//
// 1) they serve as the doc comment for the given intrinics.
// 2) they reset all variables (name, fn, etc.)
//
// # Variables
//
// name    - The prefix of the function, suffixes are auto
//           generated by the type they get passed.
//
// fn      - The function to call in rust-land.
//
// aarch64 - The intrinisc to check on aarch64 architecture.
//           If this is given but no arm intrinsic is provided,
//           the function will exclusively be generated for
//           aarch64.
//           This is used to generate both aarch64 specific and
//           shared intrinics by first only specifying th aarch64
//           variant then the arm variant.
// 
// arm     - The arm v7 intrinics used to checked for arm code
//           generation. All neon functions available in arm are
//           also available in aarch64. If no aarch64 intrinic was
//           set they are assumed to be the same.
//
// a       - First input for tests, it gets scaled to the size of
//           the type.
//
// b       - Second input for tests, it gets scaled to the size of
//           the type.
//
// # special values
//
// TRUE - 'true' all bits are set to 1
// FALSE - 'false' all bits are set to 0
// FF - same as 'true'
// MIN - minimal value (either 0 or the lowest negative number)
// MAX - maximal value propr to overflow
//
// # validate <values>
// Validates a and b aginst the expected result of the test.
// The special values 'TRUE' and 'FALSE' can be used to
// represent the corect NEON representation of true or
// false values. It too gets scaled to the type.
// 
// Validate needs to be called before generate as it sets
// up the rules for validation that get generated for each
// type.
// # generate <types>
// The generate command generates the intrinsics, it uses the
// Variables set and can be called multiple times while overwriting
// some of the variables.

