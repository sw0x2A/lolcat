package colorizer

import "testing"

func TestRainbowColorizer_Rainbowize(t *testing.T) {
	type args struct {
		freq float64
		i    float64
	}
	tests := []struct {
		name string
		c    *RainbowColorizer
		args args
		want string
	}{
		{
			name: "first",
			c:    NewRainbowColorizer(),
			args: args{
				freq: 0.2,
				i:    1.0,
			},
			want: "\x1b[38;2;153;223;8m",
		}, {
			name: "second",
			c:    NewRainbowColorizer(),
			args: args{
				freq: 0.2,
				i:    2.0,
			},
			want: "\x1b[38;2;177;205;2m",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			c := &RainbowColorizer{}
			if got := c.Rainbowize(tt.args.freq, tt.args.i); got != tt.want {
				t.Errorf("RainbowColorizer.Rainbowize() = %q, want %q", got, tt.want)
			}
		})
	}
}
