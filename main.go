package main

import (
	"fmt"
	"net"
	"sync"
	"time"
)

func measureTime(fn func()) {
	start := time.Now()
	fn()
	elapsed := time.Since(start) / 1e9
	fmt.Printf("Execution Time: %d Secs.\n", elapsed)
}

func tcpScan() {
	var wg sync.WaitGroup
	for i := 1; i <= 65535; i++ {
		wg.Add(1)
		go func(p int) {
			defer wg.Done()
			addr := fmt.Sprintf("20.194.168.24:%d", p)
			conn, err := net.Dial("tcp", addr)
			if err != nil {
				fmt.Printf("%s 关闭了\n", addr)
				return
			}
			_ = conn.Close()
			fmt.Printf("%s 打开了！！！\n", addr)
		}(i)
	}
	wg.Wait()
}

func main() {
	measureTime(tcpScan)
}
