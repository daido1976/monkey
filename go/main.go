package main

import (
	"fmt"
	"os"
	"os/user"

	"github.com/daido1976/monkey/go/repl"
)

func main() {
	user, err := user.Current()
	if err != nil {
		panic(err)
	}

	fmt.Printf("Hello %s! This is the Monky programming language!\n", user.Username)
	repl.Start(os.Stdin, os.Stdout)
}
