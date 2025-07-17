java:
	java Request.java

python:
	python3 request.py

nodejs:
	node request.js

go:
	go run go/request.go

csharp:
	dotnet run

c:
	gcc -Wall -Wextra -pedantic -o request request.c -lcurl && ./request

rust:
	cargo run -q

clean:
	rm -rf bin obj target request
