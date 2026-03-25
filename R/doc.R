#' @include extendr-wrappers.R
NULL

#' @export
print.Doc <- function(self, ...) {
  cat(self$to_string(), "\n", sep = "")
  invisible(self)
}
