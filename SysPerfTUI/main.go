package main

import (
	"SysPerfTUI/globals"
	"SysPerfTUI/Modules"
	"context"
	"log"
	"sync"
	"time"

	//https://grpc.io/docs/languages/go/quickstart/ - Resource for grpc basics
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func globalConstructors(){
	globals.Mainwaitgroup = new(sync.WaitGroup)
  conn, err := grpc.Dial("localhost:5001", grpc.WithTransportCredentials(insecure.NewCredentials()))
  globals.Conn = conn
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	globals.Ctx = ctx
	globals.Cancel = cancel
}

func main(){
	globalConstructors()
	defer globals.Cancel()
	defer globals.Conn.Close()
	modules.MainCpuService()
	modules.RenderWidgets()
}
