package colorizer

type Colorizer interface {
	Colorize(freq float64, i float64) string
}
