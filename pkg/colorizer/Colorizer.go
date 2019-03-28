package colorizer

type Colorizer interface {
	Rainbowize(freq float64, i float64) string
	Reset() string
}
