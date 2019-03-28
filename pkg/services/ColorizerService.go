package services

import "github.com/sw0x2A/lolcat/pkg/colorizer"

type ColorizerService struct {
	colorizer colorizer.Colorizer
}

func (c *ColorizerService) Colorize (freq float64, i float64) string {
	return c.colorizer.Colorize(freq, i)
}

func NewColorizerService(c colorizer.Colorizer) *ColorizerService {
	return &ColorizerService{c}
}
