package modules

import (
	"SysPerfTUI/globals"
	"context"
	"math/rand"
	"strconv"
	"time"

	"github.com/mum4k/termdash"
	"github.com/mum4k/termdash/cell"
	"github.com/mum4k/termdash/container"
	"github.com/mum4k/termdash/linestyle"
	"github.com/mum4k/termdash/terminal/tcell"
	"github.com/mum4k/termdash/terminal/terminalapi"
	"github.com/mum4k/termdash/widgets/barchart"
	"github.com/mum4k/termdash/widgets/button"
	"github.com/mum4k/termdash/widgets/donut"
	"github.com/mum4k/termdash/widgets/gauge"
)
func playGauge(ctx context.Context, g *gauge.Gauge, delay time.Duration, percent int32) {
	progress := int(percent)

	ticker := time.NewTicker(delay)
	defer ticker.Stop()
	for {
		select {
		case <-ticker.C:
			if err := g.Percent(progress); err != nil {
				panic(err)
			}
			progress = int(percent)
		case <-ctx.Done():
			return
		}
	}
}
func playDonut(ctx context.Context, d *donut.Donut, delay time.Duration) {
	progress := int(globals.CpuDataBuf[0])

	ticker := time.NewTicker(delay)
	defer ticker.Stop()
	for {
		select {
		case <-ticker.C:
			if err := d.Percent(progress); err != nil {
				panic(err)
			}
			progress = int(globals.CpuDataBuf[0])
		case <-ctx.Done():
			return
		}
	}
}

func playBarChart(ctx context.Context, bc *barchart.BarChart, delay time.Duration) {
	const max = 100

	ticker := time.NewTicker(delay)
	defer ticker.Stop()
	for {
		select {
		case <-ticker.C:
			var values []int
			for i := 1; i <= int(globals.InitCpuData); i++ {
				temp := globals.CpuDataBuf[i]
				if temp > 100 || temp < 0 {
					temp = 100
				}
				values = append(values, int(temp))
			}

			if err := bc.Values(values, max); err != nil {
				panic(err)
			}

		case <-ctx.Done():
			return
		}
	}
}

