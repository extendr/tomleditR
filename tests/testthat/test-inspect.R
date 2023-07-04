test_that("inspect() from empty string", {
  expect_snapshot(inspect(""))
})

test_that("inspect() throws without an argument", {
  expect_error(inspect())
})

test_that("inspect() throws is received character vector", {
  expect_error(inspect(c("", "")))
})