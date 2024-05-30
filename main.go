package main

import "fmt"

func main() {
	query, err := NewSqlBuilder().SelectFrom("two").SQL()
	fmt.Println(err)
	fmt.Println(query)
}
