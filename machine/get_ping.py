import requests

sequences = requests.get("http://127.0.0.1:12345/ping/")
print(sequences)

print(sequences.content)
print(sequences.connection)
print(sequences.cookies)
print(sequences.headers)
print(sequences.history)
print(sequences.elapsed)
print(sequences.encoding)
print(sequences.raw)
print(sequences.reason)
print(sequences.request)
print(sequences.status_code)
print(sequences.url)
print(sequences.text)
