import hedera
import os
import time

# Generate a new key pair for the new account

secret, _ = hedera.SecretKey.generate("")
public = secret.public

print(f"secret = {secret}")
print(f"public = {public}")

# Operator is the account that sends the transaction to the network
# This account is charged for the transaction fee

operator = os.environ["OPERATOR"]
operator_secret = hedera.SecretKey(os.environ["OPERATOR_SECRET"])

# Connect to a specific testnet node

client = hedera.Client("testnet.hedera.com:50001")

# Create our account

tx = client.create_account()
tx.operator = operator
tx.key = public
tx.initial_balance = 45
tx.sign(operator_secret)

tx_id = tx.execute()
print(f"created account; transaction = {tx_id}")

# If we got here we know we passed pre-check
# Depending on your requirements that may be enough for some kinds of transactions

# Wait 2 seconds to be sure the account is created ..
time.sleep(2)

# Get the receipt and check the status to prove it was successful
receipt = client.transaction(tx_id).receipt().get()
if receipt.status != 1:
    raise Exception(f"transaction has a non-successful status: {receipt.status}")

# Account can be `None` if we were not creating an account
print(f"account = {receipt.account_id}")
