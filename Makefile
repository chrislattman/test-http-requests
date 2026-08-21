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
	cmake --build build && cmake --build build --target run-request
# 	gcc -Wall -Wextra -pedantic -o request request.c -lcurl && ./request

rust:
	cargo run -q

clean:
	rm -rf bin obj target request
