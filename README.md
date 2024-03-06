# Open Source Chain Forensics

## Overview

Blockchain analysis uses various clustering methods to identify and monitor entities that transact on the network. While many of the heuristics used for clustering are well known, they are in practice exclusively used by private surveillance companies that sell the data to financial institutions and governments. There are no public and open-source alternatives for doing forensic analysis on a local full node. The only public analysis tool that exists today is [OXT](https://oxt.me), but it is closed-source and cannot be run locally.

A few reasons why this would be beneficial:
- Exposing bad privacy practices publicly is an incentive to change those behaviors and improve our tools.
- It offers a way for privacy-focused developers to measure and demonstrate the efficacy of the services they build.
- It can highlight the strengths and drawbacks of specific clustering heuristics.
- Maybe it puts surveillance firms out of business.

## Project

Write a program that connects to a bitcoin node (txindex=1), and groups addresses into ownership clusters (aka "entities"). Some ideas:
- Implement common clustering heuristics such as address reuse and common-input ownership (spending from multiple inputs typically implies common ownership over those inputs).
- Identify wallet fingerprints in transactions and the software that was likely used to create them.
- Attempt to identify collaborative transactions such as CoinJoins.
- Provide a user interface for visualizing entities and their flows of funds. 

## Usage

Running the backend server:

```
cd backend && cargo run <path to bitcoin core cookie file>
```

Build and serve the application locally:

```
cd frontend && trunk serve
```
