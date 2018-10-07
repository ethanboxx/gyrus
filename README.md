# Gyrus

[![pipeline status](https://gitlab.com/efunb/gyrus/badges/master/pipeline.svg)](https://gitlab.com/efunb/gyrus/commits/master)
[![License](https://img.shields.io/crates/l/gyrus.svg)](https://crates.io/crates/gyrus)
[![Latest version](https://img.shields.io/crates/v/gyrus.svg)](https://crates.io/crates/gyrus)
[![downloads-badge](https://img.shields.io/crates/d/gyrus.svg)](https://crates.io/crates/gyrus)
[![Coverage](https://codecov.io/gl/efunb/gyrus/branch/master/graph/badge.svg)](https://codecov.io/gl/efunb/gyrus)

## Help

If you run into any issues or need help with using `gyrus` in your project please email [incoming+efunb/gyrus@incoming.gitlab.com](mailto:incoming+efunb/gyrus@incoming.gitlab.com)

## How

In `gyrus` an gene looks like this.

![What a gene looks like](readme_imgs/diagram.png)

A gene takes a number of inputs each line takes a value from a node or an input the mutates it in some way.
Each node takes all the values from every line that goes into it to find its value.
The output nodes values are used as outputs from the gene.

## Downloads

[Docs](https://gitlab.com/efunb/gyrus/-/jobs/artifacts/dev/download?job=docs)

## Dependencies

| Dependency | Version | Used for                                                           |
| :--------- | :-----: | -----------------------------------------------------------------: |
| rand       | 0.5     | Generating the random numbers used in the artificial intelligence. |
| rayon      | 1.0.2   | Parallel code.                                                     |
| chrono     | 0.4     | Store time generation was made.                                    |

## **Warning**

**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/gyrus).**