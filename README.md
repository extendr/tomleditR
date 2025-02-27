
<!-- README.md is generated from README.Rmd. Please edit that file -->

# tomleditR (archived)

<!-- badges: start -->

[![codecov](https://codecov.io/gh/extendr/tomleditR/branch/main/graph/badge.svg?token=MNPMZPDPGY)](https://codecov.io/gh/extendr/tomleditR)
[![CRAN
status](https://www.r-pkg.org/badges/version/tomleditR)](https://CRAN.R-project.org/package=tomleditR)
[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
[![R-CMD-check](https://github.com/extendr/tomleditR/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/extendr/tomleditR/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

> \[!WARNING\]
>
> `tomleditR` has been archived. Refer to
> [`tomledit`](https://github.com/extendr/tomledit) as the successor to
> `tomleditR`.

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
#>  [1] "Document {"                          
#>  [2] "    root: Table("                    
#>  [3] "        Table {"                     
#>  [4] "            decor: Decor {"          
#>  [5] "                prefix: \"default\","
#>  [6] "                suffix: \"default\","
#>  [7] "            },"                      
#>  [8] "            implicit: false,"        
#>  [9] "            dotted: false,"          
#> [10] "            doc_position: None,"     
#> [11] "            span: None,"             
#> [12] "            items: {},"              
#> [13] "        },"                          
#> [14] "    ),"                              
#> [15] "    trailing: empty,"                
#> [16] "    original: Some("                 
#> [17] "        \"\","                       
#> [18] "    ),"                              
#> [19] "    span: None,"                     
#> [20] "}"
```

Or you can take a peek at a specific value:

``` r
tomleditR::get_value("[package]\nname = \"tomleditR\"\nversion = \"0.0.0\"\n", c("package", "version"))
#> [1] "0.0.0"
```