func RenderWidgets() {
	tcell.ColorMode(terminalapi.ColorMode256)
	t, err := tcell.New()
	if err != nil {
		panic(err)
	}
	defer t.Close()

	ctx, cancel := context.WithCancel(context.Background())
	bars := make([]cell.Color, globals.InitCpuData)
	bars_text_color := make([]cell.Color, globals.InitCpuData)
	bars_text := make([]string, globals.InitCpuData)
	temp := "CPU"
	for i := int32(0); i < globals.InitCpuData; i++ {
		bars[i] = cell.ColorAqua
		bars_text_color[i] = cell.ColorRed
		bars_text[i] = temp + strconv.Itoa(int(i+1))
	}
	bc, err := barchart.New(
		barchart.BarColors(bars),
		barchart.ValueColors(bars_text_color),
		barchart.ShowValues(),
		barchart.BarWidth(6),
		barchart.Labels(bars_text),
	)
	if err != nil {
		panic(err)
	}
	donut_color_1 := cell.ColorAqua
	donut_color_2 := cell.ColorRed
	overallusage, err := donut.New(
		donut.CellOpts(cell.FgColor(donut_color_1)),
		donut.Label("Overall CPU usage", cell.FgColor(donut_color_2)),
	)
	if err != nil {
		panic(err)
	}
	go playDonut(ctx, overallusage, time.Second)
	mem_color := []cell.Color{cell.ColorRed}

	used_ram, err := gauge.New(
		gauge.Height(1),
		gauge.Color(mem_color[0]),
		gauge.BorderTitle("Used RAM"),
	)
	if err != nil {
		panic(err)
	}
	go playGauge(ctx, used_ram, time.Second, globals.Mem_used_percent)

	available_ram, err := gauge.New(
		gauge.Height(1),
		gauge.Color(mem_color[0]),
		gauge.BorderTitle("Available RAM"),
	)
	if err != nil {
		panic(err)
	}
	go playGauge(ctx, available_ram, time.Second, globals.Mem_available_percentage)

	cached_ram, err := gauge.New(
		gauge.Height(1),
		gauge.Color(mem_color[0]),
		gauge.BorderTitle("Cached RAM"),
	)
	if err != nil {
		panic(err)
	}
	go playGauge(ctx, cached_ram, time.Second, globals.Mem_cached_percentage)

	free_ram, err := gauge.New(
		gauge.Height(1),
		gauge.Color(mem_color[0]),
		gauge.BorderTitle("Free RAM"),
	)
	if err != nil {
		panic(err)
	}
	go playGauge(ctx, free_ram, time.Second, globals.Mem_free_percentage)

	changecolorB, err := button.New("(c)olor", func() error {
		for i := int32(0); i < globals.InitCpuData; i++ {
			bars[i] = cell.ColorNumber(rand.Intn(256))
			bars_text_color[i] = cell.ColorNumber(rand.Intn(256))
		}
		//[TODO]Change color of donut and gagues too, I dont seem to be able to do it!
		return nil
	},
	button.FillColor(cell.ColorNumber(220)),
	button.GlobalKey('c'),
)

resetcolorB, err := button.New("(r)eset", func() error {
	for i := int32(0); i < globals.InitCpuData; i++ {
		bars[i] = cell.ColorAqua
		bars_text_color[i] = cell.ColorRed
	}
	return nil
},
button.FillColor(cell.ColorNumber(220)),
button.GlobalKey('r'),
)
if err != nil {
	panic(err)
}
go playBarChart(ctx, bc, 1*time.Second)


c, err := container.New(
	t,
	container.Border(linestyle.Light),
	container.BorderTitle("PRESS Q TO QUIT"),
	container.SplitHorizontal(
		container.Top(
			container.SplitVertical(
				container.Left(
					container.Border(linestyle.Light),
					container.BorderTitle("CPU Per core usage"),
					container.PlaceWidget(bc),
				),
				container.Right(
					//Pie chart goes here
					container.Border(linestyle.Light),
					container.BorderTitle("Total CPU usage"),
					container.PlaceWidget(overallusage),
				),
			),
		),
		container.Bottom(
			container.SplitVertical(
				container.Left(
					container.Border(linestyle.Light),
					container.BorderTitle("Options"),
					container.SplitVertical(
						container.Left(container.PlaceWidget(changecolorB)),
						container.Right(container.PlaceWidget(resetcolorB)),
					),
				),container.Right(
					//Memory stuff go here
					container.Border(linestyle.Light),
					container.BorderTitle("RAM Usage information | Total RAM detected : "+strconv.FormatFloat(globals.Mem_total,'g',5,64)+" GiB"),
					container.SplitHorizontal(
						container.Top(
							container.SplitVertical(
								container.Left(
									container.Border(linestyle.Light),
									container.BorderTitle("Total Used RAM : "+strconv.FormatFloat(globals.Mem_used,'g',5,64)+" GiB"),
									container.PlaceWidget(used_ram),
								),
								container.Right(
									container.Border(linestyle.Light),
									container.BorderTitle("Total Available RAM : "+strconv.FormatFloat(globals.Mem_available,'g',5,64)+" GiB"),
									container.PlaceWidget(available_ram),
								),
							),
						),
						container.Bottom(
							container.SplitVertical(
								container.Left(
									container.Border(linestyle.Light),
									container.BorderTitle("Total Cached RAM : "+strconv.FormatFloat(globals.Mem_cached,'g',5,64)+" GiB"),
									container.PlaceWidget(cached_ram),
								),
								container.Right(
									container.Border(linestyle.Light),
									container.BorderTitle("Total Free RAM : "+strconv.FormatFloat(globals.Mem_free,'g',5,64)+" GiB"),
									container.PlaceWidget(free_ram),
								),
							),
						),
					),
				),
			),
		),
	),
)
if err != nil {
	panic(err)
}

quitter := func(k *terminalapi.Keyboard) {
	if k.Key == 'q' || k.Key == 'Q' {
		cancel()
		return
	}
}

if err := termdash.Run(ctx, t, c, termdash.KeyboardSubscriber(quitter)); err != nil {
	panic(err)
}
}
