MAX_FEE_BASIS_POINTS = 10000  # 100%
NORMAL_FEE_RATE = 500  # 5%
MAX_FEE = 1000  # Maximum fee amount

def vulnerable_get_transfer_inverse_fee(post_fee_amount, fee_rate):
    if fee_rate == MAX_FEE_BASIS_POINTS:
        return MAX_FEE
    else:
        # Simplified calculation for demonstration
        return (post_fee_amount * fee_rate) // (10000 - fee_rate)

def correct_get_transfer_inverse_fee(post_fee_amount, fee_rate):
    # Always calculate based on percentage
    return (post_fee_amount * fee_rate) // (10000 - fee_rate)

# Test cases
test_amounts = [1000, 10000, 100000, 1000000]

print("Vulnerable function results:")
for amount in test_amounts:
    fee = vulnerable_get_transfer_inverse_fee(amount, MAX_FEE_BASIS_POINTS)
    print(f"Transfer amount: {amount}, Fee: {fee}, Received: {amount - fee}")

print("\nCorrect function results:")
for amount in test_amounts:
    fee = correct_get_transfer_inverse_fee(amount, MAX_FEE_BASIS_POINTS)
    print(f"Transfer amount: {amount}, Fee: {fee}, Received: {amount - fee}")

print("\nComparison with normal fee rate:")
for amount in test_amounts:
    normal_fee = vulnerable_get_transfer_inverse_fee(amount, NORMAL_FEE_RATE)
    print(f"Transfer amount: {amount}, 5% Fee: {normal_fee}")
