import hedera
import os

operator = os.environ["OPERATOR"]

client = hedera.Client("testnet.hedera.com:50001")

balance = client.account(operator).balance().get()

print(f"balance = {balance} tinybars")
print(f"balance = {balance / 100000000.0} hbars")

# todo: info = client.account(hedera.AccountId("0:0:2")).info().get()
