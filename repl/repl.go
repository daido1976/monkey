package repl

import (
	"bufio"
	"fmt"
	"io"
	"os"

	"github.com/daido1976/monkey/lexer"
	"github.com/daido1976/monkey/token"
)

const PROMPT = "monkey> "

func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)

	for {
		fmt.Printf(PROMPT)
		scanned := scanner.Scan()
		if !scanned {
			os.Exit(1)
		}

		line := scanner.Text()
		if line == "exit" {
			os.Exit(0)
		}

		l := lexer.New(line)

		for tok := l.NextToken(); tok.Type != token.EOF; tok = l.NextToken() {
			fmt.Printf("%+v\n", tok)
		}
	}
}
