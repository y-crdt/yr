test_that("Transaction$lock returns a Transaction", {
  doc <- Doc$new()
  trans <- Transaction$lock(doc)
  expect_true(inherits(trans, "Transaction"))
})

test_that("Multiple readonly transaction does not deadlock", {
  doc <- Doc$new()
  text <- doc$get_or_insert_text("article")

  trans1 <- Transaction$lock(doc)
  trans2 <- Transaction$lock(doc)
  expect_true(inherits(trans1, "Transaction"))
  expect_true(inherits(trans2, "Transaction"))
  trans1$unlock()
  trans2$unlock()
})

test_that("Errors when using Transaction after unlock", {
  doc <- Doc$new()
  text <- doc$get_or_insert_text("article")
  trans <- Transaction$lock(doc, mutable = TRUE)
  trans$unlock()

  expect_s3_class(trans$commit(), "extendr_error")
  expect_s3_class(text$get_string(trans), "extendr_error")
})

test_that("Transaction state_vector of empty doc is empty", {
  doc <- Doc$new()
  trans <- Transaction$lock(doc)
  on.exit(trans$unlock())
  sv <- trans$state_vector()
  expect_true(sv$is_empty())
})

for (version in c("v1", "v2")) {
  local({
    test_that(paste("Transaction encode_diff", version, "against current state vector returns empty update"), {
      doc <- Doc$new()
      text <- doc$get_or_insert_text("article")

      trans <- Transaction$lock(doc, mutable = TRUE)
      on.exit(trans$unlock())
      text$insert(trans, 0L, "hello")
      trans$commit()

      sv <- trans$state_vector()
      diff <- trans[[paste0("encode_diff_", version)]](sv)
      expect_true(is.raw(diff))
    })
  }, list(version = version))
}

for (version in c("v1", "v2")) {
  local({
    test_that(paste("apply_update", version, "errors on invalid data"), {
      doc <- Doc$new()
      trans <- Transaction$lock(doc, mutable = TRUE)
      on.exit(trans$unlock())
      expect_s3_class(trans[[paste0("apply_update_", version)]](as.raw(c(0xff))), "extendr_error")
    })
  }, list(version = version))
}
