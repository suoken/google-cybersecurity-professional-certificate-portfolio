from cryptography.fernet import Fernet

msg = 'Hello World'

key = Fernet.generate_key() # generating a key using fernet

fernet = Fernet(key) # new instance of Fernet with a key

encryptedMsg = fernet.encrypt(msg.encode()) # use the instance to encrypt a message

print('msg: ', msg)
print('encrypted msg: ', encryptedMsg)

decryptedMsg = fernet.decrypt(encryptedMsg).decode() # decrypt string with instance of key that was used for encrypting string 

print('decrypted string: ', decryptedMsg)
