package globals

import (
	"context"
	"google.golang.org/grpc"
	"sync"
)

var Mainwaitgroup *sync.WaitGroup
var Conn *grpc.ClientConn
var Ctx context.Context
var Cancel context.CancelFunc
var CpuDataBuf []float64
var InitCpuData int32
