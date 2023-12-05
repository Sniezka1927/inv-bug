The update_timestamp function in the provided code is encountering a division error when dividing the result of a multiplication operation by 1000000000000 (Large constant value). This issue seems to be related to the parameters taken from the environment, particularly the block timestamp.

The function returns an error when dividing by 1000000000000, but it works when the denominator is reduced to smaller value (f.e 100).

Manually setting the timestamp delta to any value resolves the issue, explicitly highlighting a non-deterministic aspect tied to dynamic timestamp values. The observed variation in behavior strongly suggests that the fluctuating nature of the timestamp values introduces non-deterministic changes, contributing to the underlying problem.

State updates are not necessary for triggering the bug.

The tests are successful as long as the timestamp_delta is less than 1000. All calls needs to be made within one second,

The issue doesn't seem to be caused by overflow, as no overflow errors are returned, and intermediate calculations are not overflowing.

The issue doesn't appear to be caused by overflow, as the function utilizes checked math for calculations. In the event of overflow or underflow, it returns 0.

The issue may be related to the interaction between the dynamic block timestamp and the multiplication/division operations.
