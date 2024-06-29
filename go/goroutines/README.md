This example demonstrates how to use Goroutines and channels to handle concurrent tasks in a Go CLI application, specifically for fetching web pages.

Run the application:

```
go run main.go
```
Example input and output:

```
Enter URLs to fetch. Enter 'exit' to finish.
https://www.google.com
https://www.github.com
https://www.stackoverflow.com
exit
Worker 1: Fetched https://www.google.com - Status: 200 OK
Worker 2: Fetched https://www.github.com - Status: 200 OK
Worker 3: Fetched https://www.stackoverflow.com - Status: 200 OK
All URLs fetched. Exiting.
```
