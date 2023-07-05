#' @export
print.TomlDocument <- function(x, ...) {
    print("TomlDocument")
}

#' @export
get_list <- function(x, ...) {
    UseMethod("get_list")
}

#' @export
get_list.TomlDocument <- function(x, ...) {
    x$get2(list(...))
}