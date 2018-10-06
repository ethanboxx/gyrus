# Ai Graph

[![pipeline status](https://gitlab.com/efunb/ai-graph/badges/master/pipeline.svg)](https://gitlab.com/efunb/ai-graph/commits/master)
[![License](https://img.shields.io/crates/l/ai-graph.svg)](https://crates.io/crates/ai-graph)
[![Latest version](https://img.shields.io/crates/v/ai-graph.svg)](https://crates.io/crates/ai-graph)
[![downloads-badge](https://img.shields.io/crates/d/ai-graph.svg)](https://crates.io/crates/ai-graph)
[![Coverage](https://codecov.io/gl/efunb/ai-graph/branch/master/graph/badge.svg)](https://codecov.io/gl/efunb/ai-graph)


## How

In Ai graph an gene looks like this.

![What a gene looks like](readme_imgs/diagram.png)

A gene takes a number of inputs each line takes a value from a node or an input the mutates it in some way.
Each node takes all the values from every line that goes into it to find its value.
The output nodes values are used as outputs from the gene.

## Downloads

[Docs](https://gitlab.com/efunb/ai-graph/-/jobs/artifacts/dev/download?job=docs)

## Dependencies

| Dependency | Version | Used for                                                           |
| :--------- | :-----: | -----------------------------------------------------------------: |
| rand       | 0.5     | Generating the random numbers used in the artificial intelligence. |
| rayon      | 1.0.2   | Parallel code.                                                     |

**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/ai-graph).**