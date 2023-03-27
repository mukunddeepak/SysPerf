package modules

import (
	"SysPerfTUI/globals"
	pb "SysPerfTUI/grpc"
	"context"
	"log"
	"math"
	"time"
)

func roundFloat(val float64, precision uint) float64 {
	ratio := math.Pow(10, float64(precision))
	return math.Round(val*ratio) / ratio
}

func BatPoller() {
	for {
		ctx, cancel := context.WithTimeout(context.Background(), time.Second)
		defer cancel()
		c := pb.NewFetchDataBatClient(globals.Conn)
		request := pb.BatUsageRequest{
			PrevBatUsage: 0,
		}
		result, err := c.FetchBatUsage(ctx, &request)
		if err != nil {
			log.Println(err)
		}
		globals.Charge_full_design = int32(result.Charge_Full_Design)
		globals.Charge_full = int32(result.Charge_Full)
		globals.Charge_now = int32(result.Charge_Now)
		globals.Bat_level = roundFloat((float64(result.Charge_Now)/float64(result.Charge_Full))*100, 2)
		globals.Bat_health = roundFloat((float64(result.Charge_Full)/float64(result.Charge_Full_Design))*100, 2)
		// log.Println(globals.Charge_now, globals.Bat_health)
		time.Sleep(1 * time.Second)
	}
}

func MainBatService() {
	go BatPoller()
	time.Sleep(2 * time.Second)
}
