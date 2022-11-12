package modules

import (
	"SysPerfTUI/globals"
	pb "SysPerfTUI/grpc"
	"context"
	"log"
	"time"
)

func MemPoller(){
	for {
		ctx, cancel := context.WithTimeout(context.Background(), time.Second)
		defer cancel()
		c := pb.NewFetchDataMemClient(globals.Conn)
		request := pb.MemUsageRequest{
			PrevMemUsage: 0,
		}
		result, err := c.FetchMemUsage(ctx, &request)
		if err != nil {
			log.Println(err)
		}
		globals.Mem_total = float64(float64(result.Mem_Total)/(1024.0*1024.0))
		globals.Mem_used = float64(float64(result.Mem_Total-result.MemAvailable)/(1024.0*1024.0))
		globals.Mem_available  = float64(float64(result.MemAvailable)/(1024.0*1024.0))
		globals.Mem_cached = float64(float64(result.Mem_Cached)/(1024.0*1024.0))
		globals.Mem_free = float64(float64(result.Mem_Free)/(1024.0*1024.0))
		globals.Mem_used_percent = int32((globals.Mem_used/globals.Mem_total)*100)
		globals.Mem_available_percentage = int32((globals.Mem_available/globals.Mem_total)*100)
		globals.Mem_cached_percentage = int32((globals.Mem_cached/globals.Mem_total)*100)
		globals.Mem_free_percentage = int32((globals.Mem_free/globals.Mem_total)*100)
		time.Sleep(1*time.Second)
	}
}

func MainMemService(){
	go MemPoller()
	time.Sleep(2*time.Second)
}



