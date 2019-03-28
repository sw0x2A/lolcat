package colorizer

import (
	"fmt"
	"math"
)

type RGBColorizer struct {
}

func (c *RGBColorizer) Rainbowize(freq float64, i float64) string {
	red := math.Sin(freq*i+0)*127 + 128
	green := math.Sin(freq*i+2*math.Pi/3)*127 + 128
	blue := math.Sin(freq*i+4*math.Pi/3)*127 + 128
	return fmt.Sprintf("\x1b[38;2;%.f;%.f;%.fm", red, green, blue)
}

func (c *RGBColorizer) Reset() string {
	return fmt.Sprintf("\x1b[0m")
}

func NewRGBColorizer() *RGBColorizer {
	return &RGBColorizer{}
}
