
<!-- README.md is generated from README.Rmd. Please edit that file -->

# tomleditR

<!-- badges: start -->

[![codecov](https://codecov.io/gh/extendr/tomleditR/branch/main/graph/badge.svg?token=MNPMZPDPGY)](https://codecov.io/gh/extendr/tomleditR)
[![CRAN
status](https://www.r-pkg.org/badges/version/tomleditR)](https://CRAN.R-project.org/package=tomleditR)
[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)

[![R-CMD-check](https://github.com/extendr/tomleditR/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/extendr/tomleditR/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

The goal of tomleditR is to expose the
[toml_edit](https://docs.rs/toml_edit/latest/toml_edit/) crate to R.

## Installation

You can install the development version of tomleditR from
[GitHub](https://github.com/) with:

``` r
# install.packages("devtools")
devtools::install_github("extendr/tomleditR")
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
tomleditR::inspect("")
#> Document {
#>     root: Table(
#>         Table {
#>             decor: Decor {
#>                 prefix: "default",
#>                 suffix: "default",
#>             },
#>             implicit: false,
#>             dotted: false,
#>             doc_position: None,
#>             span: None,
#>             items: {},
#>         },
#>     ),
#>     trailing: empty,
#>     original: Some(
#>         "",
#>     ),
#>     span: None,
#> }
```
