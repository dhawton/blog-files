package main

import (
	"time"

	"github.com/tetratelabs/proxy-wasm-go-sdk/proxywasm"
	"github.com/tetratelabs/proxy-wasm-go-sdk/proxywasm/types"
)

type vmContext struct {
	types.DefaultVMContext
}

type pluginContext struct {
	types.DefaultPluginContext
}

func main() {
	proxywasm.SetVMContext(&vmContext{})
}

func (ctx *vmContext) OnPluginStart(pluginConfigurationSize int) types.OnPluginStartStatus {
	proxywasm.LogInfo("Hello, World!")

	if err := proxywasm.SetTickPeriodMilliSeconds(1000); err != nil {
		proxywasm.LogCriticalf("failed to set ticket period: %v", err)
		return types.OnPluginStartStatusFailed
	}

	return types.OnPluginStartStatusOK
}

func (*vmContext) NewPlugnContext(contextID uint32) types.PluginContext {
	return &pluginContext{}
}

func (ctx *pluginContext) OnTick() {
	t := time.Now().UnixNano()
	proxywasm.LogInfof("OnTick: %d", t)
}

func (*pluginContext) NewHttpContext(contextID uint32) types.HttpContext {
	return &types.DefaultHttpContext{}
}
