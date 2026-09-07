java:
	java Request.java

python:
	python3 request.py

nodejs:
	node request.js

go:
	go run go/request.go

csharp:
	dotnet run -v q

c:
	cmake --build build --target run-request
# 	gcc -Wall -Wextra -pedantic -o request request.c -lcurl && ./request

rust:
	cargo run -q

clean:
	dotnet clean && cmake --build build --target clean && cargo clean
