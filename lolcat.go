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
	"log"
	"math"
	"math/rand"
	"os"
	"time"
)

const (
	freq   = 0.2
	spread = 2.5
)

func getRainbowColors(freq float64, i float64) string {
	red := math.Sin(freq*i+0)*127 + 128
	green := math.Sin(freq*i+2*math.Pi/3)*127 + 128
	blue := math.Sin(freq*i+4*math.Pi/3)*127 + 128
	return fmt.Sprintf("\x1b[38;2;%.f;%.f;%.fm", red, green, blue)
}

func lolcat(fh *os.File) {
	rand.Seed(time.Now().UnixNano())
	seed := rand.Intn(255)
	scanner := bufio.NewScanner(fh)
	for lineIndex := 0; scanner.Scan(); lineIndex++ {
		for runeIndex, rune := range scanner.Text() {
			fmt.Printf("%s%c", getRainbowColors(freq, float64(seed+runeIndex+lineIndex)/spread), rune)
		}
		fmt.Printf("\x1b[0m\n")
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
