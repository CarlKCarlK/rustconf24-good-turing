# Good-Turing Estimation

Live demo: <https://carlkcarlk.github.io/rustconf24-good-turing/>

This project implements the **Good-Turing Estimation** algorithm in Rust. The algorithm estimates the probability of unseen events based on the frequency of observed events. Specifically, given an input text file, it analyzes the even-numbered lines to estimate the number of distinct words that appear only on the odd-numbered lines.

[According to I.J. Good and Alan Turing](https://en.wikipedia.org/wiki/Good%E2%80%93Turing_frequency_estimation), a reasonable estimate is the number of words that appear exactly once on the even lines.

The live demo displays both the estimated and actual number of distinct words that appear only on the odd lines.

This is an example from the article [Nine Rules for Running Rust in the Browser](https://medium.com/@carlmkadie).

Project home: <https://github.com/CarlKCarlK/rustconf24-good-turing>

## License

This project is dual-licensed under the MIT and Apache 2.0 licenses. You may choose either license when using this code.
