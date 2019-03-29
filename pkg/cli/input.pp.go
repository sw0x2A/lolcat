package cli

import "os"
import "log"

type Input struct {
}

func (i *Input) Read() *os.File {
	if len(os.Args) == 1 {
		return os.Stdin
	} else {
		for _, fname := range os.Args[1:] {
			fh, err := os.Open(fname)
			if err != nil {
				log.Fatal(err)
			}
			return fh
		}
	}
	panic("no file handle")
}

func NewInput() *Input {
	return &Input{}
}
