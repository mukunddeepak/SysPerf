package main

import (
	"SysPerfTUI/Modules"
	"SysPerfTUI/globals"
	"log"
	"sync"

	//https://grpc.io/docs/languages/go/quickstart/ - Resource for grpc basics
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func globalConstructors() {
	globals.Mainwaitgroup = new(sync.WaitGroup)
	conn, err := grpc.Dial("localhost:5001", grpc.WithTransportCredentials(insecure.NewCredentials()))
	globals.Conn = conn
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	globals.CpuGraphBuf = make([]float64, 25)
}

func main() {
	globalConstructors()
	defer globals.Conn.Close()
	go modules.RollingCpuUpdate()
	modules.MainCpuService()
	modules.MainMemService()
	modules.MainBatService()
	modules.RenderWidgets()
}
