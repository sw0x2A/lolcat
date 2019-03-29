package main

/**
 * A command like cat(1)
 *
 *    $ lolcat foo.txt
 *    $ lolcat foo.txt bar.txt
 *    $ lolcat < foo.txt
 */

import (
	"github.com/sw0x2A/lolcat/pkg/app"
)

func main() {
	a := app.NewApp()
	a.Run()
}
