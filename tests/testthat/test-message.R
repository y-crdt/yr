test_that("SyncMessage construction, step detection, and accessors", {
  sv <- StateVector$decode_v1(as.raw(c(0x00)))

  msg1 <- SyncMessage$new(sync_step1 = sv)
  expect_equal(msg1$step(), "sync_step1")
  expect_true(msg1$is_sync_step1())
  expect_false(msg1$is_sync_step2())
  expect_false(msg1$is_update())
  expect_true(inherits(msg1$state_vector(), "StateVector"))
  expect_s3_class(msg1$data(), "extendr_error")

  raw_data <- as.raw(c(0x01, 0x02, 0x03))

  msg2 <- SyncMessage$new(sync_step2 = raw_data)
  expect_equal(msg2$step(), "sync_step2")
  expect_false(msg2$is_sync_step1())
  expect_true(msg2$is_sync_step2())
  expect_false(msg2$is_update())
  expect_equal(msg2$data(), raw_data)
  expect_s3_class(msg2$state_vector(), "extendr_error")

  msg3 <- SyncMessage$new(update = raw_data)
  expect_equal(msg3$step(), "update")
  expect_false(msg3$is_sync_step1())
  expect_false(msg3$is_sync_step2())
  expect_true(msg3$is_update())
  expect_equal(msg3$data(), raw_data)
  expect_s3_class(msg3$state_vector(), "extendr_error")

  # Exactly one argument required
  expect_s3_class(SyncMessage$new(), "extendr_error")
  expect_s3_class(
    SyncMessage$new(sync_step2 = raw_data, update = raw_data),
    "extendr_error"
  )
})
