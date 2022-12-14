package main

import (
	"os"

	"github.com/RedstoneWizard08/dragon/cmd"
	"github.com/RedstoneWizard08/dragon/internal/common"
)

func main() {
	common.ViewsFS = os.DirFS("views")

	config := common.GetAppConfig()

	cmd.Exit(config)
	cmd.RunCMD(config)
}
