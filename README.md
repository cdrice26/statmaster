# StatMaster

This is a WebAssembly library for performing hypothesis tests and confidence intervals in statistics. It is based on [statrs](https://crates.io/crates/statrs) and simply uses its provided distributions to compute hypothesis tests and confidence intervals.

## Usage
### Confidence Intervals
- One-sample Z-interval
```ts
one_samp_z_interval(column: Array<number>, alpha: number)
```
- Two-sample Z-interval
```ts
two_samp_z_interval(column1: Array<number>, column2: Array<number>, alpha: number)
```
- One-sample T-interval
```ts
one_samp_t_interval(column: Array<number>, alpha: number)
```
- Two-sample T-interval
```ts
two_samp_t_interval(column1: Array<number>, column2: Array<number>, alpha: number)
```
- Two-sample Variance Interval
```ts
two_samp_var_interval(column1: Array<number>, column2: Array<number>, alpha: number)
```

Using any of these functions returns an array with the lower and upper bounds.

### Hypothesis Tests
- Two-sample Variance F-Test
```ts
variance_test(column1: Array<number>, column2: Array<number>, tails: "two-sided" | "less" | "greater"): {f: number, p: number}
```
- One-way ANOVA Test
```ts
anova_1way_test(data: Array<Array<number>>): {f: number, p: number}
```
- Linear Regression Test
```ts
regression_test(x: Array<number>, y: Array<number>): {f: number, p: number}
```

## Installation
StatMaster is not yet stable and is not published on npm. To install, clone this repo, run ```wasm-pack build``` followed by ```npm link```. Then in the project you wish to use this in, run ```npm link statmaster```. You will need to run ```npm link statmaster``` any time you refresh your dependencies.