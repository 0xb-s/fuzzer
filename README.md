#  Fuzzer 

This project is designed for both synchronous and asynchronous target functions. It allows developers to automatically generate and test various inputs for functions, uncovering potential crashes, bugs, or unexpected behavior.

**Note:** This project is in a very early stage of development.


## Project Overview

The fuzzer framework allows:

- **Input Generation**: Automatically generates inputs for the target functions in various formats such as binary, text, JSON, or XML.
- **Mutation Support**: You can mutate existing inputs to explore edge cases and unusual input values.
- **Asynchronous and Synchronous Target Support**: You can define both synchronous and asynchronous target functions, allowing you to fuzz a wide variety of code.
- **Error Handling**: The fuzzer reports crashes, timeouts, and other execution issues during fuzzing.

The primary goal is to help identify and resolve defects in code by subjecting it to randomly generated or mutated inputs.

## Usage

1. **Define a target function** you want to fuzz.
2. **Configure the fuzzer** to generate inputs and mutate them if needed.
3. **Run the fuzzer** and review its output to identify issues.


## Documentation

The summary of the documentation can be found [here](documentation/src/SUMMARY.md).
