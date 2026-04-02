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

test_that("SyncMessage two-peer sync via SyncStep1/SyncStep2", {
  doc1 <- Doc$new()
  doc2 <- Doc$new()

  # Write content into doc1
  text1 <- doc1$get_or_insert_text("test")
  doc1$with_transaction(function(txn) text1$push(txn, "hello"), mutable = TRUE)

  # doc2 sends its state vector as SyncStep1
  step1 <- SyncMessage$new(
    sync_step1 = doc2$with_transaction(function(txn) txn$state_vector())
  )
  expect_true(step1$is_sync_step1())

  # doc1 responds with a SyncStep2 containing the diff
  step2 <- SyncMessage$new(
    sync_step2 = doc1$with_transaction(
      function(txn) txn$encode_diff_v1(step1$state_vector())
    )
  )
  expect_true(step2$is_sync_step2())

  # Encode and decode SyncStep2 (simulates network transfer)
  step2_decoded <- SyncMessage$decode_v1(step2$encode_v1())
  expect_equal(step2_decoded$step(), "sync_step2")

  # doc2 applies the update from SyncStep2
  doc2$with_transaction(
    function(txn) txn$apply_update_v1(step2_decoded$data()),
    mutable = TRUE
  )

  # Verify doc2 now has the same content
  text2 <- doc2$get_or_insert_text("test")
  doc2$with_transaction(function(txn) {
    expect_equal(text2$get_string(txn), "hello")
  })

  # doc1 makes an incremental edit after initial sync
  sv_before <- doc1$with_transaction(function(txn) txn$state_vector())
  doc1$with_transaction(function(txn) text1$push(txn, " world"), mutable = TRUE)

  # Encode only the incremental diff as an Update message
  update_msg <- SyncMessage$new(
    update = doc1$with_transaction(
      function(txn) txn$encode_diff_v1(sv_before)
    )
  )
  expect_true(update_msg$is_update())

  # Encode and decode Update (simulates network transfer)
  update_decoded <- SyncMessage$decode_v1(update_msg$encode_v1())
  expect_equal(update_decoded$step(), "update")

  # doc2 applies the incremental update
  doc2$with_transaction(
    function(txn) txn$apply_update_v1(update_decoded$data()),
    mutable = TRUE
  )

  # Verify doc2 has the full content
  doc2$with_transaction(function(txn) {
    expect_equal(text2$get_string(txn), "hello world")
  })
})

test_that("SyncMessage equality", {
  sv <- StateVector$decode_v1(as.raw(c(0x00)))
  raw_data <- as.raw(c(0x01, 0x02, 0x03))

  msg1a <- SyncMessage$new(sync_step1 = sv)
  msg1b <- SyncMessage$new(sync_step1 = sv)
  expect_true(msg1a == msg1b)
  expect_false(msg1a != msg1b)

  msg2 <- SyncMessage$new(sync_step2 = raw_data)
  expect_false(msg1a == msg2)
  expect_true(msg1a != msg2)

  msg3a <- SyncMessage$new(update = raw_data)
  msg3b <- SyncMessage$new(update = raw_data)
  expect_true(msg3a == msg3b)
  expect_false(msg3a != msg3b)
  expect_false(msg2 == msg3a)
})

test_that("SyncMessage decode errors on invalid data", {
  expect_s3_class(SyncMessage$decode_v1(as.raw(c(0xff))), "extendr_error")
  expect_s3_class(SyncMessage$decode_v2(as.raw(c(0xff))), "extendr_error")
})
