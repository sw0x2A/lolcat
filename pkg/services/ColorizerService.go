package services

import "github.com/sw0x2A/lolcat/pkg/colorizer"

type ColorizerService struct {
	colorizer colorizer.Colorizer
}

func (c *ColorizerService) Rainbowize(freq float64, i float64) string {
	return c.colorizer.Rainbowize(freq, i)
}

func (c *ColorizerService) Reset() string {
	return c.colorizer.Reset()
}

func NewColorizerService(c colorizer.Colorizer) *ColorizerService {
	return &ColorizerService{c}
}
