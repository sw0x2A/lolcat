package app

import (
	"github.com/sw0x2A/lolcat/pkg/cli"
	"github.com/sw0x2A/lolcat/pkg/colorizer"
	"github.com/sw0x2A/lolcat/pkg/services"
)

type App struct {
	input *cli.Input
	service *services.ColorizerService
}

func (a *App) Run() {
		ir := a.input.Read()
		a.service.Colorize(ir)
}

func NewApp() *App {
	input := cli.NewInput()
	c := colorizer.NewRainbowColorizer()
	cs := services.NewColorizerService(c)
	return &App{ input, cs }
}
