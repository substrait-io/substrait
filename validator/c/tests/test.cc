#include <string>
#include <gtest/gtest.h>
#include <substrait_validator.h>

TEST(BasicTest, BasicTest) {

    // To not depend on the Substrait format, just throw garbage at the parser.
    // It should immediately fail to parse that, of course, but we can still
    // do some basic interface testing that way.
    std::string nonsense = "protobuf bytes normally go here";

    // Try parsing nonsense.
    auto handle = substrait_validator_parse(
        reinterpret_cast<const uint8_t*>(nonsense.c_str()),
        nonsense.size()
    );
    ASSERT_NE(handle, nullptr);

    // That should fail.
    EXPECT_EQ(substrait_validator_check(handle), -1);

    // Try getting a list of error messages.
    uint64_t data_size = 0;
    auto data_ptr = substrait_validator_export_diagnostics(handle, &data_size);

    // Those messages should still be valid after freeing the handle.
    substrait_validator_free(handle);

    // Check sanity.
    ASSERT_NE(data_ptr, nullptr);
    EXPECT_GT(data_size, 0);
    EXPECT_EQ(strlen(reinterpret_cast<const char*>(data_ptr)), data_size);
    EXPECT_EQ(
        reinterpret_cast<const char*>(data_ptr),
        std::string("Error (plan): failed to parse proto format\n")
    );

    // Free the buffer.
    substrait_validator_free_exported(data_ptr);

}
