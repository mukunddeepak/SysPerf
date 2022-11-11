package modules

import (
	"SysPerfTUI/globals"
	"SysPerfTUI/grpc"
	pb "SysPerfTUI/grpc"
	"context"
	"log"
	"strconv"
	"time"
)

func CoreUsageHandler(core_id int32, c grpc.FetchDataClient) {
	for {
		ctx, cancel := context.WithTimeout(context.Background(), time.Second)
		defer cancel()
		c := pb.NewFetchDataClient(globals.Conn)
		request := pb.CpuUsageRequest{
			NeededCpuUsage: strconv.Itoa(int(core_id)),
		}
		result, err := c.FetchCpuUsage(ctx, &request)
		if err != nil {
			log.Println(err)
		}
		globals.CpuDataBuf[core_id] = float64(result.GetCpuUsage())
		time.Sleep(time.Second)
	}
}

func MainCpuService() {
	defer globals.Mainwaitgroup.Done()
	c := pb.NewFetchDataClient(globals.Conn)
	InitDetailsRequest := pb.EmptyReq{}
	InitDetails, err := c.InitCpuDetail(globals.Ctx, &InitDetailsRequest)
	globals.InitCpuData = InitDetails.GetNumberOfCpu()
	if err != nil {
		log.Println("RPC error")
	}
	globals.CpuDataBuf = make([]float64, InitDetails.GetNumberOfCpu()+1)
	globals.Mainwaitgroup.Add(1)
	for i := int32(0); i <= InitDetails.GetNumberOfCpu(); i++ {
		go CoreUsageHandler(i, c)
	}
	return
}
