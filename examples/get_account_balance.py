import hedera

client = hedera.Client("testnet.hedera.com:50001")
balance = client.account("0:0:2").balance().get()

print("balance = {} tinybars".format(balance))
