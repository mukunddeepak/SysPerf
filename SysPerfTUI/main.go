package main

import (
	pb "SysPerfTUI/grpc"
	"context"
	"log"
	"time"
	"fmt"
	//https://grpc.io/docs/languages/go/quickstart/ - Resource for grpc basics
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)
func main(){
  conn, err := grpc.Dial("localhost:5001", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewFetchDataClient(conn)
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	request := pb.CpuUsageRequest{
	  NeededCpuUsage: "0",
	}
	result, err := c.FetchCpuUsage(ctx,&request)
	if err!=nil{
	  log.Println("RPC error")
	}
	fmt.Println(result.GetCpuId(), result.GetCpuUsage())
}
