package main

/**
 * A command like cat(1)
 *
 *    $ lolcat foo.txt
 *    $ lolcat foo.txt bar.txt
 *    $ lolcat < foo.txt
 */

import (
	"bufio"
	"fmt"
	"github.com/sw0x2A/lolcat/pkg/colorizer"
	"github.com/sw0x2A/lolcat/pkg/services"
	"log"
	"math/rand"
	"os"
	"time"
)

const (
	freq   = 0.2
	spread = 2.5
)


func lolcat(fh *os.File) {
	rand.Seed(time.Now().UnixNano())
	seed := rand.Intn(255)
	scanner := bufio.NewScanner(fh)
	rgbc := colorizer.NewRGBColorizer()
	cs := services.NewColorizerService(rgbc)
	for lineIndex := 0; scanner.Scan(); lineIndex++ {
		for runeIndex, r := range scanner.Text() {
			fmt.Printf("%s%c", cs.Rainbowize(freq, float64(seed+runeIndex+lineIndex)/spread), r)
		}
		fmt.Printf("%s\n", cs.Reset())
	}
}

func main() {
	if len(os.Args) == 1 {
		lolcat(os.Stdin)
	} else {
		for _, fname := range os.Args[1:] {
			fh, err := os.Open(fname)
			if err != nil {
				log.Fatal(err)
			}
			lolcat(fh)
		}
	}
}
