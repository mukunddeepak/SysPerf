package modules

import (
	"SysPerfTUI/globals"
	"context"
	//"math/rand"
	"strconv"
	"time"
  "math"
  "math/rand"

	"fmt"

	"github.com/mum4k/termdash"
	"github.com/mum4k/termdash/cell"
	"github.com/mum4k/termdash/container"
	"github.com/mum4k/termdash/linestyle"
	"github.com/mum4k/termdash/terminal/tcell"
	"github.com/mum4k/termdash/terminal/terminalapi"
	"github.com/mum4k/termdash/widgets/barchart"
	"github.com/mum4k/termdash/widgets/donut"
	"github.com/mum4k/termdash/widgets/gauge"
	"github.com/mum4k/termdash/widgets/text"
	"github.com/mum4k/termdash/widgets/linechart"
	"github.com/mum4k/termdash/widgets/button"
)
func playGauge(ctx context.Context, g *gauge.Gauge, delay time.Duration, percent *int32) {
	progress := int(*percent)

	ticker := time.NewTicker(delay)
	defer ticker.Stop()
	for {
		select {
		case <-ticker.C:
			if err := g.Percent(progress); err != nil {
				panic(err)
			}
			progress = int(*percent)
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
func writeText(widget *text.Text, time_dur time.Duration, variable *float64){
	for{
		widget.Reset()
		widget.Write(strconv.FormatFloat(*variable,'g',5,64)+" GiB")
		time.Sleep(time_dur)
	}
}

//TO DELETE
func sineInputs() []float64 {
	var res []float64

	for i := 0; i < 200; i++ {
		v := math.Sin(float64(i) / 100 * math.Pi)
		res = append(res, v)
	}
	return res
}
func playLineChart(ctx context.Context, lc *linechart.LineChart, delay time.Duration) {
	ticker := time.NewTicker(delay) //Using timeseries here
	defer ticker.Stop()
	for{
		select {
		case <-ticker.C:
			if err := lc.Series("first", globals.CpuGraphBuf,linechart.SeriesCellOpts(cell.FgColor(cell.ColorNumber(33)))); err != nil {
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
	printed_once := false
	init_value_x := t.Size().X
	init_value_y := t.Size().Y
	for{
		if(t.Size().X<158 || t.Size().Y<39){
			if !printed_once{
				fmt.Println("Please reduce terminal size")
				printed_once = true
			}
		}else if t.Size().X!=init_value_x || t.Size().Y!=init_value_y{
			fmt.Print("Please reduce terminal size further!")
			init_value_y = t.Size().Y
			init_value_x = t.Size().X
		}else{
			fmt.Println("Sized!")
			break
		}
	}
	time.Sleep(time.Second)
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
		barchart.BarWidth(5),
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
	mem_color := make([]cell.Color, 1)
	mem_color[0] = cell.ColorRed

	used_ram, err := gauge.New(
		gauge.Height(4),
		gauge.Color(mem_color[0]),
		gauge.BorderTitle("Used RAM"),
	)
	if err != nil {
		panic(err)
	}
	go playGauge(ctx, used_ram, time.Second, &globals.Mem_used_percent)

	available_ram, err := gauge.New(
		gauge.Height(4),
		gauge.Color(mem_color[0]),
		gauge.BorderTitle("Available RAM"),
	)
	if err != nil {
		panic(err)
	}
	go playGauge(ctx, available_ram, time.Second, &globals.Mem_available_percentage)

	cached_ram, err := gauge.New(
		gauge.Height(4),
		gauge.Color(mem_color[0]),
		gauge.BorderTitle("Cached RAM"),
	)
	if err != nil {
		panic(err)
	}
	go playGauge(ctx, cached_ram, time.Second, &globals.Mem_cached_percentage)

	free_ram, err := gauge.New(
		gauge.Height(4),
		gauge.Color(mem_color[0]),
		gauge.BorderTitle("Free RAM"),
	)
	if err != nil {
		panic(err)
	}
	go playGauge(ctx, free_ram, time.Second, &globals.Mem_free_percentage)

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

used_ram_text, err := text.New()
	if err != nil {
		panic(err)
	}
	go writeText(used_ram_text, time.Second, &globals.Mem_used)

available_ram_text, err := text.New()
	if err != nil {
		panic(err)
	}
	go writeText(available_ram_text, time.Second, &globals.Mem_available)

cached_ram_text, err := text.New()
	if err != nil {
		panic(err)
	}
	go writeText(cached_ram_text, time.Second, &globals.Mem_cached)

free_ram_text, err := text.New()
	if err != nil {
		panic(err)
	}
	go writeText(free_ram_text, time.Second, &globals.Mem_free)
const redrawInterval = time.Second
lc, err := linechart.New(
    linechart.AxesCellOpts(cell.FgColor(cell.ColorRed)),
    linechart.YLabelCellOpts(cell.FgColor(cell.ColorAqua)),
    linechart.XLabelCellOpts(cell.FgColor(cell.ColorAqua)),
)
if err != nil{
    panic(err)
}
go playLineChart(ctx, lc, redrawInterval)
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
					container.SplitVertical(
						container.Left(
					//Pie chart goes here
					container.Border(linestyle.Light),
					container.BorderTitle("Total CPU usage"),
					container.PlaceWidget(overallusage),
						),
						container.Right(
							container.SplitHorizontal(
								container.Top(
									container.PlaceWidget(changecolorB),
								),
								container.Bottom(
									container.PlaceWidget(resetcolorB),
								),
							),
						),
					),
				),
			),
		),
		container.Bottom(
			container.SplitVertical(
                container.Left(
                    container.Border(linestyle.Light),
                    container.BorderTitle("CPU Line Graph"),
                    container.PlaceWidget(lc),
                ),container.Right(
					//Memory stuff go here
					container.Border(linestyle.Light),
					container.BorderTitle("RAM Usage information | Total RAM detected : "+strconv.FormatFloat(globals.Mem_total,'g',5,64)+" GiB"),
					container.SplitHorizontal(
						container.Top(
							container.SplitVertical(
								container.Left(
									container.Border(linestyle.Light),
									container.BorderTitle("Total Used RAM : "),
									container.SplitHorizontal(
										container.Top(
											container.PlaceWidget(used_ram),
										),
										container.Bottom(
											container.PlaceWidget(used_ram_text),
										),
									),
								),
								container.Right(
									container.Border(linestyle.Light),
									container.BorderTitle("Total Available RAM : "),
									container.SplitHorizontal(
										container.Top(
									container.PlaceWidget(available_ram),

										),
										container.Bottom(
											container.PlaceWidget(available_ram_text),
										),
									),
								),
							),
						),
						container.Bottom(
							container.SplitVertical(
								container.Left(
									container.Border(linestyle.Light),
									container.BorderTitle("Total Cached RAM : "),
									container.SplitHorizontal(
										container.Top(
									container.PlaceWidget(cached_ram),

										),
										container.Bottom(
											container.PlaceWidget(cached_ram_text),
										),
									),
								),
								container.Right(
									container.Border(linestyle.Light),
									container.BorderTitle("Total Free RAM : "),
									container.SplitHorizontal(
										container.Top(
									container.PlaceWidget(free_ram),
										),
										container.Bottom(
											container.PlaceWidget(free_ram_text),
										),
									),
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
