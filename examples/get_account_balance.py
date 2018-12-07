import hedera

client = hedera.Client("testnet.hedera.com:50001")

# todo: await client.account("0.0.2").balance
balance = client.account("0:0:2").balance().get()

print(f"balance = {balance} tinybars")
