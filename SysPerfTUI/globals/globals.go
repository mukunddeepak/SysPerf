package globals

import (
	"sync"

	"google.golang.org/grpc"
)

var Mainwaitgroup *sync.WaitGroup
var Conn *grpc.ClientConn
var CpuDataBuf []float64
var CpuGraphBuf []float64
var InitCpuData int32

var Mem_total float64

var Mem_used float64
var Mem_used_percent int32

var Mem_available float64
var Mem_available_percentage int32

var Mem_cached float64
var Mem_cached_percentage int32

var Mem_free float64
var Mem_free_percentage int32

var Charge_full_design int32
var Charge_full int32
var Charge_now int32
