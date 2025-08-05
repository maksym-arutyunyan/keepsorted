# SPECS

Defines the expected behavior of `keepsorted`.

## Problem

Unsorted lists in source and config files are tedious to maintain and can detach comments from the items they describe.

## Audience

Developers and CI systems that need deterministic ordering while keeping human commentary intact.

## Goals

- Detect annotated blocks or supported file types that require sorting.
- Reorder items deterministically while preserving associated comments.
- Offer check, diff, and fix modes so projects can enforce or apply sorting.

## Out of scope

Custom sort orders for Rust derive macros.
Derive attributes are either alphabetized or use canonical order.

## Training Module

The training module tunes strategy parameters by testing many
configurations and ranking them with standard performance metrics.

- Historical data is split into train and test sets. By default, the
  most recent four months form the test window, but this period is
  configurable.
- Backtests run on the train portion for every parameter combination.
- The best-performing configuration is evaluated on the held-out test
  data.
- Rankings use metrics such as return on investment and Sharpe ratio.
- Output includes a concise summary file so results are easy to compare.
- Runs are deterministic and optimized for quick feedback on demo
  setups.
