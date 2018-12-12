import hedera

secret, mnemonic = hedera.SecretKey.generate("")
public = secret.public

print(f"secret   = {secret}")
print(f"mnemonic = {mnemonic}")
print(f"public   = {public}")
