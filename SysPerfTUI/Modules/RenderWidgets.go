package modules

import (
	"SysPerfTUI/globals"
	"context"
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
)
func playBarChart(ctx context.Context, bc *barchart.BarChart, delay time.Duration) {
	const max = 100

	ticker := time.NewTicker(delay)
	defer ticker.Stop()
	for {
		select {
		case <-ticker.C:
			var values []int
			for i := 0; i < int(globals.InitCpuData);i++ {
				temp := globals.CpuDataBuf[i]
				if temp > 100 || temp < 0{
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

func RenderWidgets(){
  t, err := tcell.New()
	if err != nil {
		panic(err)
	}
	defer t.Close()

	ctx, cancel := context.WithCancel(context.Background())
	bars := make([]cell.Color, globals.InitCpuData+1)
	bars_text_color := make([]cell.Color, globals.InitCpuData+1)
	bars_text := make([]string, globals.InitCpuData+1)
	temp := "CPU"
	for i:=int32(0);i<=globals.InitCpuData;i++{
	  bars[i] = cell.ColorAqua
	  bars_text_color[i] = cell.ColorPurple
	  bars_text[i] = temp+strconv.Itoa(int(i))
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
	subB, err := button.New("(s)ubtract", func() error {
		return nil
	},
		button.FillColor(cell.ColorNumber(220)),
		button.GlobalKey('s'),
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
			container.Top(container.PlaceWidget(bc)),
			container.Bottom(container.PlaceWidget(subB)),
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
