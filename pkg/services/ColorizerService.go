package services

import (
	"bufio"
	"fmt"
	"github.com/sw0x2A/lolcat/pkg/colorizer"
	"math/rand"
	"os"
	"time"
)

const (
	freq   = 0.2
	spread = 2.5
)

type ColorizerService struct {
	colorizer colorizer.Colorizer
}

func (c *ColorizerService) Colorize(fh *os.File) {
	rand.Seed(time.Now().UnixNano())
	seed := rand.Intn(255)
	scanner := bufio.NewScanner(fh)
	for lineIndex := 0; scanner.Scan(); lineIndex++ {
		for runeIndex, r := range scanner.Text() {
			fmt.Printf("%s%c", c.colorizer.Rainbowize(freq, float64(seed+runeIndex+lineIndex)/spread), r)
		}
		fmt.Printf("%s\n", c.colorizer.Reset())
	}
}

func NewColorizerService(c colorizer.Colorizer) *ColorizerService {
	return &ColorizerService{c}
}
