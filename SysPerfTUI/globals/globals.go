package globals

import(
	"google.golang.org/grpc"
	"context"
	"sync"
)

var Mainwaitgroup *sync.WaitGroup
var Conn *grpc.ClientConn
var Ctx context.Context
var Cancel context.CancelFunc
var CpuDataBuf []float64
var InitCpuData int32
