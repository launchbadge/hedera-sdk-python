import hedera, os

client = hedera.Client("testnet.hedera.com:50001")

# todo: client.set_operator("0:0:2", lambda: os.environ["OPERATOR_SECRET"])
# todo: client.set_operator(hedera.AccountId("0:0:2"), lambda: os.environ["OPERATOR_SECRET"])

# todo: await client.account("0.0.2").balance
balance = client.account(hedera.AccountId("0:0:2")).balance().get()

print(f"balance = {balance} tinybars")
print(f"balance = {balance / 100000000.0} hbars")

# todo: await client.account("0.0.2").info
# todo: info = client.account(hedera.AccountId("0:0:2")).info().get()
