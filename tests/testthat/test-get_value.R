test_that("get_value() works when querying simple values", {
  expect_equal(get_value("[package]\nname = \"tomleditR\"\nversion = \"0.0.0\"\n", c("package", "version")), "0.0.0")
})