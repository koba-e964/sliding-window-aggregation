# sliding-window-aggregation

<p align="center">
 <a href="https://crates.io/crates/sliding-window-aggregation">
 <img src="https://img.shields.io/crates/v/sliding-window-aggregation.svg" alt="Crates.io: sliding-window-aggregation">
 </a>
 <a href="https://docs.rs/sliding-window-aggregation">
  <img src="https://docs.rs/sliding-window-aggregation/badge.svg" alt="Documentation">
 </a>
 <a href="https://travis-ci.org/koba-e964/sliding-window-aggregation">
   <img src="https://travis-ci.org/koba-e964/sliding-window-aggregation.svg?branch=master" alt="Build Status">
 </a>
 <a href="LICENSE">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT">
 </a>
</p>

This crate provides an implementation of the Sliding Window Aggregation (SWAg) data structure.

SWAg is basically a queue with a fast folding operation. SWAg supports the following operations, each one of which takes amortized O(1) time:

- `push_back`: push an element to the back of the queue.
- `pop_front`: pop an element from the front of the queue.
- `fold_all`: for an operation `op`, compute the _fold_ of all elements in the queue, i.e. `a1 op a2 op ... op an` if the queue's content is `[a1, a2, ..., an]`.

A detailed explanation is given in [https://scrapbox.io/data-structures/Sliding_Window_Aggregation](https://scrapbox.io/data-structures/Sliding_Window_Aggregation).