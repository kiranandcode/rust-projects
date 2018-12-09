## Notes from networking in rust

These are some questions I've made to help assimilating the information presented in the book (particularly important as it is too easy to just word for word copy the text without thinking what it does).


1. Write a simple TCP based echo server in rust.

	Hint: ```
		You'll need TcpStream, and TcpListener to start with.
		Set up a tcp listener, accept all incoming connections, and then read their input and return it.
		You might want to import the read/write traits from the stdlib *nudge nudge, wink wink*.
	```

2. Use netcat to connect and test the server out.
```
	Hint:	netcat <server-ip> <port>
```

3. Write a simple TCP client to connect to your server to test it out.
