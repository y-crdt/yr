#' @include extendr-wrappers.R
NULL

#' @export StateVector
NULL

#' @export
`==.StateVector` <- function(e1, e2) {
  e1$equal(e2)
}

#' @export
`!=.StateVector` <- function(e1, e2) {
  e1$not_equal(e2)
}

#' @export
`<.StateVector` <- function(e1, e2) {
  e1$less_than(e2)
}

#' @export
`<=.StateVector` <- function(e1, e2) {
  e1$less_than_equal(e2)
}

#' @export
`>.StateVector` <- function(e1, e2) {
  e1$greater_than(e2)
}

#' @export
`>=.StateVector` <- function(e1, e2) {
  e1$greater_than_equal(e2)
}
